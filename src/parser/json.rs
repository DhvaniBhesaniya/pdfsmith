// ─────────────────────────────────────────────────────────────────────────────
// parser/json.rs — JSON structured document → Markdown conversion
//
// Accepts a simple, intuitive JSON structure and converts it into Markdown.
// The Markdown is then rendered to HTML by comrak (same as `from_markdown`).
//
// ─── JSON Structure ──────────────────────────────────────────────────────────
//
// The JSON is an array of *content blocks*.  Each block has a `"type"` field.
//
// Every block can optionally carry a `"paraSequence"` field that defines its
// hierarchical position in the document:
//
//   "paraSequence": "1"       → top-level section
//   "paraSequence": "1.1"     → sub-section
//   "paraSequence": "1.1.1"   → sub-sub-section
//   "paraSequence": "2"       → next top-level section
//
// For **heading** blocks, the heading level (h1–h6) is automatically derived
// from the depth of the paraSequence (number of dot-separated parts).
// You can still override it with an explicit `"level"` field.
//
// Supported block types:
//
//   { "type": "heading",   "paraSequence": "1",   "text": "Introduction" }
//   { "type": "heading",   "paraSequence": "1.1", "text": "Background" }
//   { "type": "paragraph", "text": "Some **markdown** text." }
//   { "type": "code",      "language": "rust", "text": "fn main() {}" }
//   { "type": "list",      "ordered": false, "items": ["Item A", "Item B"] }
//   { "type": "quote",     "text": "Something wise." }
//   { "type": "table",     "headers": ["A","B"], "rows": [["1","2"]] }
//   { "type": "image",     "src": "https://…", "alt": "logo" }
//   { "type": "divider" }
//   { "type": "html",      "text": "<div class='custom'>…</div>" }
//
// The top-level JSON can be:
//   • An array directly: `[ { "type": … }, … ]`
//   • An object with a `"content"` array: `{ "content": [ … ] }`
//
// ─────────────────────────────────────────────────────────────────────────────

use serde_json::Value;

use crate::error::{MdocError, Result};

/// Converts a JSON document into a Markdown string.
///
/// The returned Markdown is then fed through comrak like any other
/// Markdown input — the user's CSS and config apply normally.
///
/// # `paraSequence`
///
/// Every block can carry an optional `"paraSequence"` field — a string
/// like `"1"`, `"1.1"`, `"1.2.3"` that defines its hierarchical position.
///
/// For **heading** blocks, this does two things:
///   1. The heading level (h1–h6) is **derived** from the depth of the
///      sequence (e.g. `"1.2"` → h2, `"1.2.3"` → h3).
///   2. The number is prefixed to the heading text automatically.
///
/// You can override the derived level with an explicit `"level"` field.
///
/// ```json
/// [
///   { "type": "heading", "paraSequence": "1",     "text": "Introduction" },
///   { "type": "heading", "paraSequence": "1.1",   "text": "Background" },
///   { "type": "heading", "paraSequence": "1.2",   "text": "Scope" },
///   { "type": "heading", "paraSequence": "1.2.1", "text": "Details" },
///   { "type": "heading", "paraSequence": "2",     "text": "Conclusion" }
/// ]
/// ```
///
/// Produces:
///   - `# 1 Introduction`
///   - `## 1.1 Background`
///   - `## 1.2 Scope`
///   - `### 1.2.1 Details`
///   - `# 2 Conclusion`
pub fn json_to_markdown(json_doc: &Value) -> Result<String> {
    let blocks = resolve_content_array(json_doc)?;

    let mut md = String::new();

    for block in blocks {
        let block_type = block["type"]
            .as_str()
            .unwrap_or("paragraph");

        match block_type {
            "heading" => render_heading(block, &mut md),
            "paragraph" | "text" => render_paragraph(block, &mut md),
            "code" => render_code(block, &mut md),
            "list" => render_list(block, &mut md),
            "quote" | "blockquote" => render_quote(block, &mut md),
            "table" => render_table(block, &mut md),
            "image" | "img" => render_image(block, &mut md),
            "divider" | "hr" => render_divider(&mut md),
            "html" | "raw" => render_raw_html(block, &mut md),
            _ => {
                // Unknown type — treat text field as a paragraph if present
                if let Some(text) = block["text"].as_str() {
                    md.push_str(text);
                    md.push_str("\n\n");
                }
            }
        }
    }

    Ok(md)
}

