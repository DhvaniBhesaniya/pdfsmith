// examples/from_json.rs
//
// Generate a 2+ page PDF from structured JSON content blocks.
// Uses "paraSequence" on headings for hierarchical numbering (1, 1.1, 1.2.1).
// No header/footer — pure content showcase.
//
// Run:  cargo run --example from_json

use pdfsmith::PdfBuilder;

fn main() {
    env_logger::init();

    let json = serde_json::json!([
        // ─── 1 ──────────────────────────────────────────────────────
        { "type": "heading", "paraSequence": "1", "text": "Project Status Report" },
        { "type": "paragraph", "text": "This report summarises the current state of **Project Aurora**, covering timeline, team structure, technical highlights, risk assessment, and next steps. It is generated entirely from structured JSON using the `pdfsmith` crate." },

        // ─── 1.1 ────────────────────────────────────────────────────
        { "type": "heading", "paraSequence": "1.1", "text": "Timeline" },
        { "type": "paragraph", "text": "The following table shows the major project phases, their current status, and target dates:" },
        { "type": "table",
          "headers": ["Phase", "Status", "Start", "Due", "Owner"],
          "rows": [
              ["Discovery",      "Complete",    "2025-09-01", "2025-10-15", "Sarah Chen"],
              ["Design",         "Complete",    "2025-10-16", "2025-12-20", "Raj Patel"],
              ["Implementation", "In Progress", "2026-01-06", "2026-04-30", "Dev Team"],
              ["Testing",        "Pending",     "2026-05-01", "2026-06-15", "QA Team"],
              ["Deployment",     "Pending",     "2026-06-16", "2026-06-30", "DevOps"],
              ["Post-launch",    "Planned",     "2026-07-01", "2026-08-31", "Support"]
          ]
        },

        // ─── 1.2 ────────────────────────────────────────────────────
        { "type": "heading", "paraSequence": "1.2", "text": "Team Structure" },
        { "type": "paragraph", "text": "The project currently has **14 active contributors** across four functional teams:" },
        { "type": "list", "ordered": false, "items": [
            "**Core Engineering** (5) — backend services, database layer, API design",
            "**Frontend** (3) — React dashboard, component library, accessibility",
            "**QA & Testing** (3) — automated tests, performance benchmarks, security audits",
            "**DevOps & Infra** (3) — CI/CD pipelines, cloud provisioning, monitoring"
        ]},

        // ─── 1.3 ────────────────────────────────────────────────────
        { "type": "heading", "paraSequence": "1.3", "text": "Key Highlights" },
        { "type": "list", "ordered": true, "items": [
            "Core module is **feature complete** — all 47 planned endpoints are live",
            "Performance benchmarks exceed targets by 22% on p99 latency",
            "Security audit passed with zero critical findings",
            "Frontend lighthouse score: 96 (Performance), 100 (Accessibility)",
            "Two open issues remain in the tracker (see 1.3.2)"
        ]},

        // ─── 1.3.1 ──────────────────────────────────────────────────
        { "type": "heading", "paraSequence": "1.3.1", "text": "Performance Details" },
        { "type": "paragraph", "text": "Average response time improved by **32%** compared to the previous quarter. The p50 latency dropped from 45ms to 31ms, and p99 latency from 320ms to 210ms." },
        { "type": "table",
          "headers": ["Metric", "Q4 2025", "Q1 2026", "Change"],
          "rows": [
              ["p50 latency",   "45 ms",  "31 ms",  "▼ 31%"],
              ["p99 latency",   "320 ms", "210 ms", "▼ 34%"],
              ["Throughput",    "1,200 rps", "1,850 rps", "▲ 54%"],
              ["Error rate",    "0.12%",  "0.04%",  "▼ 67%"],
              ["Memory usage",  "512 MB", "380 MB", "▼ 26%"]
          ]
        },

        // ─── 1.3.2 ──────────────────────────────────────────────────
        { "type": "heading", "paraSequence": "1.3.2", "text": "Open Issues" },
        { "type": "list", "ordered": true, "items": [
            "**AURORA-847** — Memory leak in long-running WebSocket sessions (assigned: Alex, priority: high)",
            "**AURORA-912** — Flaky integration test in CI pipeline — passes locally but fails 15% of the time on ARM runners (assigned: Priya, priority: medium)"
        ]},

        // ─── 2 ──────────────────────────────────────────────────────
        { "type": "heading", "paraSequence": "2", "text": "Architecture Overview" },
        { "type": "paragraph", "text": "Project Aurora uses a modular, layered architecture. Each component can be developed, tested, and deployed independently." },

        // ─── 2.1 ────────────────────────────────────────────────────
        { "type": "heading", "paraSequence": "2.1", "text": "Service Layer" },
        { "type": "code", "language": "rust", "text": "pub struct ServiceLayer {\n    auth: AuthService,\n    users: UserService,\n    billing: BillingService,\n    notifications: NotificationService,\n}\n\nimpl ServiceLayer {\n    pub async fn handle_request(&self, req: Request) -> Response {\n        let user = self.auth.verify(&req).await?;\n        match req.route() {\n            \"/users/*\"    => self.users.handle(user, req).await,\n            \"/billing/*\"  => self.billing.handle(user, req).await,\n            \"/notify/*\"   => self.notifications.handle(user, req).await,\n            _             => Response::not_found(),\n        }\n    }\n}" },

        // ─── 2.2 ────────────────────────────────────────────────────
        { "type": "heading", "paraSequence": "2.2", "text": "Database Schema" },
        { "type": "paragraph", "text": "The primary data store is PostgreSQL 16 with the following key tables:" },
        { "type": "table",
          "headers": ["Table", "Rows (est.)", "Indexes", "Notes"],
          "rows": [
              ["users",          "1.2M",  "3", "Partitioned by region"],
              ["transactions",   "48M",   "5", "Time-series partitioned"],
              ["audit_log",      "120M",  "2", "Append-only, compressed"],
              ["sessions",       "350K",  "2", "TTL: 24 hours"],
              ["notifications",  "8.5M",  "3", "Soft-delete enabled"]
          ]
        },

        // ─── 3 ──────────────────────────────────────────────────────
        { "type": "heading", "paraSequence": "3", "text": "Risk Assessment" },
        { "type": "paragraph", "text": "The following risks have been identified and are being actively monitored:" },
        { "type": "table",
          "headers": ["Risk", "Impact", "Probability", "Mitigation"],
          "rows": [
              ["Scope creep",        "High",   "Medium", "Strict change-request process"],
              ["Key-person dependency", "High", "Low",    "Cross-training & documentation"],
              ["Third-party API outage", "Medium", "Medium", "Circuit breakers & fallbacks"],
              ["Security vulnerability", "Critical", "Low", "Regular audits & dependency scanning"]
          ]
        },

        // ─── 4 ──────────────────────────────────────────────────────
        { "type": "heading", "paraSequence": "4", "text": "Next Steps" },
        { "type": "list", "ordered": true, "items": [
            "Resolve **AURORA-847** memory leak before April 25",
            "Complete integration test stabilisation by April 28",
            "Begin QA test cycle on May 1",
            "Schedule security pen-test for May 10–14",
            "Prepare deployment runbook by May 20",
            "Go/No-Go meeting on June 12"
        ]},

        { "type": "divider" },

        { "type": "quote", "text": "Ship early, ship often.  Every week of delay is a week of lost feedback." },

        { "type": "paragraph", "text": "Next review meeting is scheduled for **May 1, 2026** at 10:00 AM UTC.  All stakeholders are expected to attend." }
    ]);

    let pdf = PdfBuilder::new()
        .title("Project Aurora — Status Report")
        .from_json(&json)
        .expect("Failed to generate PDF");

    std::fs::write("examples/output_pdfs/from_json.pdf", &pdf).expect("Failed to write PDF");
    println!("PDF saved to examples/output_pdfs/from_json.pdf ({} bytes)", pdf.len());
}
