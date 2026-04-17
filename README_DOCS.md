# pdfsmith

A fully customizable Rust library for generating PDFs from **Markdown**, **structured JSON**, or **raw HTML**.

Everything is under the user's control — CSS styling, page size, margins, orientation, headers, footers, Markdown extensions, and Chrome rendering options. Supply your own CSS or use the clean built-in default.

- 🚀 Built with Rust
- 📄 Outputs polished PDF documents
- ✨ Supports Markdown, JSON, and HTML input

---

## Features

| Feature | Description |
|---|---|
| **Markdown → PDF** | Convert any Markdown string or file to a styled PDF |
| **JSON → PDF** | Define document content with typed JSON blocks (headings, paragraphs, tables, code, lists, etc.) |
| **HTML → PDF** | Pass your own pre-built HTML directly |
| **Custom CSS** | Replace the default stylesheet entirely, or append extra rules |
| **Paper Sizes** | A4, Letter, Legal, or any custom width × height |
| **Page Margins** | Set top, bottom, left, right margins independently (in inches) |
| **Orientation** | Portrait or landscape |
| **Headers** | Simple left/center/right text, or full custom HTML |
| **Footers** | Simple left/center/right text with auto page numbers, or full custom HTML |
| **Heading Numbers** | Automatic hierarchical numbering (1, 1.1, 1.1.1) via JSON `paraSequence` or CSS counters |
| **Markdown Extensions** | Tables, footnotes, strikethrough, task lists, autolinks, superscript, description lists — all toggleable |
| **File Input** | Read `.md` or `.json` files directly from disk |
| **Builder Pattern** | Fluent, chainable API — configure only what you need |

---

## Quick Start

```rust
use pdfsmith::PdfBuilder;

fn main() {
    let pdf_bytes = PdfBuilder::new()
        .from_markdown("# Hello World\n\nThis is my first PDF.")
        .expect("PDF generation failed");

    std::fs::write("hello.pdf", &pdf_bytes).expect("Failed to write file");
    println!("PDF saved! ({} bytes)", pdf_bytes.len());
}
```

That's it. Three lines of code, zero configuration. The default CSS, A4 paper, 0.75-inch margins, and no header/footer are applied automatically.

---

## Input Sources

### From Markdown String

```rust
use pdfsmith::PdfBuilder;

let pdf = PdfBuilder::new()
    .from_markdown("# Title\n\nSome **bold** and *italic* text.")
    .unwrap();
```

All standard Markdown is supported: headings, bold, italic, links, images, code blocks, tables, task lists, blockquotes, horizontal rules, footnotes, strikethrough, and more.

### From Markdown File

```rust
use pdfsmith::PdfBuilder;

let pdf = PdfBuilder::new()
    .from_markdown_file("docs/report.md")
    .unwrap();
```

### From JSON

```rust
use pdfsmith::PdfBuilder;

let json = serde_json::json!([
    { "type": "heading", "paraSequence": "1", "text": "Report Title" },
    { "type": "paragraph", "text": "Some content with **Markdown** formatting." },
    { "type": "heading", "paraSequence": "1.1", "text": "Details" },
    { "type": "paragraph", "text": "The heading level is derived from paraSequence depth." }
]);

let pdf = PdfBuilder::new()
    .from_json(&json)
    .unwrap();
```

JSON is first converted to Markdown, then rendered through the same pipeline — so all CSS and config applies equally.

### From JSON File

```rust
use pdfsmith::PdfBuilder;

let pdf = PdfBuilder::new()
    .from_json_file("data/document.json")
    .unwrap();
```

### From Raw HTML

```rust
use pdfsmith::PdfBuilder;

let html = r#"
<!DOCTYPE html>
<html>
<head><style>body { font-family: Arial; }</style></head>
<body><h1>Hello</h1><p>From raw HTML.</p></body>
</html>
"#;

let pdf = PdfBuilder::new()
    .from_html(html)
    .unwrap();
```

When using `from_html`, the HTML is used **as-is** — no Markdown conversion, no CSS injection. Header/footer and Chrome options still apply.

---

## JSON Structure Reference

The JSON input is an array of **content blocks**. Each block has a `"type"` field that determines how it's rendered.

Two top-level formats are accepted:

