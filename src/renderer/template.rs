// ─────────────────────────────────────────────────────────────────────────────
// renderer/template.rs — Simple, clean header & footer template builders
//
// Chrome's print-to-PDF accepts custom HTML strings for the repeating
// header and footer.  The user can pass their own HTML, or use the
// simple left / center / right layout that this module builds.
//
// Defaults:
//   header → empty (no header)
//   footer → centred page number: "Page 1 / 5"
// ─────────────────────────────────────────────────────────────────────────────

use crate::config::PdfConfig;

// ─────────────────────────────────────────────────────────────────────────────
// Header
// ─────────────────────────────────────────────────────────────────────────────

/// Builds the Chrome PDF header template.
///
/// Priority:
///   1. `header.custom_html` → returned as-is.
///   2. Structured `left` / `center` / `right` → three-column flex layout.
///   3. Nothing configured → empty `<span>` (invisible).
pub fn build_header_template(config: &PdfConfig) -> String {
    if !config.display_header_footer {
        return "<span></span>".into();
    }

    // Custom HTML takes full control
    if let Some(ref html) = config.header.custom_html {
        return html.clone();
    }

    let h = &config.header;
    let left = h.left.as_deref().unwrap_or("");
    let center = h.center.as_deref().unwrap_or("");
    let right = h.right.as_deref().unwrap_or("");

    // If nothing is configured, produce an invisible header
    if left.is_empty() && center.is_empty() && right.is_empty() {
        return "<span></span>".into();
    }

    let font_size = h.font_size.as_deref().unwrap_or("9px");
    let color = h.color.as_deref().unwrap_or("#555");

    format!(
        r#"<div style="width:100%; font-size:{font_size}; color:{color}; display:flex; justify-content:space-between; align-items:center; padding:4px 0.75in 4px 0.75in; font-family:Helvetica,Arial,sans-serif;">
  <span>{left}</span>
  <span>{center}</span>
  <span>{right}</span>
</div>"#,
        font_size = font_size,
        color = color,
        left = left,
        center = center,
        right = right,
    )
}

// ─────────────────────────────────────────────────────────────────────────────
// Footer
// ─────────────────────────────────────────────────────────────────────────────

/// Builds the Chrome PDF footer template.
///
/// Priority:
///   1. `footer.custom_html` → returned as-is.
///   2. Structured `left` / `center` / `right` → three-column flex layout.
///   3. Nothing configured → simple centred page number.
///
/// Inside any text field or custom HTML you can use Chrome placeholders:
///   - `<span class="pageNumber"></span>` — current page
///   - `<span class="totalPages"></span>` — total page count
pub fn build_footer_template(config: &PdfConfig) -> String {
    if !config.display_header_footer {
        return "<span></span>".into();
    }

    // Custom HTML takes full control
    if let Some(ref html) = config.footer.custom_html {
        return html.clone();
    }

    let f = &config.footer;
    let font_size = f.font_size.as_deref().unwrap_or("8px");
    let color = f.color.as_deref().unwrap_or("#555");

    let left = f.left.as_deref().unwrap_or("");
    let center = f.center.as_deref().unwrap_or("");
    let right = f.right.as_deref().unwrap_or("");

    // If user configured at least one field, use their layout
    if !left.is_empty() || !center.is_empty() || !right.is_empty() {
        return format!(
            r#"<div style="width:100%; font-size:{font_size}; color:{color}; display:flex; justify-content:space-between; align-items:center; padding:4px 0.75in; font-family:Helvetica,Arial,sans-serif;">
  <span>{left}</span>
  <span>{center}</span>
  <span>{right}</span>
</div>"#,
            font_size = font_size,
            color = color,
            left = left,
            center = center,
            right = right,
        );
    }

    // Default: simple centred page number
    format!(
        r#"<div style="width:100%; font-size:{font_size}; color:{color}; text-align:center; padding:4px 0.75in; font-family:Helvetica,Arial,sans-serif;">
  Page <span class="pageNumber"></span> / <span class="totalPages"></span>
</div>"#,
        font_size = font_size,
        color = color,
    )
}
