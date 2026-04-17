// ─────────────────────────────────────────────────────────────────────────────
// error.rs — Unified error types for pdfsmith
// ─────────────────────────────────────────────────────────────────────────────

use thiserror::Error;

/// All errors that can occur during PDF generation.
#[derive(Debug, Error)]
pub enum MdocError {
    /// Failed to download a remote image (e.g. header logo).
    #[error("Image download failed for '{url}': {reason}")]
    ImageDownload { url: String, reason: String },

    /// Headless Chrome could not be launched or failed during rendering.
    #[error("Chrome error: {0}")]
    Chrome(String),

    /// File I/O error (reading input, writing temp files, etc.).
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON parsing / structure error.
    #[error("JSON error: {0}")]
    Json(String),

    /// Any other unclassified error.
    #[error("{0}")]
    Other(String),
}

/// Convenience alias used throughout the crate.
pub type Result<T> = std::result::Result<T, MdocError>;