```jsonc
// Format A: bare array
[
  { "type": "heading", "paraSequence": "1", "text": "Title" },
  { "type": "paragraph", "text": "Body." },
  { "type": "heading", "paraSequence": "1.1", "text": "Sub-section" },
  { "type": "paragraph", "text": "More body." }
]

// Format B: object with "content" key
{
  "content": [
    { "type": "heading", "paraSequence": "1", "text": "Title" },
    { "type": "paragraph", "text": "Body." }
  ]
}
```

### Block Types

#### `heading`

```json
{ "type": "heading", "paraSequence": "1.2", "text": "Section Title" }
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `paraSequence` | string | No | Hierarchical section number (e.g. `"1"`, `"1.2"`, `"1.2.1"`, `"A.1"`). The heading level (h1–h6) is **derived from the depth** (number of dot-separated parts). The sequence is prefixed to the heading text automatically. |
| `text` | string | Yes | Heading text (Markdown allowed) |
| `level` | number (1–6) | No | Explicit heading level — overrides the level derived from `paraSequence`. If neither `paraSequence` nor `level` is given, defaults to h1. |

#### `paragraph` (alias: `text`)

```json
{ "type": "paragraph", "text": "Some **bold** text with [links](https://example.com)." }
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `text` | string | Yes | Paragraph content (Markdown allowed) |

#### `code`

```json
{ "type": "code", "language": "rust", "text": "fn main() {\n    println!(\"Hello\");\n}" }
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `language` | string | No (default: `""`) | Syntax highlighting hint |
| `text` | string | Yes | Code content |

#### `list`

```json
{ "type": "list", "ordered": true, "items": ["First", "Second", "Third"] }
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ordered` | boolean | No (default: `false`) | `true` for numbered list, `false` for bullets |
| `items` | array of strings | Yes | List items (Markdown allowed in each item) |

#### `quote` (alias: `blockquote`)

```json
{ "type": "quote", "text": "To be or not to be." }
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `text` | string | Yes | Quote content (Markdown allowed) |

#### `table`

```json
{
  "type": "table",
  "headers": ["Name", "Role", "Location"],
  "rows": [
    ["Alice", "Engineer", "London"],
    ["Bob", "Designer", "Berlin"]
  ]
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `headers` | array of strings | Yes | Column headers |
| `rows` | array of arrays | No | Data rows (each row is an array of strings) |

#### `image` (alias: `img`)

```json
{ "type": "image", "src": "https://example.com/photo.png", "alt": "A photo" }
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `src` | string | Yes | Image URL or path |
| `alt` | string | No (default: `""`) | Alt text |

#### `divider` (alias: `hr`)

```json
{ "type": "divider" }
```

Renders a horizontal rule (`---`). No fields needed.

#### `html` (alias: `raw`)

```json
{ "type": "html", "text": "<div style='color:red;'>Custom HTML</div>" }
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `text` | string | Yes | Raw HTML passed through as-is |

---

## Examples

The repository includes **eight** runnable examples in the `examples/` directory, each producing a 2+ page PDF.

### Example 1 — Minimal Markdown

Markdown string → PDF with `heading_numbers(true)`. Default CSS, no header/footer. Covers tables, code blocks, lists, blockquotes.

```bash
cargo run --example from_markdown
```

### Example 2 — JSON with paraSequence

Structured JSON content blocks with `"paraSequence"` on every heading for hierarchical numbering. The heading level (h1–h6) is derived automatically from the sequence depth.

```rust
// examples/from_json.rs  (abbreviated)
let json = serde_json::json!([
    { "type": "heading", "paraSequence": "1",     "text": "Project Status Report" },
    { "type": "paragraph", "text": "This report summarises the current state." },
    { "type": "heading", "paraSequence": "1.1",   "text": "Timeline" },
    { "type": "table", "headers": ["Phase","Status","Due"], "rows": [["Design","Complete","2026-01-15"]] },
    { "type": "heading", "paraSequence": "1.2",   "text": "Key Highlights" },
    { "type": "heading", "paraSequence": "1.2.1", "text": "Performance Details" },
    { "type": "heading", "paraSequence": "2",     "text": "Architecture" },
    // ... more blocks ...
]);

let pdf = PdfBuilder::new()
    .title("Project Report")
    .from_json(&json)
    .expect("Failed to generate PDF");
```

Output headings:

```text
1 Project Status Report       (depth 1 → h1)
  1.1 Timeline                (depth 2 → h2)
  1.2 Key Highlights          (depth 2 → h2)
    1.2.1 Performance Details  (depth 3 → h3)
