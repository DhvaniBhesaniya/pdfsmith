// ─────────────────────────────────────────────────────────────────────────────
// renderer/mod.rs — HTML assembly + Chrome PDF rendering
// ─────────────────────────────────────────────────────────────────────────────

pub mod chrome;
pub mod html;
pub mod template;

pub use chrome::render_html_to_pdf;
pub use html::wrap_body_in_html_document;
pub use template::{build_footer_template, build_header_template};
