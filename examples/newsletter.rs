// examples/newsletter.rs
//
// Colourful newsletter / magazine style:
//   • Vibrant custom CSS with coloured sections
//   • No heading numbers — clean editorial look
//   • Header + footer
//   • 2+ pages
//
// Run:  cargo run --example newsletter

use pdfsmith::{FooterConfig, HeaderConfig, PaperSize, PdfBuilder};

fn main() {
    env_logger::init();

    let css = r#"
        * { box-sizing: border-box; }
        body {
            font-family: "Segoe UI", Helvetica, Arial, sans-serif;
            font-size: 10.5pt;
            line-height: 1.6;
            color: #333;
            margin: 0; padding: 0;
        }

        /* ── Headings: colourful, no numbering ── */
        h1 {
            font-size: 28pt;
            color: #fff;
            background: linear-gradient(135deg, #6366f1, #8b5cf6);
            padding: 16px 24px;
            border-radius: 8px;
            margin: 0 0 0.6em 0;
        }
        h2 {
            font-size: 18pt;
            color: #6366f1;
            border-left: 5px solid #6366f1;
            padding-left: 12px;
            margin-top: 1.5em;
        }
        h3 {
            font-size: 13pt;
            color: #059669;
            margin-top: 1.2em;
        }
        h4 {
            font-size: 11pt;
            color: #d97706;
        }

        /* ── Paragraphs ── */
        p { margin: 0 0 1em 0; }

        /* ── Links ── */
        a { color: #6366f1; text-decoration: underline; }

        /* ── Code ── */
        code {
            font-family: "Fira Code", "Cascadia Code", Consolas, monospace;
            font-size: 90%;
            background: #f3f4f6;
            padding: 2px 6px;
            border-radius: 3px;
        }
        pre {
            font-family: "Fira Code", Consolas, monospace;
            font-size: 9pt;
            background: #1e1b4b;
            color: #e0e7ff;
            padding: 16px;
            border-radius: 8px;
            overflow-x: auto;
        }
        pre code { background: transparent; color: inherit; padding: 0; }

        /* ── Blockquote: accent card ── */
        blockquote {
            margin: 1em 0;
            padding: 16px 20px;
            background: #eef2ff;
            border-left: 5px solid #6366f1;
            border-radius: 0 8px 8px 0;
            color: #4338ca;
            font-size: 11pt;
        }

        /* ── Tables ── */
        table { border-collapse: collapse; width: 100%; margin-bottom: 1em; }
        th, td { padding: 10px 14px; border: 1px solid #e5e7eb; }
        th { background: #6366f1; color: white; font-weight: 600;
             text-transform: uppercase; font-size: 9pt; letter-spacing: 0.5px; }
        tr:nth-child(even) { background: #f9fafb; }

        /* ── Lists ── */
        ul, ol { padding-left: 1.8em; }
        li { margin-top: 0.3em; }

        /* ── Horizontal rule ── */
        hr { border: 0; height: 3px; background: linear-gradient(90deg, #6366f1, #8b5cf6, #a78bfa); margin: 2em 0; border-radius: 2px; }

        /* ── Images ── */
        img { max-width: 100%; border-radius: 8px; }
    "#;

    let markdown = r##"
# The Developer Digest — April 2026

Welcome to the April edition of **The Developer Digest**, your monthly
roundup of what's new, what's trending, and what's worth learning in the
world of software engineering.

## Editor's Note

Spring is here, and so is a wave of exciting releases.  Rust 1.85 shipped
with async closures stabilised, Node.js 24 LTS is out, and the AI coding
assistant space is evolving faster than ever.  Let's dive in.

## Language & Framework Updates

### Rust 1.85

The headline feature is **async closures** (`async || { ... }`).  This
eliminates the need for `async move` workarounds in many callback-heavy
APIs:

```rust
let handler = async |req: Request| {
    let data = fetch_data(&req).await?;
    Ok(Response::json(&data))
};
```

Other highlights:

- `impl Trait` in trait methods (stabilised)
- New `std::sync::LazyLock` for one-time initialisation
- Compiler error messages now show *suggested fixes* inline
- Cargo workspace improvements for large monorepos

### Node.js 24 LTS

| Feature               | Status      |
|-----------------------|-------------|
| V8 engine 13.0        | ✓ Included  |
| Built-in test runner   | ✓ Stable    |
| `fetch()` global      | ✓ Stable    |
| Permission model       | ✓ Stable    |
| Single-executable apps | ✓ Stable    |
| TypeScript loader      | Experimental|

The built-in test runner is now production-ready:

```javascript
import { test, describe, it } from 'node:test';
import assert from 'node:assert';

describe('Calculator', () => {
    it('should add two numbers', () => {
        assert.strictEqual(1 + 1, 2);
    });

    it('should handle floating point', () => {
        assert.strictEqual(0.1 + 0.2, 0.30000000000000004);
    });
});
```

### Go 1.24

Go 1.24 brings **generic type aliases**, improved HTTP routing, and a new
`log/slog` structured logging package:

```go
type Result[T any] = struct {
    Value T
    Error error
}

func Fetch[T any](url string) Result[T] {
    // ...
}
```

## Trending This Month

### AI Coding Assistants: A Comparison

| Tool            | Model        | IDE Support     | Price     |
|-----------------|-------------|-----------------|-----------|
| GitHub Copilot  | GPT-4o / Custom | VS Code, JetBrains, Vim | $10/mo |
| Cursor          | Multiple    | Cursor (fork of VS Code)| $20/mo  |
| Cody            | Claude / StarCoder | VS Code, JetBrains | Free / $9/mo |
| Amazon CodeWhisperer | Custom | VS Code, JetBrains | Free / $19/mo |
| Tabnine         | Custom      | All major IDEs  | Free / $12/mo |

> The real question isn't whether to use an AI assistant — it's how to use
> one effectively without losing your understanding of the codebase.

### WebAssembly Component Model

The Component Model is finally reaching v1.0.  This means:

1. **Language-agnostic components** — write in Rust, call from JS
2. **Composability** — snap components together like LEGO
3. **Sandboxing** — each component runs in its own memory space
4. **Interface Types** — automatic marshalling of complex data structures

## Deep Dive: Building a PDF Library in Rust

This newsletter was generated using `pdfsmith`, a Rust library that converts
Markdown and structured JSON into PDFs via headless Chrome.

### Architecture

The pipeline is straightforward:

1. **Parse** — Markdown or JSON → HTML body (via comrak)
2. **Style** — Wrap in full HTML document with CSS
3. **Render** — Headless Chrome prints the page to PDF
4. **Return** — Raw bytes ready to write or stream

### Why Chrome?

Alternatives like `wkhtmltopdf` and `weasyprint` exist, but Chrome has
advantages:

| Feature         | Chrome | wkhtmltopdf | WeasyPrint |
|-----------------|--------|-------------|------------|
| CSS Grid        | ✓      | ✗           | Partial    |
| Flexbox         | ✓      | Partial     | ✓          |
| @media print    | ✓      | ✓           | ✓          |
| JavaScript      | ✓      | Partial     | ✗          |
| Emoji rendering | ✓      | ✗           | ✓          |
| Active maint.   | ✓      | ✗ (archived)| ✓          |

### Example: This Newsletter

```rust
let pdf = PdfBuilder::new()
    .title("The Developer Digest")
    .custom_css(newsletter_css)
    .display_header_footer(true)
    .header(HeaderConfig { ... })
    .footer(FooterConfig { ... })
    .from_markdown(markdown)
    .unwrap();
```

## Community Picks

### Open Source Projects Worth Watching

- **Zed** — A GPU-accelerated code editor written in Rust
- **Deno 2.0** — Node-compatible runtime, now with `npm:` specifiers
- **Turso** — LibSQL-based edge database, SQLite for the cloud
- **Biome** — Fast formatter + linter (Rome successor)
- **Ruff** — Python linter in Rust, 100x faster than flake8

### Conferences Coming Up

| Event              | Date           | Location     |
|--------------------|---------------|--------------|
| RustConf 2026      | June 12–14    | Montreal     |
| JSConf EU          | June 20–21    | Berlin       |
| GopherCon          | July 8–11     | Chicago      |
| KubeCon NA         | Oct 14–17     | Salt Lake City|
| Strange Loop       | Sept 18–19    | St. Louis    |

## Quick Tips

### Git: Undo the Last Commit (Keep Changes)

```bash
git reset --soft HEAD~1
```

### Rust: Measure Function Duration

```rust
let start = std::time::Instant::now();
do_work();
println!("Took: {:?}", start.elapsed());
```

### CSS: Truncate Text with Ellipsis

```css
.truncate {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 200px;
}
```

---

> Stay curious.  Ship code.  Read the docs.

*See you next month!  — The Developer Digest Team*
"##;

    let pdf = PdfBuilder::new()
        .title("Developer Digest — April 2026")
        .paper_size(PaperSize::A4)
        .custom_css(css)
        // No heading_numbers — editorial style
        .display_header_footer(true)
        .header(HeaderConfig {
            center: Some("The Developer Digest — April 2026".into()),
            font_size: Some("8px".into()),
            color: Some("#6366f1".into()),
            ..Default::default()
        })
        .footer(FooterConfig {
            left: Some("© 2026 DevDigest".into()),
            right: Some(
                r#"<span class="pageNumber"></span> / <span class="totalPages"></span>"#.into(),
            ),
            font_size: Some("8px".into()),
            color: Some("#9ca3af".into()),
            ..Default::default()
        })
        .from_markdown(markdown)
        .expect("Failed to generate PDF");

    std::fs::write("examples/output_pdfs/newsletter.pdf", &pdf).expect("Failed to write PDF");
    println!("PDF saved to examples/output_pdfs/newsletter.pdf ({} bytes)", pdf.len());
}
