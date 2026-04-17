# pdfsmith <img src="assets/pdfsmith.svg" width="80" height="60" alt="pdfsmith">

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Crates.io](https://img.shields.io/crates/v/pdfsmith.svg?style=for-the-badge&logo=rust&logoColor=white)](https://crates.io/crates/pdfsmith)
[![Docs.rs](https://img.shields.io/docsrs/pdfsmith?style=for-the-badge&logo=docs.rs&logoColor=white)](https://docs.rs/pdfsmith)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge&logo=mit&logoColor=white)](https://opensource.org/licenses/MIT)

A fully customizable Rust library for generating PDFs from **Markdown**, **structured JSON**, or **raw HTML**! 🚀

Everything is under the user's control — CSS styling, page size, margins, orientation, headers, footers, Markdown extensions, and Chrome rendering options. Supply your own CSS or use the clean built-in default. ✨

- 🚀 Built with Rust
- 📄 Outputs polished PDF documents
- ✨ Supports Markdown, JSON, and HTML input
- 🎨 Fully customizable with CSS
- 📏 Configurable paper sizes and margins
- 🔄 Supports headers, footers, and page numbers
- 🏷️ Markdown extensions galore
- 🖼️ Handles images and tables seamlessly
- ⚡ Fast and efficient PDF generation

---

## Table of Contents

- [pdfsmith ](#pdfsmith-)
  - [Table of Contents](#table-of-contents)
  - [Features](#features)
  - [API Overview](#api-overview)
  - [Examples Summary](#examples-summary)
  - [Prerequisites](#prerequisites)
    - [Install Chromium / Chrome](#install-chromium--chrome)
      - [Ubuntu / Debian](#ubuntu--debian)
      - [Fedora / RHEL](#fedora--rhel)
      - [macOS](#macos)
      - [Windows](#windows)
      - [Docker](#docker)
      - [Verify the Installation](#verify-the-installation)
  - [Installation](#installation)
    - [Optional: Enable Logging](#optional-enable-logging)
  - [Quick Start](#quick-start)
  - [Input Sources](#input-sources)
    - [From Markdown String](#from-markdown-string)
    - [From Markdown File](#from-markdown-file)
    - [From JSON](#from-json)
    - [From JSON File](#from-json-file)
    - [From Raw HTML](#from-raw-html)
  - [JSON Structure Reference](#json-structure-reference)
    - [Block Types](#block-types)
      - [`heading`](#heading)
      - [`paragraph` (alias: `text`)](#paragraph-alias-text)
      - [`code`](#code)
      - [`list`](#list)
      - [`quote` (alias: `blockquote`)](#quote-alias-blockquote)
      - [`table`](#table)
      - [`image` (alias: `img`)](#image-alias-img)
      - [`divider` (alias: `hr`)](#divider-alias-hr)
      - [`html` (alias: `raw`)](#html-alias-raw)
  - [Customization Guide](#customization-guide)
    - [CSS Styling](#css-styling)
      - [Option 1: Use the built-in default](#option-1-use-the-built-in-default)
      - [Option 2: Replace the default CSS entirely](#option-2-replace-the-default-css-entirely)
      - [Option 3: Append extra CSS to the default](#option-3-append-extra-css-to-the-default)
      - [Option 4: Load CSS from a file](#option-4-load-css-from-a-file)
      - [Access the default CSS](#access-the-default-css)
    - [Paper Size](#paper-size)
    - [Page Margins](#page-margins)
    - [Orientation](#orientation)
    - [Headers](#headers)
      - [Simple left / center / right](#simple-left--center--right)
      - [Full custom HTML](#full-custom-html)
      - [No header (default)](#no-header-default)
    - [Footers](#footers)
      - [Default footer (when enabled)](#default-footer-when-enabled)
      - [Custom left / center / right](#custom-left--center--right)
      - [Full custom HTML footer](#full-custom-html-footer)
      - [Chrome Page Number Placeholders](#chrome-page-number-placeholders)
    - [Markdown Extensions](#markdown-extensions)
    - [Heading Numbers](#heading-numbers)
      - [Approach 1: JSON `auto_number` (JSON input only)](#approach-1-json-auto_number-json-input-only)
      - [Manual `number` field](#manual-number-field)
      - [Approach 2: CSS-based `heading_numbers` (works with ALL input types)](#approach-2-css-based-heading_numbers-works-with-all-input-types)
    - [Chrome Options](#chrome-options)
  - [Full Configuration Reference](#full-configuration-reference)
    - [PdfConfig Fields](#pdfconfig-fields)
  - [Examples](#examples)
    - [Example 1 — Minimal Markdown](#example-1--minimal-markdown)
    - [Example 2 — JSON with paraSequence](#example-2--json-with-parasequence)
    - [Example 3 — Custom CSS](#example-3--custom-css)
    - [Example 4 — Full Document](#example-4--full-document)
    - [Example 5 — Corporate Report](#example-5--corporate-report)
    - [Example 6 — Images](#example-6--images)
    - [Example 7 — Newsletter](#example-7--newsletter)
    - [Example 8 — Technical Doc (JSON)](#example-8--technical-doc-json)
  - [Running the Examples](#running-the-examples)
  - [Error Handling](#error-handling)
    - [Error Variants](#error-variants)
  - [Architecture](#architecture)
    - [Pipeline](#pipeline)
  - [Troubleshooting](#troubleshooting)
    - ["Chrome error: Failed to launch Chrome/Chromium"](#chrome-error-failed-to-launch-chromechromium)
    - [PDF is blank](#pdf-is-blank)
    - [Header/footer not showing](#headerfooter-not-showing)
    - [Custom CSS not applied](#custom-css-not-applied)
    - [Docker / CI: "No usable sandbox"](#docker--ci-no-usable-sandbox)
  - [License](#license)
  - [Table of Contents](#table-of-contents-1)
  - [Features](#features-1)
  - [API Overview](#api-overview-1)
  - [Examples Summary](#examples-summary-1)
  - [Prerequisites](#prerequisites-1)
    - [Install Chromium / Chrome](#install-chromium--chrome-1)
      - [Ubuntu / Debian](#ubuntu--debian-1)
      - [Fedora / RHEL](#fedora--rhel-1)
      - [macOS](#macos-1)
      - [Windows](#windows-1)
      - [Docker](#docker-1)
      - [Verify the Installation](#verify-the-installation-1)
  - [Installation](#installation-1)
    - [Optional: Enable Logging](#optional-enable-logging-1)
  - [Quick Start](#quick-start-1)
  - [Input Sources](#input-sources-1)
    - [From Markdown String](#from-markdown-string-1)
    - [From Markdown File](#from-markdown-file-1)
    - [From JSON](#from-json-1)
    - [From JSON File](#from-json-file-1)
    - [From Raw HTML](#from-raw-html-1)
  - [JSON Structure Reference](#json-structure-reference-1)
    - [Block Types](#block-types-1)
      - [`heading`](#heading-1)
      - [`paragraph` (alias: `text`)](#paragraph-alias-text-1)
      - [`code`](#code-1)
      - [`list`](#list-1)
      - [`quote` (alias: `blockquote`)](#quote-alias-blockquote-1)
      - [`table`](#table-1)
      - [`image` (alias: `img`)](#image-alias-img-1)
      - [`divider` (alias: `hr`)](#divider-alias-hr-1)
      - [`html` (alias: `raw`)](#html-alias-raw-1)
  - [Customization Guide](#customization-guide-1)
    - [CSS Styling](#css-styling-1)
      - [Option 1: Use the built-in default](#option-1-use-the-built-in-default-1)
      - [Option 2: Replace the default CSS entirely](#option-2-replace-the-default-css-entirely-1)
      - [Option 3: Append extra CSS to the default](#option-3-append-extra-css-to-the-default-1)
      - [Option 4: Load CSS from a file](#option-4-load-css-from-a-file-1)
      - [Access the default CSS](#access-the-default-css-1)
    - [Paper Size](#paper-size-1)
    - [Page Margins](#page-margins-1)
    - [Orientation](#orientation-1)
    - [Headers](#headers-1)
      - [Simple left / center / right](#simple-left--center--right-1)
      - [Full custom HTML](#full-custom-html-1)
      - [No header (default)](#no-header-default-1)
    - [Footers](#footers-1)
      - [Default footer (when enabled)](#default-footer-when-enabled-1)
      - [Custom left / center / right](#custom-left--center--right-1)
      - [Full custom HTML footer](#full-custom-html-footer-1)
      - [Chrome Page Number Placeholders](#chrome-page-number-placeholders-1)
    - [Markdown Extensions](#markdown-extensions-1)
    - [Heading Numbers](#heading-numbers-1)
      - [Approach 1: JSON `auto_number` (JSON input only)](#approach-1-json-auto_number-json-input-only-1)
      - [Manual `number` field](#manual-number-field-1)
      - [Approach 2: CSS-based `heading_numbers` (works with ALL input types)](#approach-2-css-based-heading_numbers-works-with-all-input-types-1)
    - [Chrome Options](#chrome-options-1)
  - [Full Configuration Reference](#full-configuration-reference-1)
    - [PdfConfig Fields](#pdfconfig-fields-1)
  - [Examples](#examples-1)
    - [Example 1 — Minimal Markdown](#example-1--minimal-markdown-1)
    - [Example 2 — JSON with paraSequence](#example-2--json-with-parasequence-1)
    - [Example 3 — Custom CSS](#example-3--custom-css-1)
    - [Example 4 — Full Document](#example-4--full-document-1)
    - [Example 5 — Corporate Report](#example-5--corporate-report-1)
    - [Example 6 — Images](#example-6--images-1)
    - [Example 7 — Newsletter](#example-7--newsletter-1)
    - [Example 8 — Technical Doc (JSON)](#example-8--technical-doc-json-1)
  - [Running the Examples](#running-the-examples-1)
  - [Error Handling](#error-handling-1)
    - [Error Variants](#error-variants-1)
  - [Architecture](#architecture-1)
    - [Pipeline](#pipeline-1)
  - [Troubleshooting](#troubleshooting-1)
    - ["Chrome error: Failed to launch Chrome/Chromium"](#chrome-error-failed-to-launch-chromechromium-1)
    - [PDF is blank](#pdf-is-blank-1)
    - [Header/footer not showing](#headerfooter-not-showing-1)
    - [Custom CSS not applied](#custom-css-not-applied-1)
    - [Docker / CI: "No usable sandbox"](#docker--ci-no-usable-sandbox-1)
  - [License](#license-1)

---

## Features

| Feature | Description |
|---|---|
| **Markdown → PDF** | Convert any Markdown string or file to a styled PDF 📝➡️📄 |
| **JSON → PDF** | Define document content with typed JSON blocks (headings, paragraphs, tables, code, lists, etc.) 📋➡️📄 |
| **HTML → PDF** | Pass your own pre-built HTML directly 🌐➡️📄 |
| **Custom CSS** | Replace the default stylesheet entirely, or append extra rules 🎨 |
| **Paper Sizes** | A4, Letter, Legal, or any custom width × height 📏 |
| **Page Margins** | Set top, bottom, left, right margins independently (in inches) 📐 |
| **Orientation** | Portrait or landscape 🔄 |
| **Headers** | Simple left/center/right text, or full custom HTML 📄 |
| **Footers** | Simple left/center/right text with auto page numbers, or full custom HTML 📄 |
| **Heading Numbers** | Automatic hierarchical numbering (1, 1.1, 1.1.1) via JSON `paraSequence` or CSS counters 🔢 |
| **Markdown Extensions** | Tables, footnotes, strikethrough, task lists, autolinks, superscript, description lists — all toggleable 📝 |
| **File Input** | Read `.md` or `.json` files directly from disk 📁 |
| **Builder Pattern** | Fluent, chainable API — configure only what you need 🛠️ |

---

## API Overview

- `PdfBuilder` — main fluent builder for generating PDFs from Markdown, JSON, or HTML. 🏗️
- `PdfConfig` — full PDF generation configuration. ⚙️
- `PaperSize`, `PageMargins`, `HeaderConfig`, `FooterConfig`, `MarkdownOptions` — core configuration helpers. 🧰
- `DEFAULT_CSS` — inspect or reuse the built-in stylesheet. 🎨
- `MdocError` / `Result` — error type and result alias used throughout the crate. ❌

## Examples Summary

- Minimal Markdown PDF generation 📝
- JSON document generation with `paraSequence` headings 📋
- Custom CSS styling and theming 🎨
- Full document generation with headers, footers, and page options 📄
- Report-style documents, newsletters, technical docs, and images 📰
- And much more! 🌟

---

## Prerequisites

### Install Chromium / Chrome

`pdfsmith` uses **headless Chrome/Chromium** under the hood to render HTML to PDF. You must have Chrome or Chromium installed on any machine that generates PDFs. 🌐

#### Ubuntu / Debian

```bash
# Option A: via snap (recommended)
sudo snap install chromium

# Option B: via apt
sudo apt update
sudo apt install -y chromium-browser
```

#### Fedora / RHEL

```bash
sudo dnf install chromium
```

#### macOS

```bash
# If you have Google Chrome installed, it works automatically.
# Otherwise:
brew install --cask chromium
```

#### Windows

Install [Google Chrome](https://www.google.com/chrome/) or [Chromium](https://www.chromium.org/getting-involved/download-chromium). The `headless_chrome` crate will auto-detect it. 🪟

#### Docker

If your app runs in a container, add Chromium to your Dockerfile:

```dockerfile
RUN apt-get update && apt-get install -y chromium-browser --no-install-recommends && rm -rf /var/lib/apt/lists/*
```

> **Why not auto-install?** A library crate should never silently install system software. Chromium is a ~200 MB browser — the user should install it explicitly and control which version is on their system. 🤔

#### Verify the Installation

```bash
chromium --version
# or
chromium-browser --version
# or
google-chrome --version
```

If any of these prints a version number, you're ready. ✅

---

## Installation

Add `pdfsmith` to your `Cargo.toml`:

```toml
[dependencies]
pdfsmith = "0.1.0"
```

Or via the command line:

```bash
cargo add pdfsmith
```

### Optional: Enable Logging

`pdfsmith` uses the [`log`](https://crates.io/crates/log) crate. To see progress messages, add a logger like `env_logger`:

```toml
[dependencies]
env_logger = "0.11"
```

Then initialise it in your code:

```rust
fn main() {
    env_logger::init(); // call once at startup
    // ... your code ...
}
```

Run with `RUST_LOG=info` to see output:

```bash
RUST_LOG=info cargo run
```

---

## Quick Start

```rust
use pdfsmith::PdfBuilder;

fn main() {
    let pdf_bytes = PdfBuilder::new()
        .from_markdown("# Hello World\n\nThis is my first PDF.")
        .expect("PDF generation failed");

    std::fs::write("hello.pdf", &pdf_bytes).expect("Failed to write file");
    println!("PDF saved! ({} bytes)", pdf_bytes.len());
}
```

That's it. Three lines of code, zero configuration. The default CSS, A4 paper, 0.75-inch margins, and no header/footer are applied automatically. 🚀

---

## Input Sources

### From Markdown String

```rust
use pdfsmith::PdfBuilder;

let pdf = PdfBuilder::new()
    .from_markdown("# Title\n\nSome **bold** and *italic* text.")
    .unwrap();
```

All standard Markdown is supported: headings, bold, italic, links, images, code blocks, tables, task lists, blockquotes, horizontal rules, footnotes, strikethrough, and more. 📝

### From Markdown File

```rust
use pdfsmith::PdfBuilder;

let pdf = PdfBuilder::new()
    .from_markdown_file("docs/report.md")
    .unwrap();
```

### From JSON

```rust
use pdfsmith::PdfBuilder;

let json = serde_json::json!([
    { "type": "heading", "paraSequence": "1", "text": "Report Title" },
    { "type": "paragraph", "text": "Some content with **Markdown** formatting." },
    { "type": "heading", "paraSequence": "1.1", "text": "Details" },
    { "type": "paragraph", "text": "The heading level is derived from paraSequence depth." }
]);

let pdf = PdfBuilder::new()
    .from_json(&json)
    .unwrap();
```

JSON is first converted to Markdown, then rendered through the same pipeline — so all CSS and config applies equally. 📋

### From JSON File

```rust
use pdfsmith::PdfBuilder;

let pdf = PdfBuilder::new()
    .from_json_file("data/document.json")
    .unwrap();
```

### From Raw HTML

```rust
use pdfsmith::PdfBuilder;

let html = r#"
<!DOCTYPE html>
<html>
<head><style>body { font-family: Arial; }</style></head>
<body><h1>Hello</h1><p>From raw HTML.</p></body>
</html>
"#;

let pdf = PdfBuilder::new()
    .from_html(html)
    .unwrap();
```

When using `from_html`, the HTML is sent to Chrome **as-is** — no CSS injection, no Markdown conversion. Header/footer and page options still apply. 🌐

---

## JSON Structure Reference

The JSON input is an array of **content blocks**. Each block has a `"type"` field that determines how it's rendered.

Two top-level formats are accepted:

```jsonc
// Format A: bare array
[
  { "type": "heading", "paraSequence": "1", "text": "Title" },
  { "type": "paragraph", "text": "Body." },
  { "type": "heading", "paraSequence": "1.1", "text": "Sub-section" },
  { "type": "paragraph", "text": "More body." }
]

// Format B: object with "content" key
{
  "content": [
    { "type": "heading", "paraSequence": "1", "text": "Title" },
    { "type": "paragraph", "text": "Body." }
  ]
}
```

### Block Types

#### `heading`

```json
{ "type": "heading", "paraSequence": "1.2", "text": "Section Title" }
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `paraSequence` | string | No | Hierarchical section number (e.g. `"1"`, `"1.2"`, `"1.2.1"`, `"A.1"`). The heading level (h1–h6) is **derived from the depth** (number of dot-separated parts). The sequence is prefixed to the heading text automatically. |
| `text` | string | Yes | Heading text (Markdown allowed) |
| `level` | number (1–6) | No | Explicit heading level — overrides the level derived from `paraSequence`. If neither `paraSequence` nor `level` is given, defaults to h1. |

#### `paragraph` (alias: `text`)

```json
{ "type": "paragraph", "text": "Some **bold** text with [links](https://example.com)." }
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `text` | string | Yes | Paragraph content (Markdown allowed) |

#### `code`

```json
{ "type": "code", "language": "rust", "text": "fn main() {\n    println!(\"Hello\");\n}" }
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `language` | string | No (default: `""`) | Syntax highlighting hint |
| `text` | string | Yes | Code content |

#### `list`

```json
{ "type": "list", "ordered": true, "items": ["First", "Second", "Third"] }
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ordered` | boolean | No (default: `false`) | `true` for numbered list, `false` for bullets |
| `items` | array of strings | Yes | List items (Markdown allowed in each item) |

#### `quote` (alias: `blockquote`)

```json
{ "type": "quote", "text": "To be or not to be." }
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `text` | string | Yes | Quote content (Markdown allowed) |

#### `table`

```json
{
  "type": "table",
  "headers": ["Name", "Role", "Location"],
  "rows": [
    ["Alice", "Engineer", "London"],
    ["Bob", "Designer", "Berlin"]
  ]
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `headers` | array of strings | Yes | Column headers |
| `rows` | array of arrays | No | Data rows (each row is an array of strings) |

#### `image` (alias: `img`)

```json
{ "type": "image", "src": "https://example.com/photo.png", "alt": "A photo" }
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `src` | string | Yes | Image URL or path |
| `alt` | string | No (default: `""`) | Alt text |

#### `divider` (alias: `hr`)

```json
{ "type": "divider" }
```

Renders a horizontal rule (`---`). No fields needed.

#### `html` (alias: `raw`)

```json
{ "type": "html", "text": "<div style='color:red;'>Custom HTML</div>" }
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `text` | string | Yes | Raw HTML passed through as-is |

---

## Customization Guide

Every aspect of the PDF is configurable through `PdfBuilder` methods or the `PdfConfig` struct directly. 🎛️

### CSS Styling

#### Option 1: Use the built-in default

Do nothing — a clean, neutral, print-friendly stylesheet is applied automatically. 🎨

```rust
let pdf = PdfBuilder::new()
    .from_markdown("# Uses default CSS")
    .unwrap();
```

#### Option 2: Replace the default CSS entirely

```rust
let pdf = PdfBuilder::new()
    .custom_css(r#"
        body { font-family: Georgia, serif; font-size: 12pt; color: #333; }
        h1 { color: navy; border-bottom: 2px solid navy; }
        code { background: #f0f0f0; padding: 2px 4px; }
        table { border-collapse: collapse; width: 100%; }
        th, td { border: 1px solid #ccc; padding: 8px; }
    "#)
    .from_markdown("# Fully custom styled")
    .unwrap();
```

When `custom_css` is set, the built-in stylesheet is **completely replaced**. You are in full control. 🎨

#### Option 3: Append extra CSS to the default

```rust
let pdf = PdfBuilder::new()
    .extra_css("h1 { color: darkred; } blockquote { border-color: blue; }")
    .from_markdown("# Default + tweaks")
    .unwrap();
```

`extra_css` is added **after** the default stylesheet, so your rules override specific properties while keeping everything else intact. 🎨

#### Option 4: Load CSS from a file

```rust
let css = std::fs::read_to_string("styles/my-theme.css").unwrap();
let pdf = PdfBuilder::new()
    .custom_css(css)
    .from_markdown("# Themed")
    .unwrap();
```

#### Access the default CSS

The built-in stylesheet is exported as a constant if you want to inspect or extend it programmatically:

```rust
use pdfsmith::DEFAULT_CSS;
println!("{}", DEFAULT_CSS);
```

### Paper Size

```rust
use pdfsmith::{PdfBuilder, PaperSize};

// Preset sizes
PdfBuilder::new().paper_size(PaperSize::A4);      // 8.27 × 11.69 in (default)
PdfBuilder::new().paper_size(PaperSize::Letter);   // 8.5  × 11    in
PdfBuilder::new().paper_size(PaperSize::Legal);    // 8.5  × 14    in

// Custom size (width × height in inches)
PdfBuilder::new().paper_size(PaperSize::Custom { width: 6.0, height: 9.0 }); // e.g. trade paperback
```

### Page Margins

Margins are specified in inches:

```rust
use pdfsmith::{PdfBuilder, PageMargins};

let pdf = PdfBuilder::new()
    .margins(PageMargins {
        top: 1.0,
        bottom: 1.0,
        left: 0.5,
        right: 0.5,
    })
    .from_markdown("# Custom margins")
    .unwrap();
```

Default margins are `0.75` inches on all sides. 📐

### Orientation

```rust
let pdf = PdfBuilder::new()
    .landscape(true)   // landscape
    .from_markdown("# Wide layout")
    .unwrap();
```

Default is `false` (portrait). 🔄

### Headers

Headers appear on every page. Three approaches: 📄

#### Simple left / center / right

```rust
use pdfsmith::{PdfBuilder, HeaderConfig};

let pdf = PdfBuilder::new()
    .display_header_footer(true)
    .header(HeaderConfig {
        left: Some("Company Name".into()),
        center: Some("Document Title".into()),
        right: Some("2026-04-17".into()),
        font_size: Some("9px".into()),   // optional, default: "9px"
        color: Some("#333".into()),       // optional, default: "#555"
        ..Default::default()
    })
    .from_markdown("# With header")
    .unwrap();
```

#### Full custom HTML

```rust
use pdfsmith::{PdfBuilder, HeaderConfig};

let pdf = PdfBuilder::new()
    .display_header_footer(true)
    .header(HeaderConfig {
        custom_html: Some(r#"
            <div style="width:100%; text-align:center; font-size:10px; color:#999; padding:4px;">
                My Custom Header — Page <span class="pageNumber"></span>
            </div>
        "#.into()),
        ..Default::default()
    })
    .from_markdown("# With custom header HTML")
    .unwrap();
```

#### No header (default)

```rust
let pdf = PdfBuilder::new()
    .from_markdown("# No header")   // display_header_footer defaults to false
    .unwrap();
```

### Footers

Footers work exactly like headers. 📄

#### Default footer (when enabled)

When `display_header_footer(true)` is set and no footer fields are configured, a simple centred page number is shown: `Page 1 / 5`.

```rust
let pdf = PdfBuilder::new()
    .display_header_footer(true)
    .from_markdown("# With page numbers")
    .unwrap();
```

#### Custom left / center / right

```rust
use pdfsmith::{PdfBuilder, FooterConfig};

let pdf = PdfBuilder::new()
    .display_header_footer(true)
    .footer(FooterConfig {
        left: Some("CONFIDENTIAL".into()),
        center: Some("Internal Use Only".into()),
        right: Some(r#"Page <span class="pageNumber"></span> of <span class="totalPages"></span>"#.into()),
        font_size: Some("8px".into()),
        color: Some("#888".into()),
        ..Default::default()
    })
    .from_markdown("# With footer")
    .unwrap();
```

#### Full custom HTML footer

```rust
use pdfsmith::{PdfBuilder, FooterConfig};

let pdf = PdfBuilder::new()
    .display_header_footer(true)
    .footer(FooterConfig {
        custom_html: Some(r#"
            <div style="width:100%; display:flex; justify-content:space-between; padding:4px 0.75in; font-size:8px; color:#666; font-family:Arial;">
                <span>© 2026 Acme Corp</span>
                <span>Page <span class="pageNumber"></span> / <span class="totalPages"></span></span>
            </div>
        "#.into()),
        ..Default::default()
    })
    .from_markdown("# Custom footer HTML")
    .unwrap();
```

#### Chrome Page Number Placeholders

Inside any header/footer text or `custom_html`, use these Chrome built-in placeholders:

| Placeholder | Renders as |
|---|---|
| `<span class="pageNumber"></span>` | Current page number |
| `<span class="totalPages"></span>` | Total page count |
| `<span class="title"></span>` | Document title |
| `<span class="url"></span>` | Page URL |
| `<span class="date"></span>` | Current date |

### Markdown Extensions

All extensions are enabled by default. Toggle them individually: 📝

```rust
use pdfsmith::{PdfBuilder, MarkdownOptions};

let pdf = PdfBuilder::new()
    .markdown_options(MarkdownOptions {
        tables: true,              // | col1 | col2 |
        footnotes: true,           // [^1]: footnote text
        strikethrough: true,       // ~~deleted~~
        tasklist: true,            // - [x] done
        autolink: true,            // https://auto.link
        superscript: true,         // 2^10
        description_lists: true,   // term\n: definition
        unsafe_html: true,         // pass-through raw <html> tags
    })
    .from_markdown("# All extensions on")
    .unwrap();
```

To disable a specific extension:

```rust
let pdf = PdfBuilder::new()
    .markdown_options(MarkdownOptions {
        tasklist: false,       // disable task lists
        superscript: false,    // disable superscript
        ..Default::default()   // keep everything else on
    })
    .from_markdown("# Some extensions off")
    .unwrap();
```

### Heading Numbers

Automatic hierarchical heading numbers like **1**, **1.1**, **1.1.1**, **2**, **2.1**. 🔢

There are **two approaches** — use whichever fits your workflow:

#### Approach 1: JSON `auto_number` (JSON input only)

Set `"auto_number": true` in the top-level JSON object. Numbers are generated
automatically from the heading `level` fields:

```rust
use pdfsmith::PdfBuilder;

let json = serde_json::json!({
    "auto_number": true,
    "content": [
        { "type": "heading", "level": 1, "text": "Introduction" },
        { "type": "paragraph", "text": "Intro body." },
        { "type": "heading", "level": 2, "text": "Background" },
        { "type": "heading", "level": 2, "text": "Scope" },
        { "type": "heading", "level": 3, "text": "Details" },
        { "type": "heading", "level": 1, "text": "Conclusion" }
    ]
});

let pdf = PdfBuilder::new()
    .from_json(&json)
    .unwrap();
```

This produces headings:

```
1 Introduction
1.1 Background
1.2 Scope
1.2.1 Details
2 Conclusion
```

#### Manual `number` field

Any heading block can carry a `"number"` field for manual control.  This
takes priority over auto-numbering:

```json
[
  { "type": "heading", "level": 1, "number": "A", "text": "Appendix" },
  { "type": "heading", "level": 2, "number": "A.1", "text": "First Section" },
  { "type": "heading", "level": 2, "number": "A.2", "text": "Second Section" }
]
```

#### Approach 2: CSS-based `heading_numbers` (works with ALL input types)

Enable `.heading_numbers(true)` on the builder. This injects CSS counter
rules that automatically number every heading — works for Markdown, JSON,
and raw HTML:

```rust
use pdfsmith::PdfBuilder;

let pdf = PdfBuilder::new()
    .heading_numbers(true)
    .from_markdown("# Intro\n\n## Part A\n\n## Part B\n\n### Detail\n\n# Summary")
    .unwrap();
```

Rendered headings look like:

```
1. Intro
1.1 Part A
1.2 Part B
1.2.1 Detail
2. Summary
```

The CSS approach has no dependency on JSON — it works everywhere.

> **Which should I use?**
> - Use **`auto_number`** in JSON when you want the numbers baked into
>   the Markdown text (good for exports, copy-paste).
> - Use **`heading_numbers(true)`** on the builder when you want visual
>   numbering via CSS (works with any input, numbers don’t appear in
>   the underlying text).

### Chrome Options

Fine-tune the headless Chrome rendering: 🌐

```rust
let pdf = PdfBuilder::new()
    .chrome_window_size(1920, 1080)   // browser viewport (default: 1280×900)
    .page_load_wait_secs(5)           // wait for images/JS (default: 2)
    .print_background(true)           // render background colours (default: true)
    .from_markdown("# Chrome settings")
    .unwrap();
```

---

## Full Configuration Reference

You can also set everything at once via the `PdfConfig` struct:

```rust
use pdfsmith::{PdfBuilder, PdfConfig, PaperSize, PageMargins, HeaderConfig, FooterConfig, MarkdownOptions};

let config = PdfConfig {
    title: "My Document".into(),
    custom_css: None,                                  // use built-in default
    extra_css: Some("h1 { color: navy; }".into()),     // tweak headings
    paper_size: PaperSize::A4,
    margins: PageMargins { top: 1.0, bottom: 1.0, left: 0.75, right: 0.75 },
    landscape: false,
    display_header_footer: true,
    header: HeaderConfig {
        left: Some("Acme Corp".into()),
        center: Some("Report".into()),
        right: Some("April 2026".into()),
        font_size: Some("9px".into()),
        color: Some("#333".into()),
        custom_html: None,
    },
    footer: FooterConfig {
        left: None,
        center: None,
        right: Some(r#"Page <span class="pageNumber"></span>"#.into()),
        font_size: Some("8px".into()),
        color: Some("#888".into()),
        custom_html: None,
    },
    print_background: true,
    heading_numbers: false,
    markdown_options: MarkdownOptions::default(),
    chrome_window_size: (1280, 900),
    page_load_wait_secs: 5, // use 5 when any image is available in markdown or json.
};

let pdf = PdfBuilder::with_config(config)
    .from_markdown("# Hello from PdfConfig")
    .unwrap();
```

### PdfConfig Fields

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `title` | `String` | `""` | Document title (used in `<title>` tag and default header) |
| `custom_css` | `Option<String>` | `None` | Replaces the entire default stylesheet |
| `extra_css` | `Option<String>` | `None` | Appended after the base stylesheet |
| `paper_size` | `PaperSize` | `A4` | Paper dimensions |
| `margins` | `PageMargins` | `0.75` all | Page margins in inches |
| `landscape` | `bool` | `false` | Landscape orientation |
| `display_header_footer` | `bool` | `false` | Show header and footer |
| `header` | `HeaderConfig` | empty | Header configuration |
| `footer` | `FooterConfig` | empty | Footer configuration |
| `print_background` | `bool` | `true` | Print background colours/images |
| `markdown_options` | `MarkdownOptions` | all `true` | Markdown extension toggles |
| `heading_numbers` | `bool` | `false` | CSS-based hierarchical heading numbers |
| `chrome_window_size` | `(u32, u32)` | `(1280, 900)` | Headless Chrome viewport |
| `page_load_wait_secs` | `u64` | `5` | Seconds to wait after page load ( Usually helpfull to load images perfectly) |

---

## Examples

The repository includes **eight** runnable examples in the `examples/` directory, each producing a 2+ page PDF. 📁

### Example 1 — Minimal Markdown

Markdown string → PDF with `heading_numbers(true)`. Default CSS, no header/footer. Covers tables, code blocks, lists, blockquotes.

```bash
cargo run --example from_markdown
```

### Example 2 — JSON with paraSequence

Structured JSON content blocks with `"paraSequence"` on every heading for hierarchical numbering. The heading level (h1–h6) is derived automatically from the sequence depth.

```rust
// examples/from_json.rs  (abbreviated)
let json = serde_json::json!([
    { "type": "heading", "paraSequence": "1",     "text": "Project Status Report" },
    { "type": "paragraph", "text": "This report summarises the current state." },
    { "type": "heading", "paraSequence": "1.1",   "text": "Timeline" },
    { "type": "table", "headers": ["Phase","Status","Due"], "rows": [["Design","Complete","2026-01-15"]] },
    { "type": "heading", "paraSequence": "1.2",   "text": "Key Highlights" },
    { "type": "heading", "paraSequence": "1.2.1", "text": "Performance Details" },
    { "type": "heading", "paraSequence": "2",     "text": "Architecture" },
    // ... more blocks ...
]);

let pdf = PdfBuilder::new()
    .from_json(&json)
    .expect("Failed to generate PDF");
```

Output headings:

```
1 Project Status Report       (depth 1 → h1)
  1.1 Timeline                (depth 2 → h2)
  1.2 Key Highlights          (depth 2 → h2)
    1.2.1 Performance Details  (depth 3 → h3)
2 Architecture                (depth 1 → h1)
```

```bash
cargo run --example from_json
```

### Example 3 — Custom CSS

Replaces the entire default stylesheet. Indented sub-headings (h2 → 20px, h3 → 40px, h4 → 60px), per-level colours (blue → green → gold), dark code blocks, `heading_numbers(true)` on top.

```bash
cargo run --example custom_css
```

### Example 4 — Full Document

Every option configured: title, Letter landscape paper, header, footer, `heading_numbers(true)`, extra CSS for colours. Quarterly business review theme.

```bash
cargo run --example full_document
```

### Example 5 — Corporate Report

A4 report with default CSS as the base, small tweaks via `extra_css` (indented headings, corporate colours). Header shows company name + report title, footer shows "INTERNAL" + page numbers.

```bash
cargo run --example report_style
```

### Example 6 — Images

Markdown document with remote images (Wikimedia Commons). Uses `page_load_wait_secs(5)` to allow time for image loading. Header/footer, heading numbers, extra CSS for image styling.

```bash
cargo run --example with_images
```

### Example 7 — Newsletter

Vibrant editorial style with full `custom_css`: gradient heading backgrounds, purple/green/gold colours, styled blockquotes. No heading numbers — clean magazine look. Header + footer.

```bash
cargo run --example newsletter
```

### Example 8 — Technical Doc (JSON)

Complete API reference built entirely from JSON with `paraSequence` on every heading. Uses a `build_content()` helper to construct large JSON arrays. Header/footer, indented headings via `extra_css`.

```bash
cargo run --example technical_doc
```

---

## Running the Examples

```bash
# Clone the repository
git clone github.com/DhvaniBhesaniya/pdfsmith.git
cd pdfsmith

# Run any example
cargo run --example from_markdown
cargo run --example from_json
cargo run --example custom_css
cargo run --example full_document
cargo run --example report_style
cargo run --example with_images
cargo run --example newsletter
cargo run --example technical_doc

# With logging enabled
RUST_LOG=info cargo run --example from_markdown
```

---

## Error Handling

All generation methods return `pdfsmith::Result<Vec<u8>>`, which is `Result<Vec<u8>, MdocError>`.

```rust
use pdfsmith::{PdfBuilder, MdocError};

match PdfBuilder::new().from_markdown("# Test") {
    Ok(pdf) => std::fs::write("out.pdf", pdf).unwrap(),
    Err(MdocError::Chrome(msg)) => eprintln!("Chrome error: {}", msg),
    Err(MdocError::Io(err))     => eprintln!("IO error: {}", err),
    Err(MdocError::Json(msg))   => eprintln!("JSON error: {}", msg),
    Err(e)                      => eprintln!("Error: {}", e),
}
```

### Error Variants

| Variant | When it occurs |
|---|---|
| `MdocError::Chrome(String)` | Chrome/Chromium not found, failed to launch, navigation timeout, print failure |
| `MdocError::Io(std::io::Error)` | File read/write errors |
| `MdocError::Json(String)` | Invalid JSON structure (not an array or missing `"content"` key) |
| `MdocError::ImageDownload { url, reason }` | Remote image fetch failed |
| `MdocError::Other(String)` | Catch-all for uncategorised errors |

---

## Architecture

```
pdfsmith/
├── src/
│   ├── lib.rs              # Public API — PdfBuilder and re-exports
│   ├── config.rs           # PdfConfig, PaperSize, PageMargins, HeaderConfig, FooterConfig, MarkdownOptions
│   ├── css.rs              # DEFAULT_CSS constant
│   ├── error.rs            # MdocError enum and Result type alias
│   ├── parser/
│   │   ├── mod.rs          # Re-exports
│   │   ├── markdown.rs     # Markdown → HTML body (via comrak)
│   │   └── json.rs         # JSON content blocks → Markdown
│   └── renderer/
│       ├── mod.rs          # Re-exports
│       ├── html.rs         # Wraps body HTML in full document with CSS
│       ├── template.rs     # Header/footer template builders
│       └── chrome.rs       # Headless Chrome PDF rendering
├── examples/
│   ├── from_markdown.rs
│   ├── from_json.rs
│   ├── custom_css.rs
│   ├── full_document.rs
│   ├── report_style.rs
│   ├── with_images.rs
│   ├── newsletter.rs
│   └── technical_doc.rs
├── tests/
│   ├── basic.rs
│   └── json.rs
└── Cargo.toml
```

### Pipeline

```
Markdown string ─┐
                  ├─→ HTML body ─→ Full HTML doc (+ CSS) ─→ Chrome ─→ PDF bytes
JSON blocks ─────┘    (comrak)     (wrap_body_in_html)      (headless)
```

1. **Input** — Markdown text, JSON blocks, or raw HTML.
2. **Parse** — Markdown is converted to HTML by comrak. JSON is first converted to Markdown, then through comrak.
3. **Wrap** — The HTML body is wrapped in a full `<!DOCTYPE html>` document with the resolved CSS (default, custom, or default + extra).
4. **Template** — Header and footer HTML templates are built from config.
5. **Render** — Headless Chrome loads the HTML, applies print options (paper, margins, header, footer), and outputs PDF bytes.
6. **Return** — Raw `Vec<u8>` PDF bytes ready to write to disk or send over HTTP.

---

## Troubleshooting

### "Chrome error: Failed to launch Chrome/Chromium"

Chromium is not installed or not in your `PATH`. See [Prerequisites](#prerequisites).

### PDF is blank

- Check that your Markdown/JSON input is not empty.
- Increase `page_load_wait_secs` if your HTML has images that need time to load.

### Header/footer not showing

- You must call `.display_header_footer(true)` — it defaults to `false`.
- Increase `margins.top` / `margins.bottom` to make room for the header/footer.

### Custom CSS not applied

- If you use `custom_css`, it **replaces** the default entirely. Make sure your CSS covers everything (body font, headings, tables, code blocks, etc.).
- If you only want small tweaks, use `extra_css` instead.

### Docker / CI: "No usable sandbox"

Chrome needs a sandbox. In Docker, run with `--no-sandbox`:

```dockerfile
ENV CHROME_FLAGS="--no-sandbox --disable-setuid-sandbox"
```

Or use a container image that includes Chromium with proper sandbox config.

---

## License

MIT

---

## Table of Contents

- [pdfsmith ](#pdfsmith-)
  - [Table of Contents](#table-of-contents)
  - [Features](#features)
  - [API Overview](#api-overview)
  - [Examples Summary](#examples-summary)
  - [Prerequisites](#prerequisites)
    - [Install Chromium / Chrome](#install-chromium--chrome)
      - [Ubuntu / Debian](#ubuntu--debian)
      - [Fedora / RHEL](#fedora--rhel)
      - [macOS](#macos)
      - [Windows](#windows)
      - [Docker](#docker)
      - [Verify the Installation](#verify-the-installation)
  - [Installation](#installation)
    - [Optional: Enable Logging](#optional-enable-logging)
  - [Quick Start](#quick-start)
  - [Input Sources](#input-sources)
    - [From Markdown String](#from-markdown-string)
    - [From Markdown File](#from-markdown-file)
    - [From JSON](#from-json)
    - [From JSON File](#from-json-file)
    - [From Raw HTML](#from-raw-html)
  - [JSON Structure Reference](#json-structure-reference)
    - [Block Types](#block-types)
      - [`heading`](#heading)
      - [`paragraph` (alias: `text`)](#paragraph-alias-text)
      - [`code`](#code)
      - [`list`](#list)
      - [`quote` (alias: `blockquote`)](#quote-alias-blockquote)
      - [`table`](#table)
      - [`image` (alias: `img`)](#image-alias-img)
      - [`divider` (alias: `hr`)](#divider-alias-hr)
      - [`html` (alias: `raw`)](#html-alias-raw)
  - [Customization Guide](#customization-guide)
    - [CSS Styling](#css-styling)
      - [Option 1: Use the built-in default](#option-1-use-the-built-in-default)
      - [Option 2: Replace the default CSS entirely](#option-2-replace-the-default-css-entirely)
      - [Option 3: Append extra CSS to the default](#option-3-append-extra-css-to-the-default)
      - [Option 4: Load CSS from a file](#option-4-load-css-from-a-file)
      - [Access the default CSS](#access-the-default-css)
    - [Paper Size](#paper-size)
    - [Page Margins](#page-margins)
    - [Orientation](#orientation)
    - [Headers](#headers)
      - [Simple left / center / right](#simple-left--center--right)
      - [Full custom HTML](#full-custom-html)
      - [No header (default)](#no-header-default)
    - [Footers](#footers)
      - [Default footer (when enabled)](#default-footer-when-enabled)
      - [Custom left / center / right](#custom-left--center--right)
      - [Full custom HTML footer](#full-custom-html-footer)
      - [Chrome Page Number Placeholders](#chrome-page-number-placeholders)
    - [Markdown Extensions](#markdown-extensions)
    - [Heading Numbers](#heading-numbers)
      - [Approach 1: JSON `auto_number` (JSON input only)](#approach-1-json-auto_number-json-input-only)
      - [Manual `number` field](#manual-number-field)
      - [Approach 2: CSS-based `heading_numbers` (works with ALL input types)](#approach-2-css-based-heading_numbers-works-with-all-input-types)
    - [Chrome Options](#chrome-options)
  - [Full Configuration Reference](#full-configuration-reference)
    - [PdfConfig Fields](#pdfconfig-fields)
  - [Examples](#examples)
    - [Example 1 — Minimal Markdown](#example-1--minimal-markdown)
    - [Example 2 — JSON with paraSequence](#example-2--json-with-parasequence)
    - [Example 3 — Custom CSS](#example-3--custom-css)
    - [Example 4 — Full Document](#example-4--full-document)
    - [Example 5 — Corporate Report](#example-5--corporate-report)
    - [Example 6 — Images](#example-6--images)
    - [Example 7 — Newsletter](#example-7--newsletter)
    - [Example 8 — Technical Doc (JSON)](#example-8--technical-doc-json)
  - [Running the Examples](#running-the-examples)
  - [Error Handling](#error-handling)
    - [Error Variants](#error-variants)
  - [Architecture](#architecture)
    - [Pipeline](#pipeline)
  - [Troubleshooting](#troubleshooting)
    - ["Chrome error: Failed to launch Chrome/Chromium"](#chrome-error-failed-to-launch-chromechromium)
    - [PDF is blank](#pdf-is-blank)
    - [Header/footer not showing](#headerfooter-not-showing)
    - [Custom CSS not applied](#custom-css-not-applied)
    - [Docker / CI: "No usable sandbox"](#docker--ci-no-usable-sandbox)
  - [License](#license)
  - [Table of Contents](#table-of-contents-1)
  - [Features](#features-1)
  - [API Overview](#api-overview-1)
  - [Examples Summary](#examples-summary-1)
  - [Prerequisites](#prerequisites-1)
    - [Install Chromium / Chrome](#install-chromium--chrome-1)
      - [Ubuntu / Debian](#ubuntu--debian-1)
      - [Fedora / RHEL](#fedora--rhel-1)
      - [macOS](#macos-1)
      - [Windows](#windows-1)
      - [Docker](#docker-1)
      - [Verify the Installation](#verify-the-installation-1)
  - [Installation](#installation-1)
    - [Optional: Enable Logging](#optional-enable-logging-1)
  - [Quick Start](#quick-start-1)
  - [Input Sources](#input-sources-1)
    - [From Markdown String](#from-markdown-string-1)
    - [From Markdown File](#from-markdown-file-1)
    - [From JSON](#from-json-1)
    - [From JSON File](#from-json-file-1)
    - [From Raw HTML](#from-raw-html-1)
  - [JSON Structure Reference](#json-structure-reference-1)
    - [Block Types](#block-types-1)
      - [`heading`](#heading-1)
      - [`paragraph` (alias: `text`)](#paragraph-alias-text-1)
      - [`code`](#code-1)
      - [`list`](#list-1)
      - [`quote` (alias: `blockquote`)](#quote-alias-blockquote-1)
      - [`table`](#table-1)
      - [`image` (alias: `img`)](#image-alias-img-1)
      - [`divider` (alias: `hr`)](#divider-alias-hr-1)
      - [`html` (alias: `raw`)](#html-alias-raw-1)
  - [Customization Guide](#customization-guide-1)
    - [CSS Styling](#css-styling-1)
      - [Option 1: Use the built-in default](#option-1-use-the-built-in-default-1)
      - [Option 2: Replace the default CSS entirely](#option-2-replace-the-default-css-entirely-1)
      - [Option 3: Append extra CSS to the default](#option-3-append-extra-css-to-the-default-1)
      - [Option 4: Load CSS from a file](#option-4-load-css-from-a-file-1)
      - [Access the default CSS](#access-the-default-css-1)
    - [Paper Size](#paper-size-1)
    - [Page Margins](#page-margins-1)
    - [Orientation](#orientation-1)
    - [Headers](#headers-1)
      - [Simple left / center / right](#simple-left--center--right-1)
      - [Full custom HTML](#full-custom-html-1)
      - [No header (default)](#no-header-default-1)
    - [Footers](#footers-1)
      - [Default footer (when enabled)](#default-footer-when-enabled-1)
      - [Custom left / center / right](#custom-left--center--right-1)
      - [Full custom HTML footer](#full-custom-html-footer-1)
      - [Chrome Page Number Placeholders](#chrome-page-number-placeholders-1)
    - [Markdown Extensions](#markdown-extensions-1)
    - [Heading Numbers](#heading-numbers-1)
      - [Approach 1: JSON `auto_number` (JSON input only)](#approach-1-json-auto_number-json-input-only-1)
      - [Manual `number` field](#manual-number-field-1)
      - [Approach 2: CSS-based `heading_numbers` (works with ALL input types)](#approach-2-css-based-heading_numbers-works-with-all-input-types-1)
    - [Chrome Options](#chrome-options-1)
  - [Full Configuration Reference](#full-configuration-reference-1)
    - [PdfConfig Fields](#pdfconfig-fields-1)
  - [Examples](#examples-1)
    - [Example 1 — Minimal Markdown](#example-1--minimal-markdown-1)
    - [Example 2 — JSON with paraSequence](#example-2--json-with-parasequence-1)
    - [Example 3 — Custom CSS](#example-3--custom-css-1)
    - [Example 4 — Full Document](#example-4--full-document-1)
    - [Example 5 — Corporate Report](#example-5--corporate-report-1)
    - [Example 6 — Images](#example-6--images-1)
    - [Example 7 — Newsletter](#example-7--newsletter-1)
    - [Example 8 — Technical Doc (JSON)](#example-8--technical-doc-json-1)
  - [Running the Examples](#running-the-examples-1)
  - [Error Handling](#error-handling-1)
    - [Error Variants](#error-variants-1)
  - [Architecture](#architecture-1)
    - [Pipeline](#pipeline-1)
  - [Troubleshooting](#troubleshooting-1)
    - ["Chrome error: Failed to launch Chrome/Chromium"](#chrome-error-failed-to-launch-chromechromium-1)
    - [PDF is blank](#pdf-is-blank-1)
    - [Header/footer not showing](#headerfooter-not-showing-1)
    - [Custom CSS not applied](#custom-css-not-applied-1)
    - [Docker / CI: "No usable sandbox"](#docker--ci-no-usable-sandbox-1)
  - [License](#license-1)

---

## Features

| Feature | Description |
|---|---|
| **Markdown → PDF** | Convert any Markdown string or file to a styled PDF |
| **JSON → PDF** | Define document content with typed JSON blocks (headings, paragraphs, tables, code, lists, etc.) |
| **HTML → PDF** | Pass your own pre-built HTML directly |
| **Custom CSS** | Replace the default stylesheet entirely, or append extra rules |
| **Paper Sizes** | A4, Letter, Legal, or any custom width × height |
| **Page Margins** | Set top, bottom, left, right margins independently (in inches) |
| **Orientation** | Portrait or landscape |
| **Headers** | Simple left/center/right text, or full custom HTML |
| **Footers** | Simple left/center/right text with auto page numbers, or full custom HTML |
| **Heading Numbers** | Automatic hierarchical numbering (1, 1.1, 1.1.1) via JSON `paraSequence` or CSS counters |
| **Markdown Extensions** | Tables, footnotes, strikethrough, task lists, autolinks, superscript, description lists — all toggleable |
| **File Input** | Read `.md` or `.json` files directly from disk |
| **Builder Pattern** | Fluent, chainable API — configure only what you need |

---

## API Overview

- `PdfBuilder` — main fluent builder for generating PDFs from Markdown, JSON, or HTML.
- `PdfConfig` — full PDF generation configuration.
- `PaperSize`, `PageMargins`, `HeaderConfig`, `FooterConfig`, `MarkdownOptions` — core configuration helpers.
- `DEFAULT_CSS` — inspect or reuse the built-in stylesheet.
- `MdocError` / `Result` — error type and result alias used throughout the crate.

## Examples Summary

- Minimal Markdown PDF generation
- JSON document generation with `paraSequence` headings
- Custom CSS styling and theming
- Full document generation with headers, footers, and page options
- Report-style documents, newsletters, technical docs, and images

---

## Prerequisites

### Install Chromium / Chrome

`pdfsmith` uses **headless Chrome/Chromium** under the hood to render HTML to PDF. You must have Chrome or Chromium installed on any machine that generates PDFs.

#### Ubuntu / Debian

```bash
# Option A: via snap (recommended)
sudo snap install chromium

# Option B: via apt
sudo apt update
sudo apt install -y chromium-browser
```

#### Fedora / RHEL

```bash
sudo dnf install chromium
```

#### macOS

```bash
# If you have Google Chrome installed, it works automatically.
# Otherwise:
brew install --cask chromium
```

#### Windows

Install [Google Chrome](https://www.google.com/chrome/) or [Chromium](https://www.chromium.org/getting-involved/download-chromium). The `headless_chrome` crate will auto-detect it.

#### Docker

If your app runs in a container, add Chromium to your Dockerfile:

```dockerfile
RUN apt-get update && apt-get install -y chromium-browser --no-install-recommends && rm -rf /var/lib/apt/lists/*
```

> **Why not auto-install?** A library crate should never silently install system software. Chromium is a ~200 MB browser — the user should install it explicitly and control which version is on their system.

#### Verify the Installation

```bash
chromium --version
# or
chromium-browser --version
# or
google-chrome --version
```

If any of these prints a version number, you're ready.

---

## Installation

Add `pdfsmith` to your `Cargo.toml`:

```toml
[dependencies]
pdfsmith = "0.1.0"
```

Or via the command line:

```bash
cargo add pdfsmith
```

### Optional: Enable Logging

`pdfsmith` uses the [`log`](https://crates.io/crates/log) crate. To see progress messages, add a logger like `env_logger`:

```toml
[dependencies]
env_logger = "0.11"
```

Then initialise it in your code:

```rust
fn main() {
    env_logger::init(); // call once at startup
    // ... your code ...
}
```

Run with `RUST_LOG=info` to see output:

```bash
RUST_LOG=info cargo run
```

---

## Quick Start

```rust
use pdfsmith::PdfBuilder;

fn main() {
    let pdf_bytes = PdfBuilder::new()
        .from_markdown("# Hello World\n\nThis is my first PDF.")
        .expect("PDF generation failed");

    std::fs::write("hello.pdf", &pdf_bytes).expect("Failed to write file");
    println!("PDF saved! ({} bytes)", pdf_bytes.len());
}
```

That's it. Three lines of code, zero configuration. The default CSS, A4 paper, 0.75-inch margins, and no header/footer are applied automatically.

---

## Input Sources

### From Markdown String

```rust
use pdfsmith::PdfBuilder;

let pdf = PdfBuilder::new()
    .from_markdown("# Title\n\nSome **bold** and *italic* text.")
    .unwrap();
```

All standard Markdown is supported: headings, bold, italic, links, images, code blocks, tables, task lists, blockquotes, horizontal rules, footnotes, strikethrough, and more.

### From Markdown File

```rust
use pdfsmith::PdfBuilder;

let pdf = PdfBuilder::new()
    .from_markdown_file("docs/report.md")
    .unwrap();
```

### From JSON

```rust
use pdfsmith::PdfBuilder;

let json = serde_json::json!([
    { "type": "heading", "paraSequence": "1", "text": "Report Title" },
    { "type": "paragraph", "text": "Some content with **Markdown** formatting." },
    { "type": "heading", "paraSequence": "1.1", "text": "Details" },
    { "type": "paragraph", "text": "The heading level is derived from paraSequence depth." }
]);

let pdf = PdfBuilder::new()
    .from_json(&json)
    .unwrap();
```

JSON is first converted to Markdown, then rendered through the same pipeline — so all CSS and config applies equally.

### From JSON File

```rust
use pdfsmith::PdfBuilder;

let pdf = PdfBuilder::new()
    .from_json_file("data/document.json")
    .unwrap();
```

### From Raw HTML

```rust
use pdfsmith::PdfBuilder;

let html = r#"
<!DOCTYPE html>
<html>
<head><style>body { font-family: Arial; }</style></head>
<body><h1>Hello</h1><p>From raw HTML.</p></body>
</html>
"#;

let pdf = PdfBuilder::new()
    .from_html(html)
    .unwrap();
```

When using `from_html`, the HTML is sent to Chrome **as-is** — no CSS injection, no Markdown conversion. Header/footer and page options still apply.

---

## JSON Structure Reference

The JSON input is an array of **content blocks**. Each block has a `"type"` field that determines how it's rendered.

Two top-level formats are accepted:

```jsonc
// Format A: bare array
[
  { "type": "heading", "paraSequence": "1", "text": "Title" },
  { "type": "paragraph", "text": "Body." },
  { "type": "heading", "paraSequence": "1.1", "text": "Sub-section" },
  { "type": "paragraph", "text": "More body." }
]

// Format B: object with "content" key
{
  "content": [
    { "type": "heading", "paraSequence": "1", "text": "Title" },
    { "type": "paragraph", "text": "Body." }
  ]
}
```

### Block Types

#### `heading`

```json
{ "type": "heading", "paraSequence": "1.2", "text": "Section Title" }
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `paraSequence` | string | No | Hierarchical section number (e.g. `"1"`, `"1.2"`, `"1.2.1"`, `"A.1"`). The heading level (h1–h6) is **derived from the depth** (number of dot-separated parts). The sequence is prefixed to the heading text automatically. |
| `text` | string | Yes | Heading text (Markdown allowed) |
| `level` | number (1–6) | No | Explicit heading level — overrides the level derived from `paraSequence`. If neither `paraSequence` nor `level` is given, defaults to h1. |

#### `paragraph` (alias: `text`)

```json
{ "type": "paragraph", "text": "Some **bold** text with [links](https://example.com)." }
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `text` | string | Yes | Paragraph content (Markdown allowed) |

#### `code`

```json
{ "type": "code", "language": "rust", "text": "fn main() {\n    println!(\"Hello\");\n}" }
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `language` | string | No (default: `""`) | Syntax highlighting hint |
| `text` | string | Yes | Code content |

#### `list`

```json
{ "type": "list", "ordered": true, "items": ["First", "Second", "Third"] }
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ordered` | boolean | No (default: `false`) | `true` for numbered list, `false` for bullets |
| `items` | array of strings | Yes | List items (Markdown allowed in each item) |

#### `quote` (alias: `blockquote`)

```json
{ "type": "quote", "text": "To be or not to be." }
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `text` | string | Yes | Quote content (Markdown allowed) |

#### `table`

```json
{
  "type": "table",
  "headers": ["Name", "Role", "Location"],
  "rows": [
    ["Alice", "Engineer", "London"],
    ["Bob", "Designer", "Berlin"]
  ]
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `headers` | array of strings | Yes | Column headers |
| `rows` | array of arrays | No | Data rows (each row is an array of strings) |

#### `image` (alias: `img`)

```json
{ "type": "image", "src": "https://example.com/photo.png", "alt": "A photo" }
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `src` | string | Yes | Image URL or path |
| `alt` | string | No (default: `""`) | Alt text |

#### `divider` (alias: `hr`)

```json
{ "type": "divider" }
```

Renders a horizontal rule (`---`). No fields needed.

#### `html` (alias: `raw`)

```json
{ "type": "html", "text": "<div style='color:red;'>Custom HTML</div>" }
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `text` | string | Yes | Raw HTML passed through as-is |

---

## Customization Guide

Every aspect of the PDF is configurable through `PdfBuilder` methods or the `PdfConfig` struct directly.

### CSS Styling

#### Option 1: Use the built-in default

Do nothing — a clean, neutral, print-friendly stylesheet is applied automatically.

```rust
let pdf = PdfBuilder::new()
    .from_markdown("# Uses default CSS")
    .unwrap();
```

#### Option 2: Replace the default CSS entirely

```rust
let pdf = PdfBuilder::new()
    .custom_css(r#"
        body { font-family: Georgia, serif; font-size: 12pt; color: #333; }
        h1 { color: navy; border-bottom: 2px solid navy; }
        code { background: #f0f0f0; padding: 2px 4px; }
        table { border-collapse: collapse; width: 100%; }
        th, td { border: 1px solid #ccc; padding: 8px; }
    "#)
    .from_markdown("# Fully custom styled")
    .unwrap();
```

When `custom_css` is set, the built-in stylesheet is **completely replaced**. You are in full control.

#### Option 3: Append extra CSS to the default

```rust
let pdf = PdfBuilder::new()
    .extra_css("h1 { color: darkred; } blockquote { border-color: blue; }")
    .from_markdown("# Default + tweaks")
    .unwrap();
```

`extra_css` is added **after** the default stylesheet, so your rules override specific properties while keeping everything else intact.

#### Option 4: Load CSS from a file

```rust
let css = std::fs::read_to_string("styles/my-theme.css").unwrap();
let pdf = PdfBuilder::new()
    .custom_css(css)
    .from_markdown("# Themed")
    .unwrap();
```

#### Access the default CSS

The built-in stylesheet is exported as a constant if you want to inspect or extend it programmatically:

```rust
use pdfsmith::DEFAULT_CSS;
println!("{}", DEFAULT_CSS);
```

### Paper Size

```rust
use pdfsmith::{PdfBuilder, PaperSize};

// Preset sizes
PdfBuilder::new().paper_size(PaperSize::A4);      // 8.27 × 11.69 in (default)
PdfBuilder::new().paper_size(PaperSize::Letter);   // 8.5  × 11    in
PdfBuilder::new().paper_size(PaperSize::Legal);    // 8.5  × 14    in

// Custom size (width × height in inches)
PdfBuilder::new().paper_size(PaperSize::Custom { width: 6.0, height: 9.0 }); // e.g. trade paperback
```

### Page Margins

Margins are specified in inches:

```rust
use pdfsmith::{PdfBuilder, PageMargins};

let pdf = PdfBuilder::new()
    .margins(PageMargins {
        top: 1.0,
        bottom: 1.0,
        left: 0.5,
        right: 0.5,
    })
    .from_markdown("# Custom margins")
    .unwrap();
```

Default margins are `0.75` inches on all sides.

### Orientation

```rust
let pdf = PdfBuilder::new()
    .landscape(true)   // landscape
    .from_markdown("# Wide layout")
    .unwrap();
```

Default is `false` (portrait).

### Headers

Headers appear on every page. Three approaches:

#### Simple left / center / right

```rust
use pdfsmith::{PdfBuilder, HeaderConfig};

let pdf = PdfBuilder::new()
    .display_header_footer(true)
    .header(HeaderConfig {
        left: Some("Company Name".into()),
        center: Some("Document Title".into()),
        right: Some("2026-04-17".into()),
        font_size: Some("9px".into()),   // optional, default: "9px"
        color: Some("#333".into()),       // optional, default: "#555"
        ..Default::default()
    })
    .from_markdown("# With header")
    .unwrap();
```

#### Full custom HTML

```rust
use pdfsmith::{PdfBuilder, HeaderConfig};

let pdf = PdfBuilder::new()
    .display_header_footer(true)
    .header(HeaderConfig {
        custom_html: Some(r#"
            <div style="width:100%; text-align:center; font-size:10px; color:#999; padding:4px;">
                My Custom Header — Page <span class="pageNumber"></span>
            </div>
        "#.into()),
        ..Default::default()
    })
    .from_markdown("# With custom header HTML")
    .unwrap();
```

#### No header (default)

```rust
let pdf = PdfBuilder::new()
    .from_markdown("# No header")   // display_header_footer defaults to false
    .unwrap();
```

### Footers

Footers work exactly like headers.

#### Default footer (when enabled)

When `display_header_footer(true)` is set and no footer fields are configured, a simple centred page number is shown: `Page 1 / 5`.

```rust
let pdf = PdfBuilder::new()
    .display_header_footer(true)
    .from_markdown("# With page numbers")
    .unwrap();
```

#### Custom left / center / right

```rust
use pdfsmith::{PdfBuilder, FooterConfig};

let pdf = PdfBuilder::new()
    .display_header_footer(true)
    .footer(FooterConfig {
        left: Some("CONFIDENTIAL".into()),
        center: Some("Internal Use Only".into()),
        right: Some(r#"Page <span class="pageNumber"></span> of <span class="totalPages"></span>"#.into()),
        font_size: Some("8px".into()),
        color: Some("#888".into()),
        ..Default::default()
    })
    .from_markdown("# With footer")
    .unwrap();
```

#### Full custom HTML footer

```rust
use pdfsmith::{PdfBuilder, FooterConfig};

let pdf = PdfBuilder::new()
    .display_header_footer(true)
    .footer(FooterConfig {
        custom_html: Some(r#"
            <div style="width:100%; display:flex; justify-content:space-between; padding:4px 0.75in; font-size:8px; color:#666; font-family:Arial;">
                <span>© 2026 Acme Corp</span>
                <span>Page <span class="pageNumber"></span> / <span class="totalPages"></span></span>
            </div>
        "#.into()),
        ..Default::default()
    })
    .from_markdown("# Custom footer HTML")
    .unwrap();
```

#### Chrome Page Number Placeholders

Inside any header/footer text or `custom_html`, use these Chrome built-in placeholders:

| Placeholder | Renders as |
|---|---|
| `<span class="pageNumber"></span>` | Current page number |
| `<span class="totalPages"></span>` | Total page count |
| `<span class="title"></span>` | Document title |
| `<span class="url"></span>` | Page URL |
| `<span class="date"></span>` | Current date |

### Markdown Extensions

All extensions are enabled by default. Toggle them individually:

```rust
use pdfsmith::{PdfBuilder, MarkdownOptions};

let pdf = PdfBuilder::new()
    .markdown_options(MarkdownOptions {
        tables: true,              // | col1 | col2 |
        footnotes: true,           // [^1]: footnote text
        strikethrough: true,       // ~~deleted~~
        tasklist: true,            // - [x] done
        autolink: true,            // https://auto.link
        superscript: true,         // 2^10
        description_lists: true,   // term\n: definition
        unsafe_html: true,         // pass-through raw <html> tags
    })
    .from_markdown("# All extensions on")
    .unwrap();
```

To disable a specific extension:

```rust
let pdf = PdfBuilder::new()
    .markdown_options(MarkdownOptions {
        tasklist: false,       // disable task lists
        superscript: false,    // disable superscript
        ..Default::default()   // keep everything else on
    })
    .from_markdown("# Some extensions off")
    .unwrap();
```

### Heading Numbers

Automatic hierarchical heading numbers like **1**, **1.1**, **1.1.1**, **2**, **2.1**.

There are **two approaches** — use whichever fits your workflow:

#### Approach 1: JSON `auto_number` (JSON input only)

Set `"auto_number": true` in the top-level JSON object. Numbers are generated
automatically from the heading `level` fields:

```rust
use pdfsmith::PdfBuilder;

let json = serde_json::json!({
    "auto_number": true,
    "content": [
        { "type": "heading", "level": 1, "text": "Introduction" },
        { "type": "paragraph", "text": "Intro body." },
        { "type": "heading", "level": 2, "text": "Background" },
        { "type": "heading", "level": 2, "text": "Scope" },
        { "type": "heading", "level": 3, "text": "Details" },
        { "type": "heading", "level": 1, "text": "Conclusion" }
    ]
});

let pdf = PdfBuilder::new()
    .from_json(&json)
    .unwrap();
```

This produces headings:

```
1 Introduction
1.1 Background
1.2 Scope
1.2.1 Details
2 Conclusion
```

#### Manual `number` field

Any heading block can carry a `"number"` field for manual control.  This
takes priority over auto-numbering:

```json
[
  { "type": "heading", "level": 1, "number": "A", "text": "Appendix" },
  { "type": "heading", "level": 2, "number": "A.1", "text": "First Section" },
  { "type": "heading", "level": 2, "number": "A.2", "text": "Second Section" }
]
```

#### Approach 2: CSS-based `heading_numbers` (works with ALL input types)

Enable `.heading_numbers(true)` on the builder. This injects CSS counter
rules that automatically number every heading — works for Markdown, JSON,
and raw HTML:

```rust
use pdfsmith::PdfBuilder;

let pdf = PdfBuilder::new()
    .heading_numbers(true)
    .from_markdown("# Intro\n\n## Part A\n\n## Part B\n\n### Detail\n\n# Summary")
    .unwrap();
```

Rendered headings look like:

```
1. Intro
1.1 Part A
1.2 Part B
1.2.1 Detail
2. Summary
```

The CSS approach has no dependency on JSON — it works everywhere.

> **Which should I use?**
> - Use **`auto_number`** in JSON when you want the numbers baked into
>   the Markdown text (good for exports, copy-paste).
> - Use **`heading_numbers(true)`** on the builder when you want visual
>   numbering via CSS (works with any input, numbers don’t appear in
>   the underlying text).

### Chrome Options

Fine-tune the headless Chrome rendering:

```rust
let pdf = PdfBuilder::new()
    .chrome_window_size(1920, 1080)   // browser viewport (default: 1280×900)
    .page_load_wait_secs(5)           // wait for images/JS (default: 2)
    .print_background(true)           // render background colours (default: true)
    .from_markdown("# Chrome settings")
    .unwrap();
```

---

## Full Configuration Reference

You can also set everything at once via the `PdfConfig` struct:

```rust
use pdfsmith::{PdfBuilder, PdfConfig, PaperSize, PageMargins, HeaderConfig, FooterConfig, MarkdownOptions};

let config = PdfConfig {
    title: "My Document".into(),
    custom_css: None,                                  // use built-in default
    extra_css: Some("h1 { color: navy; }".into()),     // tweak headings
    paper_size: PaperSize::A4,
    margins: PageMargins { top: 1.0, bottom: 1.0, left: 0.75, right: 0.75 },
    landscape: false,
    display_header_footer: true,
    header: HeaderConfig {
        left: Some("Acme Corp".into()),
        center: Some("Report".into()),
        right: Some("April 2026".into()),
        font_size: Some("9px".into()),
        color: Some("#333".into()),
        custom_html: None,
    },
    footer: FooterConfig {
        left: None,
        center: None,
        right: Some(r#"Page <span class="pageNumber"></span>"#.into()),
        font_size: Some("8px".into()),
        color: Some("#888".into()),
        custom_html: None,
    },
    print_background: true,
    heading_numbers: false,
    markdown_options: MarkdownOptions::default(),
    chrome_window_size: (1280, 900),
    page_load_wait_secs: 5, // use 5 when any image is available in markdown or json.
};

let pdf = PdfBuilder::with_config(config)
    .from_markdown("# Hello from PdfConfig")
    .unwrap();
```

### PdfConfig Fields

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `title` | `String` | `""` | Document title (used in `<title>` tag and default header) |
| `custom_css` | `Option<String>` | `None` | Replaces the entire default stylesheet |
| `extra_css` | `Option<String>` | `None` | Appended after the base stylesheet |
| `paper_size` | `PaperSize` | `A4` | Paper dimensions |
| `margins` | `PageMargins` | `0.75` all | Page margins in inches |
| `landscape` | `bool` | `false` | Landscape orientation |
| `display_header_footer` | `bool` | `false` | Show header and footer |
| `header` | `HeaderConfig` | empty | Header configuration |
| `footer` | `FooterConfig` | empty | Footer configuration |
| `print_background` | `bool` | `true` | Print background colours/images |
| `markdown_options` | `MarkdownOptions` | all `true` | Markdown extension toggles |
| `heading_numbers` | `bool` | `false` | CSS-based hierarchical heading numbers |
| `chrome_window_size` | `(u32, u32)` | `(1280, 900)` | Headless Chrome viewport |
| `page_load_wait_secs` | `u64` | `5` | Seconds to wait after page load ( Usually helpfull to load images perfectly) |

---

## Examples

The repository includes **eight** runnable examples in the `examples/` directory, each producing a 2+ page PDF.

### Example 1 — Minimal Markdown

Markdown string → PDF with `heading_numbers(true)`. Default CSS, no header/footer. Covers tables, code blocks, lists, blockquotes.

```bash
cargo run --example from_markdown
```

### Example 2 — JSON with paraSequence

Structured JSON content blocks with `"paraSequence"` on every heading for hierarchical numbering. The heading level (h1–h6) is derived automatically from the sequence depth.

```rust
// examples/from_json.rs  (abbreviated)
let json = serde_json::json!([
    { "type": "heading", "paraSequence": "1",     "text": "Project Status Report" },
    { "type": "paragraph", "text": "This report summarises the current state." },
    { "type": "heading", "paraSequence": "1.1",   "text": "Timeline" },
    { "type": "table", "headers": ["Phase","Status","Due"], "rows": [["Design","Complete","2026-01-15"]] },
    { "type": "heading", "paraSequence": "1.2",   "text": "Key Highlights" },
    { "type": "heading", "paraSequence": "1.2.1", "text": "Performance Details" },
    { "type": "heading", "paraSequence": "2",     "text": "Architecture" },
    // ... more blocks ...
]);

let pdf = PdfBuilder::new()
    .title("Project Report")
    .from_json(&json)
    .expect("Failed to generate PDF");
```

Output headings:

```
1 Project Status Report       (depth 1 → h1)
  1.1 Timeline                (depth 2 → h2)
  1.2 Key Highlights          (depth 2 → h2)
    1.2.1 Performance Details  (depth 3 → h3)
2 Architecture                (depth 1 → h1)
```

```bash
cargo run --example from_json
```

### Example 3 — Custom CSS

Replaces the entire default stylesheet. Indented sub-headings (h2 → 20px, h3 → 40px, h4 → 60px), per-level colours (blue → green → gold), dark code blocks, `heading_numbers(true)` on top.

```bash
cargo run --example custom_css
```

### Example 4 — Full Document

Every option configured: title, Letter landscape paper, header, footer, `heading_numbers(true)`, extra CSS for colours. Quarterly business review theme.

```bash
cargo run --example full_document
```

### Example 5 — Corporate Report

A4 report with default CSS as the base, small tweaks via `extra_css` (indented headings, corporate colours). Header shows company name + report title, footer shows "INTERNAL" + page numbers.

```bash
cargo run --example report_style
```

### Example 6 — Images

Markdown document with remote images (Wikimedia Commons). Uses `page_load_wait_secs(5)` to allow time for image loading. Header/footer, heading numbers, extra CSS for image styling.

```bash
cargo run --example with_images
```

### Example 7 — Newsletter

Vibrant editorial style with full `custom_css`: gradient heading backgrounds, purple/green/gold colours, styled blockquotes. No heading numbers — clean magazine look. Header + footer.

```bash
cargo run --example newsletter
```

### Example 8 — Technical Doc (JSON)

Complete API reference built entirely from JSON with `paraSequence` on every heading. Uses a `build_content()` helper to construct large JSON arrays. Header/footer, indented headings via `extra_css`.

```bash
cargo run --example technical_doc
```

---

## Running the Examples

```bash
# Clone the repository
git clone github.com/DhvaniBhesaniya/pdfsmith.git
cd pdfsmith

# Run any example
cargo run --example from_markdown
cargo run --example from_json
cargo run --example custom_css
cargo run --example full_document
cargo run --example report_style
cargo run --example with_images
cargo run --example newsletter
cargo run --example technical_doc

# With logging enabled
RUST_LOG=info cargo run --example from_markdown
```

---

## Error Handling

All generation methods return `pdfsmith::Result<Vec<u8>>`, which is `Result<Vec<u8>, MdocError>`.

```rust
use pdfsmith::{PdfBuilder, MdocError};

match PdfBuilder::new().from_markdown("# Test") {
    Ok(pdf) => std::fs::write("out.pdf", pdf).unwrap(),
    Err(MdocError::Chrome(msg)) => eprintln!("Chrome error: {}", msg),
    Err(MdocError::Io(err))     => eprintln!("IO error: {}", err),
    Err(MdocError::Json(msg))   => eprintln!("JSON error: {}", msg),
    Err(e)                      => eprintln!("Error: {}", e),
}
```

### Error Variants

| Variant | When it occurs |
|---|---|
| `MdocError::Chrome(String)` | Chrome/Chromium not found, failed to launch, navigation timeout, print failure |
| `MdocError::Io(std::io::Error)` | File read/write errors |
| `MdocError::Json(String)` | Invalid JSON structure (not an array or missing `"content"` key) |
| `MdocError::ImageDownload { url, reason }` | Remote image fetch failed |
| `MdocError::Other(String)` | Catch-all for uncategorised errors |

---

## Architecture

```
pdfsmith/
├── src/
│   ├── lib.rs              # Public API — PdfBuilder and re-exports
│   ├── config.rs           # PdfConfig, PaperSize, PageMargins, HeaderConfig, FooterConfig, MarkdownOptions
│   ├── css.rs              # DEFAULT_CSS constant
│   ├── error.rs            # MdocError enum and Result type alias
│   ├── parser/
│   │   ├── mod.rs          # Re-exports
│   │   ├── markdown.rs     # Markdown → HTML body (via comrak)
│   │   └── json.rs         # JSON content blocks → Markdown
│   └── renderer/
│       ├── mod.rs          # Re-exports
│       ├── html.rs         # Wraps body HTML in full document with CSS
│       ├── template.rs     # Header/footer template builders
│       └── chrome.rs       # Headless Chrome PDF rendering
├── examples/
│   ├── from_markdown.rs
│   ├── from_json.rs
│   ├── custom_css.rs
│   ├── full_document.rs
│   ├── report_style.rs
│   ├── with_images.rs
│   ├── newsletter.rs
│   └── technical_doc.rs
├── tests/
│   ├── basic.rs
│   └── json.rs
└── Cargo.toml
```

### Pipeline

```
Markdown string ─┐
                  ├─→ HTML body ─→ Full HTML doc (+ CSS) ─→ Chrome ─→ PDF bytes
JSON blocks ─────┘    (comrak)     (wrap_body_in_html)      (headless)
```

1. **Input** — Markdown text, JSON blocks, or raw HTML.
2. **Parse** — Markdown is converted to HTML by comrak. JSON is first converted to Markdown, then through comrak.
3. **Wrap** — The HTML body is wrapped in a full `<!DOCTYPE html>` document with the resolved CSS (default, custom, or default + extra).
4. **Template** — Header and footer HTML templates are built from config.
5. **Render** — Headless Chrome loads the HTML, applies print options (paper, margins, header, footer), and outputs PDF bytes.
6. **Return** — Raw `Vec<u8>` PDF bytes ready to write to disk or send over HTTP.

---

## Troubleshooting

### "Chrome error: Failed to launch Chrome/Chromium"

Chromium is not installed or not in your `PATH`. See [Prerequisites](#prerequisites).

### PDF is blank

- Check that your Markdown/JSON input is not empty.
- Increase `page_load_wait_secs` if your HTML has images that need time to load.

### Header/footer not showing

- You must call `.display_header_footer(true)` — it defaults to `false`.
- Increase `margins.top` / `margins.bottom` to make room for the header/footer.

### Custom CSS not applied

- If you use `custom_css`, it **replaces** the default entirely. Make sure your CSS covers everything (body font, headings, tables, code blocks, etc.).
- If you only want small tweaks, use `extra_css` instead.

### Docker / CI: "No usable sandbox"

Chrome needs a sandbox. In Docker, run with `--no-sandbox`:

```dockerfile
ENV CHROME_FLAGS="--no-sandbox --disable-setuid-sandbox"
```

Or use a container image that includes Chromium with proper sandbox config.

---

## License

MIT
