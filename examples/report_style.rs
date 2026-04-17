// examples/report_style.rs
//
// Corporate report style: header + footer, indented sub-headings via
// extra_css, heading numbers, 2+ pages.  Uses default CSS as the base
// and only appends small tweaks via extra_css.
//
// Run:  cargo run --example report_style

use pdfsmith::{FooterConfig, HeaderConfig, PageMargins, PaperSize, PdfBuilder};

fn main() {
    env_logger::init();

    // extra_css sits ON TOP of the built-in default — only override what
    // you need.  Here we add indent + colour to sub-headings.
    let extra = r#"
        h1 { color: #0b3d91; border-bottom: 2px solid #0b3d91; }
        h2 { color: #1a6fb5; margin-left: 16px; }
        h3 { color: #2e8b57; margin-left: 32px; }
        h4 { color: #b8860b; margin-left: 48px; }
        blockquote {
            border-left-color: #0b3d91;
            background: #f0f5fb;
            padding: 10px 16px;
        }
    "#;

    let markdown = r#"
# Annual Infrastructure Report — 2025

Prepared by the Cloud Operations team for the executive leadership group.
This report covers availability, incident analysis, capacity forecasting,
security posture, and planned improvements for 2026.

## Availability & Uptime

### Service Level Summary

| Service             | SLA Target | Actual  | Status |
|---------------------|-----------|---------|--------|
| API Gateway         | 99.95%    | 99.97%  | ✓ Met  |
| Web Application     | 99.9%     | 99.92%  | ✓ Met  |
| Database Cluster    | 99.99%    | 99.98%  | ⚠ Close|
| Message Queue       | 99.9%     | 99.95%  | ✓ Met  |
| CDN                 | 99.9%     | 99.99%  | ✓ Met  |
| Authentication      | 99.99%    | 99.995% | ✓ Met  |

### Monthly Uptime Trend

| Month    | API    | Web    | DB     | Queue  |
|----------|--------|--------|--------|--------|
| Jan 2025 | 99.98% | 99.94% | 99.99% | 99.97% |
| Feb 2025 | 99.99% | 99.96% | 99.99% | 99.98% |
| Mar 2025 | 99.95% | 99.89% | 99.97% | 99.92% |
| Apr 2025 | 99.97% | 99.93% | 99.98% | 99.96% |
| May 2025 | 99.98% | 99.95% | 99.99% | 99.97% |
| Jun 2025 | 99.96% | 99.91% | 99.98% | 99.94% |
| Jul 2025 | 99.99% | 99.97% | 100%   | 99.99% |
| Aug 2025 | 99.97% | 99.93% | 99.98% | 99.95% |
| Sep 2025 | 99.98% | 99.94% | 99.99% | 99.97% |
| Oct 2025 | 99.96% | 99.90% | 99.97% | 99.93% |
| Nov 2025 | 99.97% | 99.92% | 99.99% | 99.96% |
| Dec 2025 | 99.99% | 99.96% | 99.99% | 99.98% |

## Incident Analysis

### Severity Breakdown

| Severity | Count | MTTR (avg) | Impact Hours |
|----------|-------|-----------|--------------|
| SEV-1    | 2     | 28 min    | 0.93 hrs     |
| SEV-2    | 7     | 45 min    | 5.25 hrs     |
| SEV-3    | 23    | 2.1 hrs   | 48.3 hrs     |
| SEV-4    | 61    | 4.8 hrs   | N/A (low)    |

### Notable Incidents

#### SEV-1: Database Failover — March 12

Root cause: Primary PostgreSQL node ran out of WAL space due to a stuck
replication slot.  Automatic failover triggered but took 14 minutes
instead of the target 30 seconds because of a misconfigured health check.

**Resolution**: Fixed health check interval, added WAL space monitoring
alert at 70% threshold, updated runbook.

#### SEV-1: CDN Cache Purge Storm — October 8

Root cause: A deployment script accidentally purged the entire CDN cache
instead of a single path prefix.  Origin servers received 12x normal
traffic for 22 minutes until cache warmed back up.

**Resolution**: Added confirmation prompt and path-prefix validation to
purge scripts.  Rate-limited purge API to max 100 paths per minute.

## Capacity Planning

### Current Utilisation

| Resource        | Allocated  | Used (avg) | Used (peak) | Headroom |
|-----------------|-----------|------------|-------------|----------|
| Compute (vCPU)  | 2,400     | 1,440 (60%)| 2,040 (85%) | 15%      |
| Memory (GB)     | 9,600     | 5,760 (60%)| 7,680 (80%) | 20%      |
| Storage (TB)    | 120       | 84 (70%)   | N/A         | 30%      |
| Network (Gbps)  | 40        | 18 (45%)   | 32 (80%)    | 20%      |

### 2026 Forecast

Based on 25% year-over-year traffic growth:

1. **Compute** — Add 600 vCPU in Q2, migrate to Graviton4
2. **Memory** — Current allocation sufficient through Q3
3. **Storage** — Provision additional 40TB in Q1 (data lake expansion)
4. **Network** — Upgrade to 100Gbps backbone in Q2

## Security Posture

### Vulnerability Summary

| Category          | Critical | High | Medium | Low  |
|-------------------|----------|------|--------|------|
| Application       | 0        | 2    | 8      | 15   |
| Infrastructure    | 0        | 1    | 5      | 12   |
| Dependencies      | 0        | 3    | 11     | 28   |
| **Total**         | **0**    | **6**| **24** | **55**|

All critical and high vulnerabilities resolved within SLA (72 hours for
critical, 14 days for high).

### Compliance

- SOC 2 Type II: **Passed** (audit completed November 2025)
- ISO 27001: **Certification renewed** (valid through December 2027)
- GDPR: **Compliant** — quarterly review completed, DPA updated
- PCI DSS Level 2: **In progress** — target certification Q2 2026

## Recommendations for 2026

1. **Adopt multi-region active-active** — eliminate single-region SPOF
2. **Implement chaos engineering** — monthly game days starting Q1
3. **Upgrade observability stack** — migrate from Prometheus to Grafana Mimir
4. **Automate incident response** — PagerDuty + Terraform runbooks
5. **Zero-trust networking** — replace VPN with identity-aware proxy

---

> Infrastructure is invisible when done well.  You only notice it when it breaks.

*This report is auto-generated.  Data sourced from DataDog, PagerDuty, and AWS Cost Explorer.*
"#;

    let pdf = PdfBuilder::new()
        .title("Infrastructure Report 2025")
        .paper_size(PaperSize::A4)
        .margins(PageMargins {
            top: 0.9,
            bottom: 0.9,
            left: 0.75,
            right: 0.75,
        })
        .display_header_footer(true)
        .header(HeaderConfig {
            left: Some("NovaTech Inc.".into()),
            center: Some("Annual Infrastructure Report".into()),
            right: Some("2025".into()),
            font_size: Some("8px".into()),
            color: Some("#0b3d91".into()),
            ..Default::default()
        })
        .footer(FooterConfig {
            left: Some("INTERNAL — Do Not Distribute".into()),
            right: Some(
                r#"Page <span class="pageNumber"></span> / <span class="totalPages"></span>"#
                    .into(),
            ),
            font_size: Some("7px".into()),
            color: Some("#999".into()),
            ..Default::default()
        })
        .heading_numbers(true)
        .extra_css(extra)
        .from_markdown(markdown)
        .expect("Failed to generate PDF");

    std::fs::write("examples/output_pdfs/report_style.pdf", &pdf).expect("Failed to write PDF");
    println!("PDF saved to examples/output_pdfs/report_style.pdf ({} bytes)", pdf.len());
}
