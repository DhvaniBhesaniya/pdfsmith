// examples/custom_css.rs
//
// Replaces the entire default CSS with a custom stylesheet:
//   • Sub-headings are indented under their parent heading
//   • Different colours per heading level
//   • Serif body font, dark code blocks
//   • heading_numbers(true) works on top of custom CSS
//   • No header/footer — pure content styling showcase
//   • 2+ pages
//
// Run:  cargo run --example custom_css

use pdfsmith::PdfBuilder;

fn main() {
    env_logger::init();

    // ── The entire stylesheet — replaces the built-in default ────────
    let my_css = r#"
        /* ── Base ── */
        * { box-sizing: border-box; }
        body {
            font-family: Georgia, "Times New Roman", serif;
            font-size: 11pt;
            line-height: 1.7;
            color: #2c3e50;
            margin: 0;
            padding: 0;
        }

        /* ── Headings: different colour + increasing left indent ── */
        h1, h2, h3, h4, h5, h6 {
            font-weight: 700;
            margin-top: 1.4em;
            margin-bottom: 0.4em;
            line-height: 1.25;
        }
        h1 {
            font-size: 22pt;
            color: #1a5276;
            border-bottom: 3px solid #1a5276;
            padding-bottom: 6px;
            margin-left: 0;
        }
        h2 {
            font-size: 17pt;
            color: #2874a6;
            border-bottom: 1px solid #d4e6f1;
            padding-bottom: 4px;
            margin-left: 20px;         /* ← indented under h1 */
        }
        h3 {
            font-size: 13pt;
            color: #1e8449;
            margin-left: 40px;         /* ← indented under h2 */
        }
        h4 {
            font-size: 11pt;
            color: #b9770e;
            margin-left: 60px;         /* ← indented under h3 */
        }

        /* paragraphs / lists inherit the indent of their preceding heading */
        p, ul, ol, dl, table, pre, blockquote, details {
            margin-top: 0;
            margin-bottom: 1em;
        }

        /* ── Links ── */
        a { color: #2874a6; }

        /* ── Inline code ── */
        code, tt {
            font-family: "Courier New", Consolas, monospace;
            font-size: 90%;
            background: #eaf2f8;
            padding: 2px 5px;
            border-radius: 3px;
        }

        /* ── Code blocks: dark background ── */
        pre {
            font-family: "Courier New", Consolas, monospace;
            font-size: 9pt;
            background: #1c2833;
            color: #d5dbdb;
            padding: 16px;
            border-radius: 6px;
            overflow-x: auto;
            line-height: 1.5;
        }
        pre code {
            background: transparent;
            color: inherit;
            padding: 0;
        }

        /* ── Blockquote ── */
        blockquote {
            margin: 0.5em 0 1em 0;
            padding: 0.5em 1em;
            color: #5d6d7e;
            border-left: 4px solid #e74c3c;
            background: #fdedec;
            font-style: italic;
        }

        /* ── Tables ── */
        table {
            border-collapse: collapse;
            width: 100%;
            margin-bottom: 1em;
        }
        th, td { padding: 8px 12px; border: 1px solid #d5dbdb; }
        th { background: #2874a6; color: #fff; font-weight: 600; }
        tr:nth-child(even) { background: #eaf2f8; }

        /* ── Horizontal rule ── */
        hr {
            height: 2px;
            background: #aab7b8;
            border: 0;
            margin: 1.5em 0;
        }

        /* ── Lists ── */
        ul, ol { padding-left: 2em; }
        li { margin-top: 0.3em; }

        /* ── Images ── */
        img { max-width: 100%; border-radius: 4px; }

        /* ── Task list ── */
        .task-list-item          { list-style-type: none; }
        .task-list-item-checkbox { margin: 0 0.2em 0.25em -1.4em; vertical-align: middle; }

        /* ── Footnotes ── */
        .footnotes { font-size: 85%; color: #5d6d7e; }
    "#;

    let markdown = r##"
# Design System Documentation

This document showcases **fully custom CSS**.  Notice how each heading level
has its own colour and is indented further than its parent — creating a
clear visual hierarchy.

## Typography

The body uses **Georgia**, a classic serif typeface.  Code uses Courier New
in dark-background blocks.  Headings step through a blue → green → gold
palette.

### Font Stack

| Role     | Font Family                               | Size  |
|----------|-------------------------------------------|-------|
| Body     | Georgia, "Times New Roman", serif         | 11pt  |
| Headings | Georgia (bold)                            | 22–11pt |
| Code     | "Courier New", Consolas, monospace        | 9pt   |
| UI       | -apple-system, Helvetica, Arial           | —     |

### Colour Palette

| Level | Colour  | Hex       | Usage            |
|-------|---------|-----------|------------------|
| H1    | Dark blue | `#1a5276` | Top-level sections |
| H2    | Blue    | `#2874a6`   | Sub-sections     |
| H3    | Green   | `#1e8449`   | Details          |
| H4    | Gold    | `#b9770e`   | Deep details     |
| Body  | Charcoal | `#2c3e50`  | Paragraph text   |
| Quote | Red border| `#e74c3c` | Emphasis         |

## Component Library

### Buttons

The system provides four button variants:

1. **Primary** — `#2874a6` background, white text, 8px padding
2. **Secondary** — outlined, `#2874a6` border, transparent background
3. **Danger** — `#e74c3c` background, used for destructive actions
4. **Ghost** — no border, no background, text only

### Form Fields

All form fields follow these rules:

- 12px vertical padding, 16px horizontal
- 1px solid `#d5dbdb` border, 4px radius
- Focus state: `#2874a6` border, subtle box-shadow
- Error state: `#e74c3c` border, red helper text below

#### Input Validation Rules

Fields are validated on blur and on submit.  Custom validators can be
registered per field:

```typescript
interface Validator {
  name: string;
  test: (value: string) => boolean;
  message: string;
}

const emailValidator: Validator = {
  name: "email",
  test: (v) => /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(v),
  message: "Please enter a valid email address.",
};
```

### Cards

Cards are the primary content container:

- White background, 8px border-radius
- 1px `#d5dbdb` border, subtle `box-shadow`
- 16px internal padding
- Optional header bar with coloured left border

## Spacing System

The design system uses a **4px base grid**:

| Token  | Value | Usage                  |
|--------|-------|------------------------|
| `xs`   | 4px   | Tight inline spacing   |
| `sm`   | 8px   | Between related items  |
| `md`   | 16px  | Standard padding       |
| `lg`   | 24px  | Section gaps           |
| `xl`   | 32px  | Page-level margins     |
| `xxl`  | 48px  | Hero sections          |

## Code Examples

### Rust

```rust
#[derive(Debug, Clone)]
pub struct Theme {
    pub primary: Color,
    pub secondary: Color,
    pub background: Color,
    pub text: Color,
    pub font_family: String,
    pub base_size: f64,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            primary: Color::from_hex("#2874a6"),
            secondary: Color::from_hex("#1e8449"),
            background: Color::WHITE,
            text: Color::from_hex("#2c3e50"),
            font_family: "Georgia, serif".into(),
            base_size: 11.0,
        }
    }
}
```

### CSS

```css
.card {
    background: white;
    border: 1px solid #d5dbdb;
    border-radius: 8px;
    padding: 16px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.06);
    transition: box-shadow 0.2s ease;
}
.card:hover {
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.12);
}
```

> Good design is as little design as possible.  — Dieter Rams

## Summary

This example demonstrates:

- **Indented headings** — each level shifts right, creating a clear hierarchy
- **Level-specific colours** — blue → green → gold progression
- **Dark code blocks** — high contrast, easy to read
- **Coloured blockquotes** — red left border with pink background
- **Styled tables** — blue headers, alternating row colours
- All of this with `heading_numbers(true)` for automatic numbering

---

*End of Design System Documentation*
"##;

    let pdf = PdfBuilder::new()
        .title("Design System")
        .custom_css(my_css)
        .heading_numbers(true)
        .from_markdown(markdown)
        .expect("Failed to generate PDF");

    std::fs::write("examples/output_pdfs/custom_css.pdf", &pdf).expect("Failed to write PDF");
    println!("PDF saved to examples/output_pdfs/custom_css.pdf ({} bytes)", pdf.len());
}
