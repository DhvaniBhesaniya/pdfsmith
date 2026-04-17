// ─────────────────────────────────────────────────────────────────────────────
// renderer/html.rs — Wraps body HTML into a complete HTML document
// ─────────────────────────────────────────────────────────────────────────────

use crate::config::PdfConfig;
use crate::css::DEFAULT_CSS;

/// CSS counter rules for hierarchical heading numbering (1, 1.1, 1.1.1, …).
const HEADING_NUMBER_CSS: &str = r#"
/* ── Heading Numbers (CSS Counters) ── */
body { counter-reset: h1c h2c h3c h4c h5c h6c; }
h1 { counter-increment: h1c; counter-reset: h2c h3c h4c h5c h6c; }
h1::before { content: counter(h1c) ". "; }
h2 { counter-increment: h2c; counter-reset: h3c h4c h5c h6c; }
h2::before { content: counter(h1c) "." counter(h2c) " "; }
h3 { counter-increment: h3c; counter-reset: h4c h5c h6c; }
h3::before { content: counter(h1c) "." counter(h2c) "." counter(h3c) " "; }
h4 { counter-increment: h4c; counter-reset: h5c h6c; }
h4::before { content: counter(h1c) "." counter(h2c) "." counter(h3c) "." counter(h4c) " "; }
h5 { counter-increment: h5c; counter-reset: h6c; }
h5::before { content: counter(h1c) "." counter(h2c) "." counter(h3c) "." counter(h4c) "." counter(h5c) " "; }
h6 { counter-increment: h6c; }
h6::before { content: counter(h1c) "." counter(h2c) "." counter(h3c) "." counter(h4c) "." counter(h5c) "." counter(h6c) " "; }
"#;

/// Resolves which CSS to use: custom_css replaces default, extra_css appends,
/// heading_numbers injects CSS counters.
pub fn resolve_css(config: &PdfConfig) -> String {
    let base = match &config.custom_css {
        Some(css) => css.clone(),
        None => DEFAULT_CSS.to_string(),
    };

    let with_extra = match &config.extra_css {
        Some(extra) => format!("{}\n\n/* ── Extra CSS ── */\n{}", base, extra),
        None => base,
    };

    if config.heading_numbers {
        format!("{}\n{}", with_extra, HEADING_NUMBER_CSS)
    } else {
        with_extra
    }
}

/// Wraps the generated body HTML in a complete `<!DOCTYPE html>` document
/// with the resolved CSS stylesheet.
pub fn wrap_body_in_html_document(body_html: &str, config: &PdfConfig) -> String {
    let css = resolve_css(config);
    let title = &config.title;

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>{title}</title>
  <style>{css}</style>
</head>
<body>
{body}
</body>
</html>"#,
        title = title,
        css = css,
        body = body_html
    )
}
