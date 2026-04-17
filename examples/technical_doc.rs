// examples/technical_doc.rs
//
// Technical documentation generated entirely from JSON with paraSequence.
// Uses header + footer, heading numbers (baked into JSON via paraSequence),
// lots of code blocks, tables, and technical content.  2+ pages.
//
// Run:  cargo run --example technical_doc

use pdfsmith::{FooterConfig, HeaderConfig, PageMargins, PdfBuilder};
use serde_json::json;

/// Helper: build the content array in small chunks to avoid serde_json
/// macro recursion limits on large arrays.
fn build_content() -> serde_json::Value {
    let mut blocks = Vec::new();

    // ─── Section 1 ──────────────────────────────────────────────
    blocks.extend(vec![
        json!({ "type": "heading", "paraSequence": "1", "text": "pdfsmith API Reference" }),
        json!({ "type": "paragraph", "text": "Complete API documentation for the `pdfsmith` crate v0.1.0.  This document covers every public type, method, and configuration option." }),
    ]);

    // ─── 1.1 ────────────────────────────────────────────────────
    blocks.extend(vec![
        json!({ "type": "heading", "paraSequence": "1.1", "text": "Installation" }),
        json!({ "type": "paragraph", "text": "Add the crate to your `Cargo.toml`:" }),
        json!({ "type": "code", "language": "toml", "text": "[dependencies]\npdfsmith = \"0.1.0\"" }),
        json!({ "type": "paragraph", "text": "**Prerequisites:** Chromium or Google Chrome must be installed on the system.  On Ubuntu/Debian:" }),
        json!({ "type": "code", "language": "bash", "text": "sudo apt update && sudo apt install -y chromium-browser" }),
    ]);

    // ─── 1.2 ────────────────────────────────────────────────────
    blocks.extend(vec![
        json!({ "type": "heading", "paraSequence": "1.2", "text": "Quick Start" }),
        json!({ "type": "code", "language": "rust", "text": "use pdfsmith::PdfBuilder;\n\nfn main() {\n    let pdf = PdfBuilder::new()\n        .title(\"My Document\")\n        .heading_numbers(true)\n        .from_markdown(\"# Hello\\n\\n## World\")\n        .expect(\"Failed\");\n\n    std::fs::write(\"output.pdf\", &pdf).unwrap();\n}" }),
    ]);

    // ─── Section 2 ──────────────────────────────────────────────
    blocks.extend(vec![
        json!({ "type": "heading", "paraSequence": "2", "text": "PdfBuilder" }),
        json!({ "type": "paragraph", "text": "The main entry point.  All methods are chainable.  Call one of the generation methods (`from_markdown`, `from_json`, `from_html`) last." }),
    ]);

    // ─── 2.1 ────────────────────────────────────────────────────
    blocks.extend(vec![
        json!({ "type": "heading", "paraSequence": "2.1", "text": "Constructor Methods" }),
        json!({ "type": "table",
          "headers": ["Method", "Description"],
          "rows": [
              ["`PdfBuilder::new()`", "Create with default config (A4, 0.75\" margins, no header/footer)"],
              ["`PdfBuilder::with_config(cfg)`", "Create from an existing `PdfConfig` struct"]
          ]
        }),
    ]);

    // ─── 2.2 ────────────────────────────────────────────────────
    blocks.extend(vec![
        json!({ "type": "heading", "paraSequence": "2.2", "text": "Configuration Setters" }),
        json!({ "type": "table",
          "headers": ["Method", "Type", "Default", "Description"],
          "rows": [
              ["`title()`",                "impl Into<String>", "\"\"",          "Document title"],
              ["`custom_css()`",           "impl Into<String>", "None",         "Replace default CSS entirely"],
              ["`extra_css()`",            "impl Into<String>", "None",         "Append CSS after default"],
              ["`paper_size()`",           "PaperSize",         "A4",           "Paper dimensions"],
              ["`margins()`",              "PageMargins",       "0.75\" all",   "Page margins in inches"],
              ["`landscape()`",            "bool",              "false",        "Landscape orientation"],
              ["`header()`",               "HeaderConfig",      "empty",        "Page header"],
              ["`footer()`",               "FooterConfig",      "empty",        "Page footer"],
              ["`display_header_footer()`","bool",              "false",        "Show header/footer"],
              ["`heading_numbers()`",      "bool",              "false",        "CSS counter numbering"],
              ["`print_background()`",     "bool",              "true",         "Print background colors"],
              ["`markdown_options()`",     "MarkdownOptions",   "all true",     "Markdown extension toggles"],
              ["`chrome_window_size()`",   "(u32, u32)",        "(1280, 900)",  "Chrome viewport"],
              ["`page_load_wait_secs()`",  "u64",               "5",            "Wait after page load"]
          ]
        }),
    ]);

    // ─── 2.3 ────────────────────────────────────────────────────
    blocks.extend(vec![
        json!({ "type": "heading", "paraSequence": "2.3", "text": "Generation Methods" }),
        json!({ "type": "table",
          "headers": ["Method", "Input", "Description"],
          "rows": [
              ["`from_markdown(md)`",      "&str",              "Markdown string → PDF"],
              ["`from_markdown_file(path)`","impl AsRef<Path>",  "Read .md file → PDF"],
              ["`from_json(json)`",        "&serde_json::Value", "JSON blocks → Markdown → PDF"],
              ["`from_json_file(path)`",   "impl AsRef<Path>",  "Read .json file → PDF"],
              ["`from_html(html)`",        "&str",              "Raw HTML → PDF (no CSS injection)"]
          ]
        }),
    ]);

    // ─── Section 3 ──────────────────────────────────────────────
    blocks.extend(vec![
        json!({ "type": "heading", "paraSequence": "3", "text": "Configuration Types" }),
        json!({ "type": "heading", "paraSequence": "3.1", "text": "PaperSize" }),
        json!({ "type": "code", "language": "rust", "text": "pub enum PaperSize {\n    A4,                              // 8.27 × 11.69 in\n    Letter,                          // 8.5  × 11    in\n    Legal,                           // 8.5  × 14    in\n    Custom { width: f64, height: f64 }, // any size in inches\n}" }),
    ]);

    blocks.extend(vec![
        json!({ "type": "heading", "paraSequence": "3.2", "text": "PageMargins" }),
        json!({ "type": "code", "language": "rust", "text": "pub struct PageMargins {\n    pub top: f64,     // inches (default 0.75)\n    pub bottom: f64,  // inches (default 0.75)\n    pub left: f64,    // inches (default 0.75)\n    pub right: f64,   // inches (default 0.75)\n}" }),
    ]);

    blocks.extend(vec![
        json!({ "type": "heading", "paraSequence": "3.3", "text": "HeaderConfig / FooterConfig" }),
        json!({ "type": "paragraph", "text": "Both share the same structure.  Set `custom_html` for full control, or use `left` / `center` / `right` text fields:" }),
        json!({ "type": "code", "language": "rust", "text": "pub struct HeaderConfig {\n    pub custom_html: Option<String>,  // full HTML — overrides everything\n    pub left: Option<String>,\n    pub center: Option<String>,\n    pub right: Option<String>,\n    pub font_size: Option<String>,    // e.g. \"9px\"\n    pub color: Option<String>,        // e.g. \"#333\"\n}" }),
        json!({ "type": "paragraph", "text": "Chrome placeholders available in header/footer text:" }),
        json!({ "type": "table",
          "headers": ["Placeholder", "Renders As"],
          "rows": [
              ["<span class=\"pageNumber\">",  "Current page number"],
              ["<span class=\"totalPages\">",  "Total page count"],
              ["<span class=\"title\">",       "Document title"],
              ["<span class=\"date\">",        "Current date"]
          ]
        }),
    ]);

    blocks.extend(vec![
        json!({ "type": "heading", "paraSequence": "3.4", "text": "MarkdownOptions" }),
        json!({ "type": "code", "language": "rust", "text": "pub struct MarkdownOptions {\n    pub unsafe_html: bool,        // pass-through <html> tags\n    pub tables: bool,             // | col | col |\n    pub footnotes: bool,          // [^1]: text\n    pub description_lists: bool,  // term\\n: definition\n    pub strikethrough: bool,      // ~~text~~\n    pub tasklist: bool,           // - [x] item\n    pub autolink: bool,           // https://auto.link\n    pub superscript: bool,        // 2^10\n}  // All default to true" }),
    ]);

    // ─── Section 4 ──────────────────────────────────────────────
    blocks.extend(vec![
        json!({ "type": "heading", "paraSequence": "4", "text": "JSON Content Blocks" }),
        json!({ "type": "paragraph", "text": "When using `from_json()`, the input is an array of typed content blocks.  Each block has a `\"type\"` field and type-specific data." }),
    ]);

    blocks.extend(vec![
        json!({ "type": "heading", "paraSequence": "4.1", "text": "Block Type Reference" }),
        json!({ "type": "table",
          "headers": ["Type", "Key Fields", "Description"],
          "rows": [
              ["heading",    "paraSequence, text, level (opt)", "Section heading"],
              ["paragraph",  "text",                            "Body text (Markdown)"],
              ["code",       "language, text",                  "Fenced code block"],
              ["list",       "ordered, items",                  "Bullet or numbered list"],
              ["quote",      "text",                            "Blockquote"],
              ["table",      "headers, rows",                   "Data table"],
              ["image",      "src, alt",                        "Image from URL/path"],
              ["divider",    "—",                               "Horizontal rule"],
              ["html",       "text",                            "Raw HTML pass-through"]
          ]
        }),
    ]);

    blocks.extend(vec![
        json!({ "type": "heading", "paraSequence": "4.2", "text": "paraSequence" }),
        json!({ "type": "paragraph", "text": "The `paraSequence` field defines the hierarchical section number.  The heading level (h1–h6) is **automatically derived** from the depth:" }),
        json!({ "type": "table",
          "headers": ["paraSequence", "Depth", "Level", "Example"],
          "rows": [
              ["\"1\"",       "1", "h1", "# 1 Introduction"],
              ["\"1.1\"",     "2", "h2", "## 1.1 Background"],
              ["\"1.1.1\"",   "3", "h3", "### 1.1.1 Details"],
              ["\"A\"",       "1", "h1", "# A Appendix"],
              ["\"A.1\"",     "2", "h2", "## A.1 First section"]
          ]
        }),
        json!({ "type": "paragraph", "text": "You can override the derived level with an explicit `\"level\"` field if needed." }),
    ]);

    // ─── Section 5 ──────────────────────────────────────────────
    blocks.extend(vec![
        json!({ "type": "heading", "paraSequence": "5", "text": "Error Handling" }),
        json!({ "type": "code", "language": "rust", "text": "pub enum MdocError {\n    Chrome(String),\n    Io(std::io::Error),\n    Json(String),\n    ImageDownload { url: String, reason: String },\n    Other(String),\n}" }),
        json!({ "type": "paragraph", "text": "All generation methods return `pdfsmith::Result<Vec<u8>>`." }),
        json!({ "type": "code", "language": "rust", "text": "match PdfBuilder::new().from_markdown(\"# Test\") {\n    Ok(pdf)                       => std::fs::write(\"out.pdf\", pdf).unwrap(),\n    Err(MdocError::Chrome(msg))   => eprintln!(\"Chrome: {msg}\"),\n    Err(MdocError::Io(err))       => eprintln!(\"IO: {err}\"),\n    Err(e)                        => eprintln!(\"Error: {e}\"),\n}" }),
    ]);

    // ─── Section 6 ──────────────────────────────────────────────
    blocks.extend(vec![
        json!({ "type": "heading", "paraSequence": "6", "text": "CSS Customisation" }),
        json!({ "type": "heading", "paraSequence": "6.1", "text": "custom_css vs extra_css" }),
        json!({ "type": "table",
          "headers": ["Method", "Behaviour", "Use When"],
          "rows": [
              ["`custom_css()`", "Replaces entire default stylesheet", "Full control"],
              ["`extra_css()`",  "Appended after default stylesheet", "Small tweaks"]
          ]
        }),
        json!({ "type": "heading", "paraSequence": "6.2", "text": "heading_numbers(true)" }),
        json!({ "type": "paragraph", "text": "Injects CSS counter rules for automatic hierarchical heading numbers.  Works with **all** input types: Markdown, JSON, and HTML." }),
        json!({ "type": "divider" }),
        json!({ "type": "paragraph", "text": "*End of API Reference — pdfsmith v0.1.0*" }),
    ]);

    serde_json::Value::Array(blocks)
}

fn main() {
    env_logger::init();

    let json = build_content();

    let pdf = PdfBuilder::new()
        .title("pdfsmith API Reference")
        .margins(PageMargins {
            top: 0.9,
            bottom: 0.9,
            left: 0.7,
            right: 0.7,
        })
        .display_header_footer(true)
        .header(HeaderConfig {
            left: Some("pdfsmith v0.1.0".into()),
            right: Some("API Reference".into()),
            font_size: Some("8px".into()),
            color: Some("#555".into()),
            ..Default::default()
        })
        .footer(FooterConfig {
            center: Some(
                r#"— <span class="pageNumber"></span> —"#.into(),
            ),
            font_size: Some("8px".into()),
            color: Some("#999".into()),
            ..Default::default()
        })
        .extra_css("h1 { color: #1a5276; } h2 { color: #2874a6; margin-left: 12px; } h3 { color: #1e8449; margin-left: 24px; }")
        .from_json(&json)
        .expect("Failed to generate PDF");

    std::fs::write("examples/output_pdfs/technical_doc.pdf", &pdf).expect("Failed to write PDF");
    println!("PDF saved to examples/output_pdfs/technical_doc.pdf ({} bytes)", pdf.len());
}
