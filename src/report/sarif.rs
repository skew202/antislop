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
