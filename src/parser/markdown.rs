// ─────────────────────────────────────────────────────────────────────────────
// parser/markdown.rs — Markdown → HTML body conversion using comrak
// ─────────────────────────────────────────────────────────────────────────────

use comrak::{markdown_to_html, Options};

use crate::config::{MarkdownOptions, PdfConfig};
use crate::error::Result;

/// Builds comrak `Options` from our `MarkdownOptions`.
fn build_comrak_options(md_opts: &MarkdownOptions) -> Options {
    let mut opts = Options::default();
    opts.render.r#unsafe = md_opts.unsafe_html;
    opts.extension.table = md_opts.tables;
    opts.extension.footnotes = md_opts.footnotes;
    opts.extension.description_lists = md_opts.description_lists;
    opts.extension.strikethrough = md_opts.strikethrough;
    opts.extension.tasklist = md_opts.tasklist;
    opts.extension.autolink = md_opts.autolink;
    opts.extension.superscript = md_opts.superscript;
    opts
}

/// Converts raw Markdown text into an HTML body fragment.
///
/// This function only produces the inner `<body>` content — wrapping in a
/// full HTML document (with CSS, head, etc.) is handled by the renderer.
///
/// The Markdown extensions enabled are controlled by `config.markdown_options`.
pub fn markdown_to_body_html(md: &str, config: &PdfConfig) -> Result<String> {
    let comrak_opts = build_comrak_options(&config.markdown_options);
    let html = markdown_to_html(md, &comrak_opts);
    Ok(html)
}
