use pdfsmith::PdfBuilder;

#[test]
fn test_markdown_to_pdf() {
    let pdf = PdfBuilder::new()
        .from_markdown("# Hello\n\nWorld.")
        .unwrap();

    assert!(pdf.len() > 100, "PDF should have content");
    // PDF files start with %PDF
    assert_eq!(&pdf[..5], b"%PDF-");
}

#[test]
fn test_markdown_with_custom_css() {
    let pdf = PdfBuilder::new()
        .custom_css("body { font-family: monospace; }")
        .from_markdown("# Mono")
        .unwrap();

    assert!(pdf.len() > 100);
}

#[test]
fn test_markdown_with_extra_css() {
    let pdf = PdfBuilder::new()
        .extra_css("h1 { color: red; }")
        .from_markdown("# Red Title")
        .unwrap();

    assert!(pdf.len() > 100);
}

#[test]
fn test_heading_numbers_with_markdown() {
    let pdf = PdfBuilder::new()
        .heading_numbers(true)
        .from_markdown("# Intro\n\n## Part A\n\n## Part B\n\n### Detail\n\n# Summary")
        .unwrap();

    assert!(pdf.len() > 100);
    assert_eq!(&pdf[..5], b"%PDF-");
}

#[test]
fn test_heading_numbers_with_custom_css() {
    let pdf = PdfBuilder::new()
        .custom_css("body { font-family: Georgia; } h1 { color: navy; }")
        .heading_numbers(true)
        .from_markdown("# Title\n\n## Sub\n\n### Deep")
        .unwrap();

    assert!(pdf.len() > 100);
    assert_eq!(&pdf[..5], b"%PDF-");
}

#[test]
fn test_heading_numbers_with_html() {
    let html = r#"<!DOCTYPE html>
<html><head><style>body{font-family:sans-serif;}</style></head>
<body>
<h1>Main</h1>
<h2>Sub A</h2>
<h2>Sub B</h2>
<h3>Detail</h3>
</body></html>"#;

    let pdf = PdfBuilder::new()
        .heading_numbers(true)
        .from_html(html)
        .unwrap();

    assert!(pdf.len() > 100);
    assert_eq!(&pdf[..5], b"%PDF-");
}
