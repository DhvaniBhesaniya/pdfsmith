#![doc = include_str!("../README_DOCS.md")]

pub mod config;
pub mod css;
pub mod error;
pub mod parser;
pub mod renderer;

// ── Re-exports ───────────────────────────────────────────────────────────────

pub use config::{
    FooterConfig, HeaderConfig, MarkdownOptions, PageMargins, PaperSize, PdfConfig,
};
pub use css::DEFAULT_CSS;
pub use error::{MdocError, Result};

use log::info;
use serde_json::Value;

use parser::{json_to_markdown, markdown_to_body_html};
use renderer::{
    build_footer_template, build_header_template, render_html_to_pdf, wrap_body_in_html_document,
};

// ─────────────────────────────────────────────────────────────────────────────
// PdfBuilder
// ─────────────────────────────────────────────────────────────────────────────

/// Fluent builder for generating PDFs.
///
/// All settings have sensible defaults — you can generate a PDF with just
/// `PdfBuilder::new().from_markdown(…)`.  Customise anything you want
/// via the chainable setter methods.
///
/// # From Markdown
/// ```no_run
/// use pdfsmith::PdfBuilder;
/// let pdf = PdfBuilder::new()
///     .from_markdown("# Hello")
///     .unwrap();
/// ```
///
/// # From Markdown with custom CSS
/// ```no_run
/// use pdfsmith::PdfBuilder;
/// let pdf = PdfBuilder::new()
///     .custom_css("body { font-family: Georgia; font-size: 12pt; }")
///     .from_markdown("# Styled")
///     .unwrap();
/// ```
///
/// # From JSON
/// ```no_run
/// use pdfsmith::PdfBuilder;
/// let json = serde_json::json!([
///     { "type": "heading", "level": 1, "text": "Report" },
///     { "type": "paragraph", "text": "Revenue grew **20%**." },
///     { "type": "table", "headers": ["Q","Rev"], "rows": [["Q1","$1M"]] }
/// ]);
/// let pdf = PdfBuilder::new()
///     .from_json(&json)
///     .unwrap();
/// ```
pub struct PdfBuilder {
    config: PdfConfig,
}

impl PdfBuilder {
    /// New builder with default configuration.
    pub fn new() -> Self {
        Self {
            config: PdfConfig::default(),
        }
    }

    /// New builder from an existing [`PdfConfig`].
    pub fn with_config(config: PdfConfig) -> Self {
        Self { config }
    }

    // ── Chainable setters ────────────────────────────────────────────────

    /// Replace the entire configuration.
    pub fn config(mut self, config: PdfConfig) -> Self {
        self.config = config;
        self
    }