2 Architecture                (depth 1 → h1)
```

```bash
cargo run --example from_json
```

### Example 3 — Custom CSS

Replaces the entire default stylesheet. Indented sub-headings (h2 → 20px, h3 → 40px, h4 → 60px), per-level colours (blue → green → gold), dark code blocks, `heading_numbers(true)` on top.

```bash
cargo run --example custom_css
```

### Example 4 — Full Document

Every option configured: title, Letter landscape paper, header, footer, `heading_numbers(true)`, extra CSS for colours. Quarterly business review theme.

```bash
cargo run --example full_document
```

### Example 5 — Corporate Report

A4 report with default CSS as the base, small tweaks via `extra_css` (indented headings, corporate colours). Header shows company name + report title, footer shows "INTERNAL" + page numbers.

```bash
cargo run --example report_style
```

### Example 6 — Images

Markdown document with remote images (Wikimedia Commons). Uses `page_load_wait_secs(5)` to allow time for image loading. Header/footer, heading numbers, extra CSS for image styling.

```bash
cargo run --example with_images
```

### Example 7 — Newsletter

Vibrant editorial style with full `custom_css`: gradient heading backgrounds, purple/green/gold colours, styled blockquotes. No heading numbers — clean magazine look. Header + footer.

```bash
cargo run --example newsletter
```

### Example 8 — Technical Doc (JSON)

Complete API reference built entirely from JSON with `paraSequence` on every heading. Uses a `build_content()` helper to construct large JSON arrays. Header/footer, indented headings via `extra_css`.

```bash
cargo run --example technical_doc
```

---

## Running the Examples

```bash
# Clone the repository
git clone github.com/DhvaniBhesaniya/pdfsmith.git
cd pdfsmith

# Run any example
cargo run --example from_markdown
cargo run --example from_json
cargo run --example custom_css
cargo run --example full_document
cargo run --example report_style
cargo run --example with_images
cargo run --example newsletter
cargo run --example technical_doc



# With logging enabled
RUST_LOG=info cargo run --example from_markdown
```

---

## See working examples

You can browse the full example source files on GitHub:

- [custom_css.rs](https://github.com/DhvaniBhesaniya/pdfsmith/tree/main/examples/custom_css.rs)
- [from_json.rs](https://github.com/DhvaniBhesaniya/pdfsmith/tree/main/examples/from_json.rs)
- [from_markdown.rs](https://github.com/DhvaniBhesaniya/pdfsmith/tree/main/examples/from_markdown.rs)
- [full_document.rs](https://github.com/DhvaniBhesaniya/pdfsmith/tree/main/examples/full_document.rs)
- [newsletter.rs](https://github.com/DhvaniBhesaniya/pdfsmith/tree/main/examples/newsletter.rs)
- [report_style.rs](https://github.com/DhvaniBhesaniya/pdfsmith/tree/main/examples/report_style.rs)
- [technical_doc.rs](https://github.com/DhvaniBhesaniya/pdfsmith/tree/main/examples/technical_doc.rs)
- [with_images.rs](https://github.com/DhvaniBhesaniya/pdfsmith/tree/main/examples/with_images.rs)

---

## Error Handling

All generation methods return `pdfsmith::Result<Vec<u8>>`, which is `Result<Vec<u8>, MdocError>`.

```rust
use pdfsmith::{PdfBuilder, MdocError};

match PdfBuilder::new().from_markdown("# Test") {
    Ok(pdf) => std::fs::write("out.pdf", pdf).unwrap(),
    Err(MdocError::Chrome(msg)) => eprintln!("Chrome error: {}", msg),
    Err(MdocError::Io(err))     => eprintln!("IO error: {}", err),
    Err(MdocError::Json(msg))   => eprintln!("JSON error: {}", msg),
    Err(e)                      => eprintln!("Error: {}", e),
}
```

### Error Variants

| Variant | When it occurs |
|---|---|
| `MdocError::Chrome(String)` | Chrome/Chromium not found, failed to launch, navigation timeout, print failure |
| `MdocError::Io(std::io::Error)` | File read/write errors |
| `MdocError::Json(String)` | Invalid JSON structure (not an array or missing `"content"` key) |
| `MdocError::ImageDownload { url, reason }` | Remote image fetch failed |
| `MdocError::Other(String)` | Catch-all for uncategorised errors |
