// ─────────────────────────────────────────────────────────────────────────────
// css.rs — Clean, neutral default stylesheet
//
// Used when the user does not supply `custom_css`.
// Intentionally minimal and professional — no brand colours, no opinions.
// Users override with `custom_css` (full replace) or `extra_css` (append).
// ─────────────────────────────────────────────────────────────────────────────

/// The built-in default CSS stylesheet.
///
/// A clean, readable, print-friendly stylesheet.  Uses system font stacks
/// and neutral colours so it looks good out of the box for any document.
pub const DEFAULT_CSS: &str = r#"
* { box-sizing: border-box; }

body {
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif;
  font-size: 10pt;
  line-height: 1.5;
  color: #24292e;
  background: #fff;
  margin: 0;
  padding: 0;
  word-wrap: break-word;
}

/* ── Headings ── */
h1, h2, h3, h4, h5, h6 {
  margin-top: 1.2em;
  margin-bottom: 0.5em;
  font-weight: 600;
  line-height: 1.25;
  color: #1a1a1a;
}
h1 { font-size: 2em;    margin-top: 0; padding-bottom: 0.3em; border-bottom: 1px solid #eee; }
h2 { font-size: 1.5em;  padding-bottom: 0.3em; border-bottom: 1px solid #eee; }
h3 { font-size: 1.25em; }
h4 { font-size: 1em;    }
h5 { font-size: 0.875em; }
h6 { font-size: 0.85em; color: #555; }

/* ── Paragraphs & blocks ── */
p, blockquote, ul, ol, dl, table, pre, details {
  margin-top: 0;
  margin-bottom: 1em;
}

/* ── Links ── */
a       { color: #0366d6; text-decoration: none; }
a:hover { text-decoration: underline; }

/* ── Inline code ── */
code, tt {
  font-family: "SFMono-Regular", Consolas, "Liberation Mono", Menlo, monospace;
  font-size: 85%;
  background: #f6f8fa;
  border-radius: 3px;
  padding: 0.2em 0.4em;
}

/* ── Code blocks ── */
pre {
  font-family: "SFMono-Regular", Consolas, "Liberation Mono", Menlo, monospace;
  font-size: 85%;
  background: #f6f8fa;
  border-radius: 6px;
  padding: 16px;
  overflow-x: auto;
  line-height: 1.45;
}
pre code {
  background: transparent;
  padding: 0;
  border: 0;
  font-size: 100%;
}

/* ── Blockquote ── */
blockquote {
  margin: 0 0 1em 0;
  padding: 0 1em;
  color: #6a737d;
  border-left: 0.25em solid #dfe2e5;
}

/* ── Tables ── */
table {
  border-collapse: collapse;
  width: 100%;
  margin-bottom: 1em;
}
table th, table td {
  padding: 8px 12px;
  border: 1px solid #dfe2e5;
}
table th {
  font-weight: 600;
  background: #f6f8fa;
}
table tr:nth-child(even) {
  background: #f9f9f9;
}

/* ── Images ── */
img { max-width: 100%; }

/* ── Lists ── */
ul, ol { padding-left: 2em; }
li { margin-top: 0.25em; }

/* ── Horizontal rule ── */
hr {
  height: 1px;
  margin: 1.5em 0;
  padding: 0;
  background: #e1e4e8;
  border: 0;
}

/* ── Task list ── */
.task-list-item          { list-style-type: none; }
.task-list-item-checkbox { margin: 0 0.2em 0.25em -1.4em; vertical-align: middle; }

/* ── kbd ── */
kbd {
  display: inline-block;
  padding: 3px 5px;
  font: 11px "SFMono-Regular", Consolas, monospace;
  color: #444;
  background: #fafbfc;
  border: 1px solid #d1d5da;
  border-radius: 3px;
  box-shadow: inset 0 -1px 0 #d1d5da;
}

/* ── Strikethrough ── */
del { color: #6a737d; }

/* ── Footnotes ── */
.footnotes    { font-size: 85%; color: #6a737d; }
.footnotes ol { padding-left: 1.5em; }
"#;