    /// Set the document title (used in `<title>` and default header).
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.config.title = title.into();
        self
    }

    /// **Replace** the default CSS with your own stylesheet.
    pub fn custom_css(mut self, css: impl Into<String>) -> Self {
        self.config.custom_css = Some(css.into());
        self
    }

    /// **Append** extra CSS after the base stylesheet.
    pub fn extra_css(mut self, css: impl Into<String>) -> Self {
        self.config.extra_css = Some(css.into());
        self
    }

    /// Set paper size — `A4`, `Letter`, `Legal`, or `Custom { width, height }`.
    pub fn paper_size(mut self, size: PaperSize) -> Self {
        self.config.paper_size = size;
        self
    }

    /// Set page margins (in inches).
    pub fn margins(mut self, margins: PageMargins) -> Self {
        self.config.margins = margins;
        self
    }

    /// Enable landscape orientation.
    pub fn landscape(mut self, yes: bool) -> Self {
        self.config.landscape = yes;
        self
    }

    /// Configure the page header (left / center / right text, or custom HTML).
    pub fn header(mut self, header: HeaderConfig) -> Self {
        self.config.header = header;
        self
    }

    /// Configure the page footer (left / center / right text, or custom HTML).
    pub fn footer(mut self, footer: FooterConfig) -> Self {
        self.config.footer = footer;
        self
    }

    /// Show or hide header & footer. Default: `false`.
    pub fn display_header_footer(mut self, yes: bool) -> Self {
        self.config.display_header_footer = yes;
        self
    }

    /// Print background colours/images. Default: `true`.
    pub fn print_background(mut self, yes: bool) -> Self {
        self.config.print_background = yes;
        self
    }

    /// Configure Markdown parsing extensions.
    pub fn markdown_options(mut self, opts: MarkdownOptions) -> Self {
        self.config.markdown_options = opts;
        self
    }

    /// Chrome window size. Default: `(1280, 900)`.
    pub fn chrome_window_size(mut self, width: u32, height: u32) -> Self {
        self.config.chrome_window_size = (width, height);
        self
    }

    /// Seconds to wait after page load. Default: `2`.
    pub fn page_load_wait_secs(mut self, secs: u64) -> Self {
        self.config.page_load_wait_secs = secs;
        self
    }

    /// Enable automatic hierarchical heading numbers (1, 1.1, 1.1.1, …)
    /// via CSS counters.  Works with all input types.
    pub fn heading_numbers(mut self, yes: bool) -> Self {
        self.config.heading_numbers = yes;
        self
    }

    // ── Generation methods ───────────────────────────────────────────────

    /// Generate a PDF from a Markdown string.
    pub fn from_markdown(self, md: &str) -> Result<Vec<u8>> {
        info!("Generating PDF from Markdown ({} chars)", md.len());

        let body_html = markdown_to_body_html(md, &self.config)?;
        let full_html = wrap_body_in_html_document(&body_html, &self.config);
        let header = build_header_template(&self.config);
        let footer = build_footer_template(&self.config);

        let pdf = render_html_to_pdf(&full_html, &header, &footer, &self.config)?;
        info!("PDF from Markdown: {} bytes", pdf.len());
        Ok(pdf)
    }

    /// Generate a PDF from structured JSON content blocks.
    ///
    /// The JSON is converted to Markdown first, then rendered with the
    /// same pipeline as `from_markdown`.
    ///
    /// # JSON Structure
    ///
    /// An array of content blocks (or an object with a `"content"` array):
    ///
    /// ```json
    /// [
    ///   { "type": "heading", "level": 1, "text": "Title" },
    ///   { "type": "paragraph", "text": "Some **bold** text." },
    ///   { "type": "code", "language": "rust", "text": "fn main() {}" },
    ///   { "type": "list", "ordered": false, "items": ["A", "B", "C"] },
    ///   { "type": "quote", "text": "A wise quote." },
    ///   { "type": "table", "headers": ["X","Y"], "rows": [["1","2"]] },
    ///   { "type": "image", "src": "https://…", "alt": "photo" },
    ///   { "type": "divider" },
    ///   { "type": "html", "text": "<b>raw HTML</b>" }
    /// ]
    /// ```
    pub fn from_json(self, json_doc: &Value) -> Result<Vec<u8>> {
        info!("Generating PDF from JSON");

        let markdown = json_to_markdown(json_doc)?;
        self.from_markdown(&markdown)
    }

    /// Generate a PDF from a pre-built HTML string.
    ///
    /// The HTML is used **as-is** — no Markdown conversion, no CSS injection.
    /// Header/footer and Chrome options still apply.
    pub fn from_html(self, html: &str) -> Result<Vec<u8>> {
        info!("Generating PDF from raw HTML ({} chars)", html.len());

        let header = build_header_template(&self.config);
        let footer = build_footer_template(&self.config);

        let pdf = render_html_to_pdf(html, &header, &footer, &self.config)?;
        info!("PDF from HTML: {} bytes", pdf.len());
        Ok(pdf)
    }

    /// Generate a PDF from a Markdown **file** on disk.
    pub fn from_markdown_file(self, path: impl AsRef<std::path::Path>) -> Result<Vec<u8>> {
        let md = std::fs::read_to_string(path.as_ref())?;
        self.from_markdown(&md)
    }

    /// Generate a PDF from a JSON **file** on disk.
    pub fn from_json_file(self, path: impl AsRef<std::path::Path>) -> Result<Vec<u8>> {
        let content = std::fs::read_to_string(path.as_ref())?;
        let json_doc: Value =
            serde_json::from_str(&content).map_err(|e| MdocError::Json(e.to_string()))?;
        self.from_json(&json_doc)
    }
}

impl Default for PdfBuilder {
    fn default() -> Self {
        Self::new()
    }
}