/// Resolves the content blocks array from the JSON input.
///
/// Accepts:
///   - A bare array: `[ { "type": … }, … ]`
///   - An object with `"content"`: `{ "content": [ … ] }`
fn resolve_content_array(json_doc: &Value) -> Result<&Vec<Value>> {
    if let Some(arr) = json_doc.as_array() {
        return Ok(arr);
    }

    if let Some(arr) = json_doc["content"].as_array() {
        return Ok(arr);
    }

    Err(MdocError::Json(
        "JSON must be an array of content blocks, or an object with a \"content\" array."
            .to_string(),
    ))
}

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Returns the depth of a paraSequence string.
/// `"1"` → 1, `"1.2"` → 2, `"1.2.3"` → 3, etc.
fn sequence_depth(seq: &str) -> usize {
    seq.split('.').count()
}

// ─────────────────────────────────────────────────────────────────────────────
// Block renderers
// ─────────────────────────────────────────────────────────────────────────────

fn render_heading(block: &Value, md: &mut String) {
    let para_seq = block["paraSequence"].as_str();
    let text = block["text"].as_str().unwrap_or("");

    // Heading level: explicit "level", or derived from paraSequence depth,
    // or default 1.
    let level = if let Some(lvl) = block["level"].as_u64() {
        lvl.min(6).max(1) as usize
    } else if let Some(seq) = para_seq {
        sequence_depth(seq).min(6).max(1)
    } else {
        1
    };

    let hashes = "#".repeat(level);

    match para_seq {
        Some(seq) => md.push_str(&format!("{} {} {}\n\n", hashes, seq, text)),
        None => md.push_str(&format!("{} {}\n\n", hashes, text)),
    }
}

fn render_paragraph(block: &Value, md: &mut String) {
    let text = block["text"].as_str().unwrap_or("");
    if !text.is_empty() {
        md.push_str(text);
        md.push_str("\n\n");
    }
}

fn render_code(block: &Value, md: &mut String) {
    let lang = block["language"].as_str().unwrap_or("");
    let text = block["text"].as_str().unwrap_or("");
    md.push_str(&format!("```{}\n{}\n```\n\n", lang, text));
}

fn render_list(block: &Value, md: &mut String) {
    let ordered = block["ordered"].as_bool().unwrap_or(false);

    if let Some(items) = block["items"].as_array() {
        for (i, item) in items.iter().enumerate() {
            let text = item.as_str().unwrap_or("");
            if ordered {
                md.push_str(&format!("{}. {}\n", i + 1, text));
            } else {
                md.push_str(&format!("- {}\n", text));
            }
        }
        md.push('\n');
    }
}

fn render_quote(block: &Value, md: &mut String) {
    let text = block["text"].as_str().unwrap_or("");
    for line in text.lines() {
        md.push_str(&format!("> {}\n", line));
    }
    md.push('\n');
}

fn render_table(block: &Value, md: &mut String) {
    let headers = match block["headers"].as_array() {
        Some(h) => h,
        None => return,
    };
    let rows = block["rows"].as_array();

    // Header row
    let header_cells: Vec<&str> = headers
        .iter()
        .map(|h| h.as_str().unwrap_or(""))
        .collect();
    md.push_str(&format!("| {} |\n", header_cells.join(" | ")));

    // Separator
    let sep: Vec<&str> = header_cells.iter().map(|_| "---").collect();
    md.push_str(&format!("| {} |\n", sep.join(" | ")));

    // Data rows
    if let Some(rows) = rows {
        for row in rows {
            if let Some(cells) = row.as_array() {
                let cell_strs: Vec<&str> = cells
                    .iter()
                    .map(|c| c.as_str().unwrap_or(""))
                    .collect();
                md.push_str(&format!("| {} |\n", cell_strs.join(" | ")));
            }
        }
    }
    md.push('\n');
}

fn render_image(block: &Value, md: &mut String) {
    let src = block["src"].as_str().unwrap_or("");
    let alt = block["alt"].as_str().unwrap_or("");
    md.push_str(&format!("![{}]({})\n\n", alt, src));
}

fn render_divider(md: &mut String) {
    md.push_str("---\n\n");
}

fn render_raw_html(block: &Value, md: &mut String) {
    let text = block["text"].as_str().unwrap_or("");
    if !text.is_empty() {
        md.push_str(text);
        md.push_str("\n\n");
    }
}
