// ─────────────────────────────────────────────────────────────────────────────
// config.rs — All user-facing configuration types
//
// Every aspect of the generated PDF is customizable.
// `PdfConfig::default()` produces a clean, minimal PDF.
// ─────────────────────────────────────────────────────────────────────────────

use serde::{Deserialize, Serialize};

// ─────────────────────────────────────────────────────────────────────────────
// Paper size
// ─────────────────────────────────────────────────────────────────────────────

/// Standard paper sizes (width × height in inches).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PaperSize {
    /// 8.27 × 11.69 in
    A4,
    /// 8.5 × 11 in
    Letter,
    /// 8.5 × 14 in
    Legal,
    /// Custom width × height in inches.
    Custom { width: f64, height: f64 },
}

impl PaperSize {
    /// Returns `(width, height)` in inches.
    pub fn dimensions(&self) -> (f64, f64) {
        match self {
            PaperSize::A4 => (8.27, 11.69),
            PaperSize::Letter => (8.5, 11.0),
            PaperSize::Legal => (8.5, 14.0),
            PaperSize::Custom { width, height } => (*width, *height),
        }
    }
}

impl Default for PaperSize {
    fn default() -> Self {
        PaperSize::A4
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Page margins
// ─────────────────────────────────────────────────────────────────────────────

/// Page margins in inches.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PageMargins {
    pub top: f64,
    pub bottom: f64,
    pub left: f64,
    pub right: f64,
}

impl Default for PageMargins {
    fn default() -> Self {
        Self {
            top: 0.75,
            bottom: 0.75,
            left: 0.75,
            right: 0.75,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Header configuration
// ─────────────────────────────────────────────────────────────────────────────

/// Controls the repeating page header.
///
/// **Option A — custom_html**: provide your own HTML template.  Chrome's
/// header sandbox renders this on every page.  Use `<span class="pageNumber">`
/// and `<span class="totalPages">` for automatic page numbers.
///
/// **Option B — structured fields**: set `left`, `center`, `right` text and
/// the library builds the template for you.
///
/// **Option C — do nothing**: leave everything `None` and the default header
/// is just the document title, small and centred.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HeaderConfig {
    /// Full custom HTML template — used as-is, everything else is ignored.
    pub custom_html: Option<String>,

    /// Text for the left side of the header.
    pub left: Option<String>,
    /// Text for the centre of the header.
    pub center: Option<String>,
    /// Text for the right side of the header.
    pub right: Option<String>,

    /// Font size (CSS value, e.g. `"10px"`, `"8pt"`). Default: `"9px"`.
    pub font_size: Option<String>,
    /// Font colour (CSS value). Default: `"#555"`.
    pub color: Option<String>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Footer configuration
// ─────────────────────────────────────────────────────────────────────────────

/// Controls the repeating page footer.
///
/// Same three options as [`HeaderConfig`]: custom HTML, structured
/// left/center/right fields, or just leave defaults.
///
/// The default footer is a simple centred page number: `Page 1 / 5`.
///
/// To use Chrome's auto page numbers in `custom_html`, include
/// `<span class="pageNumber"></span>` and `<span class="totalPages"></span>`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FooterConfig {
    /// Full custom HTML template — used as-is, everything else is ignored.
    pub custom_html: Option<String>,

    /// Text for the left side of the footer.
    pub left: Option<String>,
    /// Text for the centre of the footer.  Default: `"Page <pageNumber> / <totalPages>"`.
    pub center: Option<String>,
    /// Text for the right side of the footer.
    pub right: Option<String>,

    /// Font size (CSS value). Default: `"8px"`.
    pub font_size: Option<String>,
    /// Font colour (CSS value). Default: `"#555"`.
    pub color: Option<String>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Markdown options
// ─────────────────────────────────────────────────────────────────────────────

/// Controls which comrak Markdown extensions are enabled.
///
/// All default to `true` — the renderer is batteries-included.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkdownOptions {
    pub unsafe_html: bool,
    pub tables: bool,
    pub footnotes: bool,
    pub description_lists: bool,
    pub strikethrough: bool,
    pub tasklist: bool,
    pub autolink: bool,
    pub superscript: bool,
}

impl Default for MarkdownOptions {
    fn default() -> Self {
        Self {
            unsafe_html: true,
            tables: true,
            footnotes: true,
            description_lists: true,
            strikethrough: true,
            tasklist: true,
            autolink: true,
            superscript: true,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Main configuration struct
// ─────────────────────────────────────────────────────────────────────────────

/// Top-level configuration for PDF generation.
///
/// Every field has a sensible default.  Use `PdfConfig::default()` for a
/// clean, no-frills PDF, or customise exactly what you need.
///
/// # Minimal
/// ```no_run
/// use pdfsmith::PdfConfig;
/// let cfg = PdfConfig::default();
/// ```
///
/// # Custom
/// ```no_run
/// use pdfsmith::{PdfConfig, PaperSize, PageMargins, FooterConfig};
/// let cfg = PdfConfig {
///     custom_css: Some("body { font-family: Georgia, serif; font-size: 12pt; }".into()),
///     paper_size: PaperSize::Letter,
///     margins: PageMargins { top: 0.5, bottom: 0.5, left: 0.5, right: 0.5 },
///     footer: FooterConfig { right: Some("Confidential".into()), ..Default::default() },
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfConfig {
    // ── Document metadata ────────────────────────────────────────────────

    /// Document title (used in `<title>` and default header).
    pub title: String,

    // ── Styling ──────────────────────────────────────────────────────────

    /// Custom CSS that **replaces** the built-in stylesheet entirely.
    pub custom_css: Option<String>,

    /// Extra CSS **appended** after the base stylesheet.
    /// Ideal for small tweaks without replacing the whole default.
    pub extra_css: Option<String>,

    // ── Page layout ──────────────────────────────────────────────────────

    pub paper_size: PaperSize,
    pub margins: PageMargins,
    /// Landscape orientation. Default: `false` (portrait).
    pub landscape: bool,

    // ── Header / Footer ──────────────────────────────────────────────────

    pub header: HeaderConfig,
    pub footer: FooterConfig,

    /// Show header & footer at all. Default: `false` (clean pages).
    pub display_header_footer: bool,

    // ── Rendering ────────────────────────────────────────────────────────

    /// Print background colours/images. Default: `true`.
    pub print_background: bool,

    /// Markdown parsing options.
    pub markdown_options: MarkdownOptions,

    /// Automatically number headings with hierarchical numbers
    /// (1, 1.1, 1.1.1, etc.) via CSS counters.
    ///
    /// Works with **all** input types: Markdown, JSON, and HTML.
    /// Default: `false`.
    pub heading_numbers: bool,

    // ── Chrome ───────────────────────────────────────────────────────────

    /// Chrome headless window size `(width, height)`.
    pub chrome_window_size: (u32, u32),

    /// Seconds to wait after page load before printing. Default: `2`.
    pub page_load_wait_secs: u64,
}

impl Default for PdfConfig {
    fn default() -> Self {
        Self {
            title: String::new(),
            custom_css: None,
            extra_css: None,
            paper_size: PaperSize::default(),
            margins: PageMargins::default(),
            landscape: false,
            header: HeaderConfig::default(),
            footer: FooterConfig::default(),
            display_header_footer: false,
            print_background: true,
            markdown_options: MarkdownOptions::default(),
            heading_numbers: false,
            chrome_window_size: (1280, 900),
            page_load_wait_secs: 5,
        }
    }
}
