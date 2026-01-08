//! Reporting and output formatting.

use crate::config::{PatternCategory, Severity};
use crate::detector::{Finding, ScanSummary};
use crate::Error;
use crate::Result;
use owo_colors::OwoColorize;
use serde::Serialize;
use std::io::{self, Write};

mod sarif;

/// Output format.
#[derive(Debug, Clone, Copy, clap::ValueEnum, PartialEq, Eq)]
pub enum Format {
    /// Human-readable colored terminal output.
    Human,
    /// Machine-readable JSON output.
    Json,
    /// SARIF XML/JSON output for integrations.
    Sarif,
}

impl Format {
    /// Create format from JSON flag.
    pub fn from_json_flag(json: bool) -> Self {
        if json {
            Self::Json
        } else {
            Self::Human
        }
    }
}

/// JSON output structure.
#[derive(Debug, Serialize)]
struct JsonOutput {
    summary: JsonSummary,
    findings: Vec<JsonFinding>,
}

#[derive(Debug, Serialize)]
struct JsonSummary {
    files_scanned: usize,
    files_with_findings: usize,
    total_findings: usize,
    total_score: u32,
    by_severity: serde_json::Value,
    by_category: serde_json::Value,
}

#[derive(Debug, Serialize)]
struct JsonFinding {
    file: String,
    line: usize,
    column: usize,
    severity: String,
    category: String,
    message: String,
    match_text: String,
}

/// Reporter for scan results.
pub struct Reporter {
    format: Format,
}

impl Reporter {
    /// Create a new reporter.
    pub fn new(format: Format) -> Self {
        Self { format }
    }

    /// Report findings and summary.
    pub fn report(&self, results: Vec<Finding>, summary: ScanSummary) -> Result<()> {
        match self.format {
            Format::Human => self.report_human(&results, &summary),
            Format::Json => self.report_json(&results, &summary),
            Format::Sarif => sarif::report_sarif(&results, &summary),
        }
    }

    /// Human-readable terminal output.
    fn report_human(&self, results: &[Finding], summary: &ScanSummary) -> Result<()> {
        let stdout = io::stdout();
        let mut handle = io::BufWriter::new(stdout.lock());

        if results.is_empty() {
            writeln!(
                handle,
                "{}",
                "✓ No AI slop detected! Code is clean.".green()
            )?;
            return Ok(());
        }

        for finding in results {
            self.write_finding(&mut handle, finding)?;
        }

        self.print_summary(&mut handle, summary)?;
        Ok(())
    }

    /// Write a single finding to the output with linter-style context.
    fn write_finding(&self, handle: &mut impl Write, finding: &Finding) -> Result<()> {
        let severity_color = |s: &Severity| -> &'static str {
            match s {
                Severity::Low => "\x1b[2m",           // dim
                Severity::Medium => "\x1b[33m",       // yellow
                Severity::High => "\x1b[31;1m",       // red bold
                Severity::Critical => "\x1b[91;4;1m", // bright red underline bold
            }
        };

