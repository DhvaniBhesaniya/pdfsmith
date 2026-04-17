use pdfsmith::PdfBuilder;
use serde_json::json;

#[test]
fn test_json_array_to_pdf() {
    let json = json!([
        { "type": "heading", "level": 1, "text": "Title" },
        { "type": "paragraph", "text": "Hello **world**." }
    ]);

    let pdf = PdfBuilder::new()
        .from_json(&json)
        .unwrap();

    assert!(pdf.len() > 100);
    assert_eq!(&pdf[..5], b"%PDF-");
}

#[test]
fn test_json_object_with_content_key() {
    let json = json!({
        "content": [
            { "type": "heading", "level": 2, "text": "Section" },
            { "type": "paragraph", "text": "Body text." }
        ]
    });

    let pdf = PdfBuilder::new()
        .from_json(&json)
        .unwrap();

    assert!(pdf.len() > 100);
}

#[test]
fn test_json_all_block_types() {
    let json = json!([
        { "type": "heading", "level": 1, "text": "Title" },
        { "type": "paragraph", "text": "Text." },
        { "type": "code", "language": "rust", "text": "fn main() {}" },
        { "type": "list", "ordered": true, "items": ["A", "B"] },
        { "type": "quote", "text": "Wisdom." },
        { "type": "table", "headers": ["X","Y"], "rows": [["1","2"]] },
        { "type": "image", "src": "https://example.com/img.png", "alt": "test" },
        { "type": "divider" },
        { "type": "html", "text": "<b>raw</b>" }
    ]);

    let pdf = PdfBuilder::new()
        .from_json(&json)
        .unwrap();

    assert!(pdf.len() > 100);
}

#[test]
fn test_json_invalid_structure_errors() {
    // A plain string is not a valid JSON structure for us
    let json = json!("just a string");

    let result = PdfBuilder::new().from_json(&json);
    assert!(result.is_err());
}

#[test]
fn test_json_para_sequence_headings() {
    // paraSequence defines hierarchical heading numbers and derives level
    let json = json!([
        { "type": "heading", "paraSequence": "1",     "text": "Introduction" },
        { "type": "paragraph", "text": "Intro text." },
        { "type": "heading", "paraSequence": "1.1",   "text": "Background" },
        { "type": "heading", "paraSequence": "1.2",   "text": "Scope" },
        { "type": "heading", "paraSequence": "1.2.1", "text": "Details" },
        { "type": "heading", "paraSequence": "2",     "text": "Conclusion" }
    ]);

    let pdf = PdfBuilder::new()
        .from_json(&json)
        .unwrap();

    assert!(pdf.len() > 100);
    assert_eq!(&pdf[..5], b"%PDF-");
}

#[test]
fn test_json_para_sequence_with_explicit_level() {
    // User can override the derived level with explicit "level"
    let json = json!([
        { "type": "heading", "paraSequence": "A",   "level": 1, "text": "Appendix A" },
        { "type": "heading", "paraSequence": "A.1", "level": 2, "text": "First Section" },
        { "type": "paragraph", "text": "Content." }
    ]);

    let pdf = PdfBuilder::new()
        .from_json(&json)
        .unwrap();

    assert!(pdf.len() > 100);
    assert_eq!(&pdf[..5], b"%PDF-");
}

#[test]
fn test_json_heading_without_para_sequence() {
    // Headings without paraSequence still work fine (no number prefix)
    let json = json!([
        { "type": "heading", "text": "Simple Title" },
        { "type": "paragraph", "text": "No numbering." }
    ]);

    let pdf = PdfBuilder::new()
        .from_json(&json)
        .unwrap();

    assert!(pdf.len() > 100);
    assert_eq!(&pdf[..5], b"%PDF-");
}

#[test]
fn test_heading_numbers_css_option() {
    // heading_numbers(true) should work with Markdown input (CSS counters)
    let pdf = PdfBuilder::new()
        .heading_numbers(true)
        .from_markdown("# Intro\n\n## Part A\n\n## Part B\n\n### Detail\n\n# Summary")
        .unwrap();

    assert!(pdf.len() > 100);
    assert_eq!(&pdf[..5], b"%PDF-");
}
