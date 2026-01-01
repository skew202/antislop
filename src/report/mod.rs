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

    /// Write a single finding to the output.
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
            }
        };

        let reset = "\x1b[0m";

        writeln!(
            handle,
            "{}{}:{}:{}: ",
            reset,
            finding.file.cyan(),
            finding.line.to_string().dimmed(),
            finding.column.to_string().dimmed()
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

        writeln!(handle, "    {} {}", "!".yellow(), finding.message.dimmed())?;

        writeln!(handle, "    {} {}", "→".blue(), finding.match_text.yellow())?;

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
            ] {
                if let Some(&count) = summary.by_category.get(&category) {
                    let color = match category {
                        PatternCategory::Placeholder => "\x1b[96m",
                        PatternCategory::Stub => "\x1b[91m",
                        PatternCategory::Deferral => "\x1b[95m",
                        PatternCategory::Hedging => "\x1b[93m",
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

    #[test]
    fn test_format_from_json_flag() {
        assert_eq!(Format::from_json_flag(true), Format::Json);
        assert_eq!(Format::from_json_flag(false), Format::Human);
    }
}
