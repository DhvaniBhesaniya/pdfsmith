// ─────────────────────────────────────────────────────────────────────────────
// parser/mod.rs — Input parsing: Markdown and JSON → HTML body
// ─────────────────────────────────────────────────────────────────────────────

pub mod json;
pub mod markdown;

pub use json::json_to_markdown;
pub use markdown::markdown_to_body_html;
