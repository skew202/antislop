use crate::detector::{Finding, ScanSummary};
use crate::Result;
use serde_sarif::sarif::{
    ArtifactLocationBuilder, LocationBuilder, MessageBuilder, PhysicalLocationBuilder,
    RegionBuilder, ResultBuilder, RunBuilder, SarifBuilder, ToolBuilder, ToolComponentBuilder,
};

pub fn report_sarif(results: &[Finding], _summary: &ScanSummary) -> Result<()> {
    let mut sarif_results = Vec::new();

    for finding in results {
        let rule_id = format!("{:?}", finding.category).to_lowercase();
        let map_err = |e: String| crate::Error::ConfigInvalid(e);

        let location = LocationBuilder::default()
            .physical_location(
                PhysicalLocationBuilder::default()
                    .artifact_location(
                        ArtifactLocationBuilder::default()
                            .uri(&finding.file)
                            .build()
                            .map_err(|e| map_err(e.to_string()))?,
                    )
                    .region(
                        RegionBuilder::default()
                            .start_line(finding.line as i64)
                            .start_column(finding.column as i64)
                            .end_line(finding.line as i64)
                            .end_column((finding.column + finding.match_text.len()) as i64)
                            .build()
                            .map_err(|e| map_err(e.to_string()))?,
                    )
                    .build()
                    .map_err(|e| map_err(e.to_string()))?,
            )
            .build()
            .map_err(|e| map_err(e.to_string()))?;

        let result = ResultBuilder::default()
            .rule_id(&rule_id)
            .message(
                MessageBuilder::default()
                    .text(&finding.message)
                    .build()
                    .map_err(|e| map_err(e.to_string()))?,
            )
            .level(match finding.severity.as_str() {
                "CRITICAL" | "HIGH" => "error",
                "MEDIUM" => "warning",
                _ => "note",
            })
            .locations(vec![location])
            .build()
            .map_err(|e| map_err(e.to_string()))?;

        sarif_results.push(result);
    }

    let run = RunBuilder::default()
        .tool(
            ToolBuilder::default()
                .driver(
                    ToolComponentBuilder::default()
                        .name("antislop")
                        .information_uri("https://github.com/skew202/antislop")
                        .build()
                        .map_err(|e| crate::Error::ConfigInvalid(e.to_string()))?,
                )
                .build()
                .map_err(|e| crate::Error::ConfigInvalid(e.to_string()))?,
        )
        .results(sarif_results)
        .build()
        .map_err(|e| crate::Error::ConfigInvalid(e.to_string()))?;

    let sarif = SarifBuilder::default()
        .version("2.1.0")
        .schema("https://json.schemastore.org/sarif-2.1.0.json")
        .runs(vec![run])
        .build()
        .map_err(|e| crate::Error::ConfigInvalid(e.to_string()))?;

    let json = serde_json::to_string_pretty(&sarif)
        .map_err(|e| crate::Error::ConfigInvalid(e.to_string()))?;

    println!("{}", json);
    Ok(())
}