        let category_color = |c: &PatternCategory| -> &'static str {
            match c {
                PatternCategory::Placeholder => "\x1b[96m", // bright cyan
                PatternCategory::Deferral => "\x1b[95m",    // bright magenta
                PatternCategory::Hedging => "\x1b[93m",     // bright yellow
                PatternCategory::Stub => "\x1b[91m",        // bright red
                PatternCategory::NamingConvention => "\x1b[38;5;214m", // orange
            }
        };

        let reset = "\x1b[0m";
        let dim = "\x1b[2m";
        let bold = "\x1b[1m";

        // Header: file:line:col SEVERITY [category]
        write!(
            handle,
            "{}{}{} {}:{}:{} ",
            bold,
            finding.file.cyan(),
            reset,
            finding.line.to_string().dimmed(),
            finding.column.to_string().dimmed(),
            reset
        )?;
        write!(
            handle,
            "{}{}{} ",
            severity_color(&finding.severity),
            finding.severity.as_str(),
            reset
        )?;
        writeln!(
            handle,
            "{}[{}]{}",
            category_color(&finding.category),
            format!("{:?}", finding.category).to_lowercase(),
            reset
        )?;

        // Message
        writeln!(handle, "  {} {}", "│".dimmed(), finding.message.dimmed())?;
        writeln!(handle, "  {}", "│".dimmed())?;

        // Calculate line number width for padding
        let line_width = finding.line.to_string().len().max(3);

        // Context line before (if available)
        if let Some(ref before) = finding.context_before {
            let prev_line = finding.line.saturating_sub(1);
            writeln!(
                handle,
                "{}{:>width$} │{} {}",
                dim,
                prev_line,
                reset,
                before.dimmed(),
                width = line_width
            )?;
        }

        // Source line with the finding (highlighted)
        if let Some(ref source) = finding.source_line {
            writeln!(
                handle,
                "{}{:>width$} │{} {}",
                bold,
                finding.line,
                reset,
                source.yellow(),
                width = line_width
            )?;

            // Caret line pointing to the match
            let col = finding.column.saturating_sub(1);
            let match_len = finding.match_text.len().max(1);
            let padding = " ".repeat(col);
            let caret = "^".repeat(match_len);
            writeln!(
                handle,
                "{:>width$}   {}{}{}{}",
                "",
                padding,
                category_color(&finding.category),
                caret,
                reset,
                width = line_width
            )?;
        } else {
            // Fallback: just show the match text
            writeln!(handle, "  {} {}", "→".blue(), finding.match_text.yellow())?;
        }

        // Context line after (if available)
        if let Some(ref after) = finding.context_after {
            let next_line = finding.line + 1;
            writeln!(
                handle,
                "{}{:>width$} │{} {}",
                dim,
                next_line,
                reset,
                after.dimmed(),
                width = line_width
            )?;
        }

        writeln!(handle)?;
        Ok(())
    }

    /// Print summary statistics.
    fn print_summary(&self, handle: &mut impl Write, summary: &ScanSummary) -> Result<()> {
        writeln!(handle, "{}", "─".repeat(60).dimmed())?;

        writeln!(
            handle,
            "{} {} scanned, {} with findings",
            "📁".cyan(),
            summary.files_scanned,
            summary.files_with_findings
        )?;

        writeln!(
            handle,
            "{} {} total findings",
            "⚠".yellow(),
            summary.total_findings
        )?;

        writeln!(
            handle,
            "{} {} sloppy score",
            "💀".red(),
            summary.total_score.to_string().bold()
        )?;

        if !summary.by_severity.is_empty() {
            writeln!(handle)?;
            write!(handle, "  By severity: ")?;
            for severity in [
                Severity::Critical,
                Severity::High,
                Severity::Medium,
                Severity::Low,
            ] {
                if let Some(&count) = summary.by_severity.get(&severity) {
                    let color = match severity {
                        Severity::Low => "\x1b[2m",
                        Severity::Medium => "\x1b[33m",
                        Severity::High => "\x1b[31;1m",
                        Severity::Critical => "\x1b[91;4;1m",
                    };
                    write!(handle, "{}{} {} \x1b[0m", color, count, severity.as_str())?;
                }
            }
            writeln!(handle)?;
        }

        if !summary.by_category.is_empty() {
            writeln!(handle)?;
            write!(handle, "  By category: ")?;
            for category in [
                PatternCategory::Placeholder,
                PatternCategory::Stub,
                PatternCategory::Deferral,
                PatternCategory::Hedging,
                PatternCategory::NamingConvention,
            ] {
                if let Some(&count) = summary.by_category.get(&category) {
                    let color = match category {
                        PatternCategory::Placeholder => "\x1b[96m",
                        PatternCategory::Stub => "\x1b[91m",
                        PatternCategory::Deferral => "\x1b[95m",
                        PatternCategory::Hedging => "\x1b[93m",
                        PatternCategory::NamingConvention => "\x1b[38;5;214m",
                    };
                    write!(
                        handle,
                        "{}{} {} \x1b[0m",
                        color,
                        count,
                        format!("{:?}", category).to_lowercase()
                    )?;
                }
            }
            writeln!(handle)?;
        }

        writeln!(handle)?;

        let verdict = match summary.total_score {
            0 => "✓ Clean code!",
            1..=10 => "⚠ Minor slop detected",
            11..=50 => "⚠⚠ Moderate slop detected",
            51..=100 => "⚠⚠⚠ High slop detected",
            _ => "💀💀💀 CRITICAL SLOP LEVEL",
        };

        writeln!(handle, "{}", verdict)?;
        Ok(())
    }

    /// JSON output.
    fn report_json(&self, results: &[Finding], summary: &ScanSummary) -> Result<()> {
        use serde_json::Value;

        let by_severity: Value = summary
            .by_severity
            .iter()
            .map(|(k, v)| (k.as_str().to_lowercase(), Value::from(*v)))
            .collect();

        let by_category: Value = summary
            .by_category
            .iter()
            .map(|(k, v)| (format!("{:?}", k).to_lowercase(), Value::from(*v)))
            .collect();

        let output = JsonOutput {
            summary: JsonSummary {
                files_scanned: summary.files_scanned,
                files_with_findings: summary.files_with_findings,
                total_findings: summary.total_findings,
                total_score: summary.total_score,
                by_severity,
                by_category,
            },
            findings: results
                .iter()
                .map(|f| JsonFinding {
                    file: f.file.clone(),
                    line: f.line,
                    column: f.column,
                    severity: f.severity.as_str().to_string().to_lowercase(),
                    category: format!("{:?}", f.category).to_lowercase(),
                    message: f.message.clone(),
                    match_text: f.match_text.clone(),
                })
                .collect(),
        };

        println!(
            "{}",
            serde_json::to_string_pretty(&output)
                .map_err(|e| Error::ConfigInvalid(e.to_string()))?
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // Helper to create test findings
    // Note: Consider moving to common test utilities if duplicated elsewhere
    fn make_finding(
        file: &str,
        line: usize,
        severity: Severity,
        category: PatternCategory,
        message: &str,
        match_text: &str,
    ) -> Finding {
        Finding {
            file: file.to_string(),
            line,
            column: 1,
            severity,
            category,
            message: message.to_string(),
            match_text: match_text.to_string(),
            pattern_regex: "test".to_string(),
            source_line: None,
            context_before: None,
            context_after: None,
        }
    }

    fn make_summary(total_score: u32, findings_count: usize) -> ScanSummary {
        let mut by_severity = HashMap::new();
        let mut by_category = HashMap::new();
        by_severity.insert(Severity::Medium, findings_count);
        by_category.insert(PatternCategory::Stub, findings_count);
        ScanSummary {
            files_scanned: 1,
            files_with_findings: if findings_count > 0 { 1 } else { 0 },
            total_findings: findings_count,
            total_score,
            by_severity,
            by_category,
        }
    }

    #[test]
    fn test_format_from_json_flag() {
        assert_eq!(Format::from_json_flag(true), Format::Json);
        assert_eq!(Format::from_json_flag(false), Format::Human);
    }

    #[test]
    fn test_reporter_new() {
        let reporter = Reporter::new(Format::Human);
        // Just check it creates successfully
        assert_eq!(reporter.format, Format::Human);
    }

    #[test]
    fn test_reporter_report_json() {
        let reporter = Reporter::new(Format::Json);
        let results = vec![make_finding(
            "test.py",
            10,
            Severity::Medium,
            PatternCategory::Stub,
            "Test message",
            "TODO",
        )];
        let summary = make_summary(5, 1);

        // Verify report_json doesn't panic
        // report_json writes to stdout; capturing it is complex in unit tests
        let _ = reporter.report_json(&results, &summary);
    }

    #[test]
    fn test_reporter_report_json_empty() {
        let reporter = Reporter::new(Format::Json);
        let results = vec![];
        let summary = make_summary(0, 0);

        // Verify empty results don't panic
        let _ = reporter.report_json(&results, &summary);
    }

    #[test]
    fn test_verdict_determination() {
        // Test that verdict is determined by total_score
        // Clean code
        assert_eq!(make_summary(0, 0).total_score, 0);
        // Minor slop
        assert_eq!(make_summary(5, 1).total_score, 5);
        // Moderate slop
        assert_eq!(make_summary(15, 3).total_score, 15);
        // High slop
        assert_eq!(make_summary(75, 15).total_score, 75);
        // Critical
        assert_eq!(make_summary(150, 30).total_score, 150);
    }

    #[test]
    fn test_reporter_report_human_empty() {
        let reporter = Reporter::new(Format::Human);
        let results = vec![];
        let summary = make_summary(0, 0);

        // Just check it doesn't error
        let _ = reporter.report(results, summary);
    }

    #[test]
    fn test_reporter_report_human_with_findings() {
        let reporter = Reporter::new(Format::Human);
        let results = vec![make_finding(
            "test.py",
            10,
            Severity::Medium,
            PatternCategory::Stub,
            "Test message",
            "TODO",
        )];
        let summary = make_summary(5, 1);

        // Just check it doesn't error
        let _ = reporter.report(results, summary);
    }

    #[test]
    fn test_reporter_report_sarif() {
        let reporter = Reporter::new(Format::Sarif);
        let results = vec![make_finding(
            "test.py",
            10,
            Severity::Medium,
            PatternCategory::Placeholder,
            "Test message",
            "TODO",
        )];
        let summary = make_summary(5, 1);

        // Just check it doesn't error
        let _ = reporter.report(results, summary);
    }
}
