// ─────────────────────────────────────────────────────────────────────────────
// renderer/chrome.rs — Headless Chrome PDF rendering
// ─────────────────────────────────────────────────────────────────────────────

use headless_chrome::types::PrintToPdfOptions;
use headless_chrome::{Browser, LaunchOptions};
use log::info;
use std::path::PathBuf;

use crate::config::PdfConfig;
use crate::error::{MdocError, Result};

/// Launches headless Chrome, loads the HTML, and prints to PDF.
///
/// # Arguments
/// - `full_html`       — complete HTML document string.
/// - `header_template` — Chrome header template HTML.
/// - `footer_template` — Chrome footer template HTML.
/// - `config`          — PDF configuration (margins, paper size, etc.).
///
/// # Returns
/// Raw PDF bytes on success.
pub fn render_html_to_pdf(
    full_html: &str,
    header_template: &str,
    footer_template: &str,
    config: &PdfConfig,
) -> Result<Vec<u8>> {
    info!("Launching headless Chrome…");

    let launch_options = LaunchOptions::default_builder()
        .headless(true)
        .window_size(Some(config.chrome_window_size))
        .build()
        .map_err(|e| MdocError::Chrome(format!("Failed to build launch options: {}", e)))?;

    let browser = Browser::new(launch_options).map_err(|e| {
        MdocError::Chrome(format!(
            "Failed to launch Chrome/Chromium: {}.\n\
             Install with:  sudo apt install chromium-browser\n\
             Or:            sudo snap install chromium",
            e
        ))
    })?;

    let tab = browser
        .new_tab()
        .map_err(|e| MdocError::Chrome(format!("Failed to open new tab: {}", e)))?;

    // Write temp HTML file (Chrome needs a file:// URL)
    let temp_html_path: PathBuf = std::env::temp_dir().join("_pdfsmith_render_temp.html");

    std::fs::write(&temp_html_path, full_html)?;

    let file_url = format!("file://{}", temp_html_path.display());
    info!("Temp HTML written: {}", file_url);

    tab.navigate_to(&file_url)
        .map_err(|e| MdocError::Chrome(format!("Navigation failed: {}", e)))?;

    tab.wait_until_navigated()
        .map_err(|e| MdocError::Chrome(format!("Navigation timed out: {}", e)))?;

    // Wait for body and images to load
    let _ = tab.wait_for_element("body");
    if config.page_load_wait_secs > 0 {
        std::thread::sleep(std::time::Duration::from_secs(config.page_load_wait_secs));
    }

    info!("Page loaded; printing to PDF…");

    let (paper_width, paper_height) = config.paper_size.dimensions();

    let pdf_options = PrintToPdfOptions {
        display_header_footer: Some(config.display_header_footer),
        print_background: Some(config.print_background),
        header_template: Some(header_template.to_string()),
        footer_template: Some(footer_template.to_string()),
        margin_top: Some(config.margins.top),
        margin_bottom: Some(config.margins.bottom),
        margin_left: Some(config.margins.left),
        margin_right: Some(config.margins.right),
        paper_width: Some(paper_width),
        paper_height: Some(paper_height),
        landscape: Some(config.landscape),
        prefer_css_page_size: Some(false),
        ..PrintToPdfOptions::default()
    };

    let pdf_bytes = tab
        .print_to_pdf(Some(pdf_options))
        .map_err(|e| MdocError::Chrome(format!("print-to-PDF failed: {}", e)))?;

    info!("Chrome PDF rendering complete ({} bytes)", pdf_bytes.len());

    // Clean up temp file
    if temp_html_path.exists() {
        let _ = std::fs::remove_file(&temp_html_path);
    }

    Ok(pdf_bytes)
}
