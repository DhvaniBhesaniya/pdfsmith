// examples/full_document.rs
//
// Every option in one place: title, paper size, landscape, margins,
// header, footer, heading numbers, extra CSS, 2+ pages.
//
// Run:  cargo run --example full_document

use pdfsmith::{FooterConfig, HeaderConfig, PageMargins, PaperSize, PdfBuilder};

fn main() {
    env_logger::init();

    let markdown = r#"
# Quarterly Business Review — Q1 2026

This document demonstrates **every customization option** available in
`pdfsmith`.  It uses US Letter paper in landscape, custom margins, a
company header, a confidential footer, heading numbers, and extra CSS.

## Executive Summary

Q1 2026 was a strong quarter.  Revenue grew **18%** year-over-year to
$12.4M, driven by enterprise expansion and new product launches.
Customer retention rate held steady at 94%.

Key wins:

- Signed three Fortune 500 accounts (combined ACV: $1.8M)
- Launched v2.0 of the platform with real-time analytics
- Reduced infrastructure costs by 22% through Kubernetes migration
- Net Promoter Score improved from 42 to 51

## Financial Overview

### Revenue Breakdown

| Segment          | Q1 2025   | Q1 2026   | Change  |
|------------------|-----------|-----------|---------|
| Enterprise       | $5.2M     | $6.8M     | +31%    |
| Mid-Market       | $3.1M     | $3.4M     | +10%    |
| SMB              | $1.8M     | $1.7M     | −6%     |
| Professional Svcs| $0.4M     | $0.5M     | +25%    |
| **Total**        | **$10.5M**| **$12.4M**| **+18%**|

### Operating Expenses

| Category         | Budget    | Actual    | Variance |
|------------------|-----------|-----------|----------|
| Engineering      | $3.2M     | $3.0M     | −$200K ✓ |
| Sales & Marketing| $2.8M     | $3.1M     | +$300K ⚠ |
| G&A              | $1.0M     | $0.95M    | −$50K ✓  |
| Infrastructure   | $0.8M     | $0.62M    | −$180K ✓ |
| **Total**        | **$7.8M** | **$7.67M**| **−$130K ✓** |

### Cash Position

Opening balance: **$28.4M**.  Net cash flow: **+$4.7M**.
Closing balance: **$33.1M** — runway of 14 months at current burn.

## Product & Engineering

### Platform v2.0 Launch

Released March 15, 2026.  Key features:

1. **Real-time analytics dashboard** — sub-second updates via WebSocket
2. **Multi-tenant RBAC** — role-based access with custom permission sets
3. **Bulk import API** — 10x throughput for data migration
4. **Audit log explorer** — full-text search across 120M+ events

### Engineering Metrics

| Metric              | Q4 2025 | Q1 2026 | Target  |
|---------------------|---------|---------|---------|
| Deploy frequency    | 3/week  | 5/week  | 4/week ✓|
| Lead time (commit→prod) | 4 days | 2 days | 3 days ✓|
| Change failure rate | 8%      | 4%      | <5% ✓   |
| MTTR               | 45 min  | 22 min  | <30 min ✓|
| Test coverage       | 74%     | 81%     | 80% ✓   |

### Technical Debt

Three items flagged for Q2 attention:

- Legacy billing integration (PHP) — scheduled for Rust rewrite
- Monolith → microservice split for notification service
- Database connection pooling optimisation

## Sales & Marketing

### Pipeline Summary

| Stage          | Count | Weighted Value |
|----------------|-------|----------------|
| Prospecting    | 142   | $2.1M          |
| Qualification  | 68    | $3.4M          |
| Proposal       | 31    | $4.2M          |
| Negotiation    | 12    | $2.8M          |
| Closed Won     | 47    | $6.8M          |
| Closed Lost    | 18    | $1.2M          |

### Marketing Highlights

- Blog traffic up **40%** (SEO campaign launched in January)
- Webinar series: 1,200 registrations, 38% attendance rate
- Case study with GlobalTech published — 5,000 downloads in first week
- Conference sponsorship at RustConf 2026 confirmed (June booth)

## Customer Success

### Retention & Expansion

| Metric              | Q4 2025 | Q1 2026 |
|---------------------|---------|---------|
| Gross retention     | 93%     | 94%     |
| Net revenue retention| 108%   | 112%    |
| NPS                 | 42      | 51      |
| Support tickets/week| 180     | 145     |
| Avg resolution time | 6.2 hrs | 4.8 hrs|

### Top Feature Requests

1. **PDF export** — customers want to generate reports from the platform (✓ shipped via pdfsmith)
2. **Slack integration** — real-time alerts in Slack channels
3. **Custom dashboards** — drag-and-drop widget builder
4. **SSO with SAML** — enterprise requirement for 8 accounts in pipeline

## Risk Register

| Risk                       | Impact   | Likelihood | Owner    | Mitigation                        |
|----------------------------|----------|------------|----------|-----------------------------------|
| Key engineer departure     | High     | Low        | VP Eng   | Cross-training, competitive comp  |
| AWS region outage          | Critical | Very Low   | SRE      | Multi-region failover in place    |
| Competitor price war       | Medium   | Medium     | VP Sales | Value-based selling, lock-in discounts |
| Regulatory change (GDPR)   | Medium   | Low        | Legal    | Quarterly compliance review       |
| Supply chain (hardware)    | Low      | Medium     | IT       | 90-day buffer stock               |

## Q2 2026 Priorities

1. **Close $5M+ in new ARR** — focus on enterprise pipeline
2. **Ship Slack integration** — top customer request, target: May 15
3. **Reduce MTTR to <15 min** — invest in observability tooling
4. **Hire 4 engineers** — 2 backend, 1 frontend, 1 SRE
5. **SOC 2 Type II audit** — kick off in May, complete by August
6. **Customer advisory board** — launch with 8 strategic accounts

---

> The best time to plant a tree was 20 years ago.  The second best time is now.

*This report is confidential.  Distribution limited to executive team and board members.*
"#;

    let pdf = PdfBuilder::new()
        .title("Q1 2026 Business Review")
        .paper_size(PaperSize::Letter)
        .landscape(true)
        .margins(PageMargins {
            top: 0.8,
            bottom: 0.8,
            left: 0.6,
            right: 0.6,
        })
        .display_header_footer(true)
        .header(HeaderConfig {
            left: Some("Acme Corp".into()),
            center: Some("Quarterly Business Review — Q1 2026".into()),
            right: Some("2026-04-17".into()),
            font_size: Some("9px".into()),
            color: Some("#333".into()),
            ..Default::default()
        })
        .footer(FooterConfig {
            left: Some("CONFIDENTIAL — Internal Use Only".into()),
            right: Some(
                r#"Page <span class="pageNumber"></span> of <span class="totalPages"></span>"#
                    .into(),
            ),
            font_size: Some("8px".into()),
            color: Some("#888".into()),
            ..Default::default()
        })
        .heading_numbers(true)
        .extra_css("h1 { color: #1a5276; } h2 { color: #2874a6; } h3 { color: #1e8449; } blockquote { border-color: #2874a6; background: #eaf2f8; }")
        .from_markdown(markdown)
        .expect("Failed to generate PDF");

    std::fs::write("examples/output_pdfs/full_document.pdf", &pdf).expect("Failed to write PDF");
    println!("PDF saved to examples/output_pdfs/full_document.pdf ({} bytes)", pdf.len());
}
