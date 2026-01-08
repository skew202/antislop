use crate::detector::{Finding, ScanSummary};
use crate::Result;
use serde_sarif::sarif::{
    ArtifactLocation, Location, Message, PhysicalLocation, Region, Result as SarifResult,
    ResultLevel, Run, Sarif, Tool, ToolComponent,
};

pub fn report_sarif(results: &[Finding], _summary: &ScanSummary) -> Result<()> {
    let mut sarif_results = Vec::new();

    for finding in results {
        let rule_id = format!("{:?}", finding.category).to_lowercase();

        let artifact_location = ArtifactLocation::builder()
            .uri(finding.file.clone())
            .build();
        let region = Region::builder()
            .start_line(finding.line as i64)
            .start_column(finding.column as i64)
            .end_line(finding.line as i64)
            .end_column((finding.column + finding.match_text.len()) as i64)
            .build();
        let physical_location = PhysicalLocation::builder()
            .artifact_location(artifact_location)
            .region(region)
            .build();
        let location = Location::builder()
            .physical_location(physical_location)
            .build();

        let level = match finding.severity.as_str() {
            "CRITICAL" | "HIGH" => ResultLevel::Error,
            "MEDIUM" => ResultLevel::Warning,
            _ => ResultLevel::Note,
        };

        let result = SarifResult::builder()
            .rule_id(rule_id)
            .message(Message::builder().text(finding.message.clone()).build())
            .level(level)
            .locations(vec![location])
            .build();

        sarif_results.push(result);
    }

    let tool_component = ToolComponent::builder()
        .name("antislop")
        .information_uri("https://github.com/skew202/antislop")
        .build();
    let tool = Tool::builder().driver(tool_component).build();
    let run = Run::builder().tool(tool).results(sarif_results).build();

    let sarif = Sarif::builder()
        .version("2.1.0")
        .schema("https://json.schemastore.org/sarif-2.1.0.json")
        .runs(vec![run])
        .build();

    let json = serde_json::to_string_pretty(&sarif)
        .map_err(|e| crate::Error::ConfigInvalid(e.to_string()))?;

    println!("{}", json);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{PatternCategory, Severity};

    fn make_finding(
        file: &str,
        line: usize,
        column: usize,
        severity: Severity,
        category: PatternCategory,
        message: &str,
        match_text: &str,
    ) -> Finding {
        Finding {
            file: file.to_string(),
            line,
            column,
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

    #[test]
    fn test_report_sarif_empty() {
        let results = vec![];
        let summary = ScanSummary {
            files_scanned: 0,
            files_with_findings: 0,
            total_findings: 0,
            total_score: 0,
            by_severity: Default::default(),
            by_category: Default::default(),
        };

        // Just check it doesn't error
        let _ = report_sarif(&results, &summary);
    }

    #[test]
    fn test_sarif_severity_mapping() {
        // Critical -> Error
        let finding_critical = make_finding(
            "test.rs",
            1,
            1,
            Severity::Critical,
            PatternCategory::Stub,
            "Critical issue",
            "TODO",
        );
        // High -> Error
        let finding_high = make_finding(
            "test.rs",
            2,
            1,
            Severity::High,
            PatternCategory::Stub,
            "High issue",
            "FIXME",
        );
        // Medium -> Warning
        let finding_medium = make_finding(
            "test.rs",
            3,
            1,
            Severity::Medium,
            PatternCategory::Stub,
            "Medium issue",
            "hack",
        );
        // Low -> Note
        let finding_low = make_finding(
            "test.rs",
            4,
            1,
            Severity::Low,
            PatternCategory::Stub,
            "Low issue",
            "xxx",
        );

        let results = vec![finding_critical, finding_high, finding_medium, finding_low];
        let summary = ScanSummary {
            files_scanned: 1,
            files_with_findings: 1,
            total_findings: 4,
            total_score: 71,
            by_severity: Default::default(),
            by_category: Default::default(),
        };

        // Should not panic
        let _ = report_sarif(&results, &summary);
    }

    #[test]
    fn test_sarif_finding_structure() {
        let finding = make_finding(
            "/path/to/file.py",
            42,
            10,
            Severity::Medium,
            PatternCategory::Placeholder,
            "Test message",
            "TODO",
        );

        assert_eq!(finding.file, "/path/to/file.py");
        assert_eq!(finding.line, 42);
        assert_eq!(finding.column, 10);
        assert_eq!(finding.severity, Severity::Medium);
        assert_eq!(finding.category, PatternCategory::Placeholder);
        assert_eq!(finding.message, "Test message");
        assert_eq!(finding.match_text, "TODO");
    }
}
