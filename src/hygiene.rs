//! Code hygiene survey functionality.
//!
//! Helps developers discover appropriate linters, formatters, and CI/CD tools
//! for their project by detecting project types and existing tooling.
//!
//! Tool definitions are loaded from `data/hygiene_tools.toml` for maintainability.

#![allow(clippy::write_literal)]

use owo_colors::OwoColorize;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

/// Embedded tool definitions from TOML
const TOOLS_TOML: &str = include_str!("../data/hygiene_tools.toml");

// ============================================================================
// Data Structures (deserialized from TOML)
// ============================================================================

/// A tool (linter or formatter) definition.
#[derive(Debug, Clone, Deserialize)]
pub struct ToolDef {
    pub name: String,
    pub install: String,
    pub description: String,
    #[serde(default)]
    pub config_files: Vec<String>,
}

/// Language definition with associated tools.
#[derive(Debug, Clone, Deserialize)]
pub struct LanguageDef {
    pub name: String,
    #[serde(default)]
    pub marker_files: Vec<String>,
    #[serde(default)]
    pub detect_extensions: Vec<String>,
    #[serde(default)]
    pub linters: Vec<ToolDef>,
    #[serde(default)]
    pub formatters: Vec<ToolDef>,
}

/// Detection patterns for finding tools by content.
#[derive(Debug, Clone, Deserialize)]
pub struct DetectionPattern {
    pub tool_name: String,
    pub is_linter: bool,
}

/// Root structure for the TOML file.
#[derive(Debug, Clone, Deserialize)]
struct ToolsConfig {
    #[serde(flatten)]
    pub languages: HashMap<String, LanguageDef>,
    #[serde(default)]
    pub detection_patterns: HashMap<String, (String, bool)>,
}

// ============================================================================
// Public API Types
// ============================================================================

/// A tool found or recommended.
#[derive(Debug, Clone)]
pub struct Tool {
    pub name: String,
    pub install_cmd: String,
    pub description: String,
    pub found: bool,
}

impl Tool {
    pub fn new(name: &str, install_cmd: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            install_cmd: install_cmd.to_string(),
            description: description.to_string(),
            found: false,
        }
    }

    fn from_def(def: &ToolDef) -> Self {
        Self {
            name: def.name.clone(),
            install_cmd: def.install.clone(),
            description: def.description.clone(),
            found: false,
        }
    }
}

/// CI/CD platform detection.
#[derive(Debug, Clone)]
pub struct CIPipeline {
    pub platform: String,
    pub config_file: String,
}

/// Result of a hygiene survey.
#[derive(Debug)]
pub struct HygieneSurvey {
    /// Detected project types and their marker file.
    pub project_types: Vec<(String, String)>,
    /// Linters found in the project.
    pub linters_found: Vec<String>,
    /// Formatters found in the project.
    pub formatters_found: Vec<String>,
    /// CI/CD pipelines found.
    pub ci_pipelines: Vec<CIPipeline>,
    /// Pre-commit config found.
    pub precommit_found: Option<String>,
    /// Root path surveyed.
    pub root_path: PathBuf,
    /// Recommended tools per language.
    pub recommendations: HashMap<String, Vec<Tool>>,
}

// ============================================================================
// Core Functions
// ============================================================================

/// Load tool definitions from embedded TOML.
fn load_tools_config() -> ToolsConfig {
    toml::from_str(TOOLS_TOML).expect("Failed to parse hygiene_tools.toml")
}

/// Run a hygiene survey on the given paths.
pub fn run_survey(paths: &[PathBuf]) -> HygieneSurvey {
    let root = paths.first().cloned().unwrap_or_else(|| PathBuf::from("."));
    let config = load_tools_config();

    let mut project_types: Vec<(String, String)> = Vec::new();
    let mut linters_found: HashSet<String> = HashSet::new();
    let mut formatters_found: HashSet<String> = HashSet::new();
    let mut ci_pipelines: Vec<CIPipeline> = Vec::new();
    let mut precommit_found: Option<String> = None;
    let mut recommendations: HashMap<String, Vec<Tool>> = HashMap::new();

    // Detect project types from marker files
    for (lang_key, lang_def) in &config.languages {
        // Skip detection_patterns pseudo-key
        if lang_key == "detection_patterns" {
            continue;
        }

        // Check marker files
        for marker in &lang_def.marker_files {
            let marker_path = root.join(marker);
            if marker_path.exists() {
                project_types.push((lang_def.name.clone(), marker.clone()));
                break;
            }
        }

        // Check extensions (for C#)
        if !lang_def.detect_extensions.is_empty() {
            if let Ok(entries) = fs::read_dir(&root) {
                for entry in entries.flatten() {
                    if let Some(ext) = entry.path().extension() {
                        if lang_def.detect_extensions.iter().any(|e| ext == e.as_str()) {
                            project_types.push((
                                lang_def.name.clone(),
                                format!("*.{}", ext.to_string_lossy()),
                            ));
                            break;
                        }
                    }
                }
            }
        }
    }

    // Detect linter/formatter configs
    for lang_def in config.languages.values() {
        for tool in &lang_def.linters {
            for config_file in &tool.config_files {
                if root.join(config_file).exists() {
                    linters_found.insert(tool.name.clone());
                    break;
                }
            }
        }
        for tool in &lang_def.formatters {
            for config_file in &tool.config_files {
                if root.join(config_file).exists() {
                    formatters_found.insert(tool.name.clone());
                    break;
                }
            }
        }
    }

    // Check pyproject.toml for Python tooling sections
    let pyproject = root.join("pyproject.toml");
    if pyproject.exists() {
        if let Ok(content) = fs::read_to_string(&pyproject) {
            if content.contains("[tool.ruff]") {
                linters_found.insert("ruff".to_string());
            }
            if content.contains("[tool.black]") {
                formatters_found.insert("black".to_string());
            }
            if content.contains("[tool.mypy]") {
                linters_found.insert("mypy".to_string());
            }
            if content.contains("[tool.pylint]") {
                linters_found.insert("pylint".to_string());
            }
            if content.contains("[tool.isort]") {
                formatters_found.insert("isort".to_string());
            }
        }
    }

    // Detect CI/CD pipelines and scan for tool usage
    // GitHub Actions
    let gh_workflows = root.join(".github/workflows");
    if gh_workflows.exists() {
        if let Ok(entries) = fs::read_dir(&gh_workflows) {
            let mut yml_count = 0;
            for entry in entries.flatten() {
                let ext_match = entry
                    .path()
                    .extension()
                    .map(|ext| ext == "yml" || ext == "yaml")
                    .unwrap_or(false);
                if ext_match {
                    yml_count += 1;
                    if let Ok(content) = fs::read_to_string(entry.path()) {
                        detect_tools_in_content(
                            &content,
                            &config.detection_patterns,
                            &mut linters_found,
                            &mut formatters_found,
                        );
                    }
                }
            }
            if yml_count > 0 {
                ci_pipelines.push(CIPipeline {
                    platform: "GitHub Actions".to_string(),
                    config_file: format!("{} workflow(s)", yml_count),
                });
            }
        }
    }

    // GitLab CI
    if root.join(".gitlab-ci.yml").exists() {
        ci_pipelines.push(CIPipeline {
            platform: "GitLab CI".to_string(),
            config_file: ".gitlab-ci.yml".to_string(),
        });
        if let Ok(content) = fs::read_to_string(root.join(".gitlab-ci.yml")) {
            detect_tools_in_content(
                &content,
                &config.detection_patterns,
                &mut linters_found,
                &mut formatters_found,
            );
        }
    }

    // Jenkins
    if root.join("Jenkinsfile").exists() {
        ci_pipelines.push(CIPipeline {
            platform: "Jenkins".to_string(),
            config_file: "Jenkinsfile".to_string(),
        });
    }

    // CircleCI
    if root.join(".circleci/config.yml").exists() {
        ci_pipelines.push(CIPipeline {
            platform: "CircleCI".to_string(),
            config_file: ".circleci/config.yml".to_string(),
        });
    }

    // Check scripts directory for tool usage
    let scripts_dir = root.join("scripts");
    if scripts_dir.exists() {
        if let Ok(entries) = fs::read_dir(&scripts_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map(|e| e == "sh").unwrap_or(false) {
                    if let Ok(content) = fs::read_to_string(&path) {
                        detect_tools_in_content(
                            &content,
                            &config.detection_patterns,
                            &mut linters_found,
                            &mut formatters_found,
                        );
                    }
                }
            }
        }
    }

    // Check Makefiles
    for makefile in ["Makefile", "makefile", "GNUmakefile"] {
        let makefile_path = root.join(makefile);
        if makefile_path.exists() {
            if let Ok(content) = fs::read_to_string(&makefile_path) {
                detect_tools_in_content(
                    &content,
                    &config.detection_patterns,
                    &mut linters_found,
                    &mut formatters_found,
                );
            }
        }
    }

    // Pre-commit hooks
    if root.join(".pre-commit-config.yaml").exists() {
        precommit_found = Some(".pre-commit-config.yaml".to_string());
    } else if root.join(".husky").exists() {
        precommit_found = Some(".husky/".to_string());
    } else if root.join("lefthook.yml").exists() {
        precommit_found = Some("lefthook.yml".to_string());
    }

    // Build recommendations (tools not found)
    for (lang_name, _marker) in &project_types {
        // Find the language definition
        for lang_def in config.languages.values() {
            if &lang_def.name == lang_name {
                let mut missing_tools: Vec<Tool> = Vec::new();

                // Check linters
                for tool in &lang_def.linters {
                    if !linters_found
                        .iter()
                        .any(|l| l.to_lowercase().contains(&tool.name.to_lowercase()))
                    {
                        missing_tools.push(Tool::from_def(tool));
                    }
                }

                // Check formatters
                for tool in &lang_def.formatters {
                    if !formatters_found
                        .iter()
                        .any(|f| f.to_lowercase().contains(&tool.name.to_lowercase()))
                    {
                        missing_tools.push(Tool::from_def(tool));
                    }
                }

                if !missing_tools.is_empty() {
                    recommendations.insert(lang_name.clone(), missing_tools);
                }
                break;
            }
        }
    }

    HygieneSurvey {
        project_types,
        linters_found: linters_found.into_iter().collect(),
        formatters_found: formatters_found.into_iter().collect(),
        ci_pipelines,
        precommit_found,
        root_path: root,
        recommendations,
    }
}

/// Detect tools from content by pattern matching.
fn detect_tools_in_content(
    content: &str,
    patterns: &HashMap<String, (String, bool)>,
    linters: &mut HashSet<String>,
    formatters: &mut HashSet<String>,
) {
    let content_lower = content.to_lowercase();
    for (pattern, (tool_name, is_linter)) in patterns {
        if content_lower.contains(&pattern.to_lowercase()) {
            if *is_linter {
                linters.insert(tool_name.clone());
            } else {
                formatters.insert(tool_name.clone());
            }
        }
    }
}

// ============================================================================
// Report Output
// ============================================================================

/// Print the hygiene survey report with rich TUI formatting.
pub fn print_report(survey: &HygieneSurvey) {
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout.lock());
    let _ = print_report_to(&mut handle, survey);
}

fn print_report_to(handle: &mut impl Write, survey: &HygieneSurvey) -> io::Result<()> {
    let box_width = 68;
    let amber = "\x1b[38;5;214m";
    let reset = "\x1b[0m";
    let bold = "\x1b[1m";

    // Helper to draw a box line
    let draw_box_top = |h: &mut dyn Write| -> io::Result<()> {
        writeln!(h, "{}┌{}┐{}", amber, "─".repeat(box_width), reset)
    };
    let draw_box_bottom = |h: &mut dyn Write| -> io::Result<()> {
        writeln!(h, "{}└{}┘{}", amber, "─".repeat(box_width), reset)
    };
    let draw_box_section = |h: &mut dyn Write, title: &str| -> io::Result<()> {
        let padding = box_width - title.len() - 3;
        writeln!(h, "{}┌─ {} {}┐{}", amber, title, "─".repeat(padding), reset)
    };

    // Title box
    draw_box_top(handle as &mut dyn Write)?;
    let title = "CODE   HYGIENE   SURVEY";
    let pad_left = (box_width - title.len()) / 2;
    writeln!(
        handle,
        "{}│{}{}{}{}│{}",
        amber,
        " ".repeat(pad_left),
        bold,
        title,
        " ".repeat(box_width - pad_left - title.len()),
        reset
    )?;
    let path_str = survey.root_path.display().to_string();
    let path_line = format!("for {}", path_str);
    let pad_left = (box_width - path_line.len()) / 2;
    writeln!(
        handle,
        "{}│{}{}{}│{}",
        amber,
        " ".repeat(pad_left),
        path_line.dimmed(),
        " ".repeat(box_width - pad_left - path_line.len()),
        reset
    )?;
    draw_box_bottom(handle as &mut dyn Write)?;
    writeln!(handle)?;

    // Project Detection
    draw_box_section(handle as &mut dyn Write, "PROJECT DETECTION")?;
    writeln!(handle, "{}│{}│{}", amber, " ".repeat(box_width), reset)?;

    if survey.project_types.is_empty() {
        writeln!(
            handle,
            "{}│  {}No project markers found. Add Cargo.toml, package.json, etc.{}{}│{}",
            amber,
            "\x1b[2m",
            reset,
            " ".repeat(box_width - 60),
            reset
        )?;
    } else {
        for (lang_name, marker) in &survey.project_types {
            let line = format!("  {} PROJECT", lang_name.to_uppercase());
            let marker_str = format!("{} found", marker);
            let padding = box_width - line.len() - marker_str.len() - 2;
            writeln!(
                handle,
                "{}│{}{}{}{}  {}│{}",
                amber,
                bold,
                line,
                reset,
                " ".repeat(padding),
                marker_str.dimmed(),
                reset
            )?;

            // Progress bar
            let bar_width = 48;
            let bar = "█".repeat(bar_width);
            let percent = "100%";
            let pad = box_width - bar_width - 10 - percent.len();
            writeln!(
                handle,
                "{}│  {}{}{}  {}{}│{}",
                amber,
                amber,
                bar,
                reset,
                percent,
                " ".repeat(pad),
                reset
            )?;
        }
    }
    writeln!(handle, "{}│{}│{}", amber, " ".repeat(box_width), reset)?;
    draw_box_bottom(handle as &mut dyn Write)?;
    writeln!(handle)?;

    // Linters & Formatters
    draw_box_section(handle as &mut dyn Write, "LINTERS & FORMATTERS")?;
    writeln!(handle, "{}│{}│{}", amber, " ".repeat(box_width), reset)?;

    if survey.linters_found.is_empty() && survey.formatters_found.is_empty() {
        writeln!(
            handle,
            "{}│  {}✗ No linter or formatter configs found{}{}│{}",
            amber,
            "\x1b[31m",
            reset,
            " ".repeat(box_width - 43),
            reset
        )?;
    } else {
        for linter in &survey.linters_found {
            let line = format!("  ✓ {}", linter);
            let pad = box_width - line.len() - 20;
            writeln!(
                handle,
                "{}│{}\x1b[32m{}{}{} {}│{}",
                amber,
                "",
                line,
                reset,
                " ".repeat(pad),
                "linter".dimmed(),
                reset
            )?;
        }
        for formatter in &survey.formatters_found {
            let line = format!("  ✓ {}", formatter);
            let pad = box_width - line.len() - 20;
            writeln!(
                handle,
                "{}│{}\x1b[32m{}{}{} {}│{}",
                amber,
                "",
                line,
                reset,
                " ".repeat(pad),
                "formatter".dimmed(),
                reset
            )?;
        }
    }

    writeln!(handle, "{}│{}│{}", amber, " ".repeat(box_width), reset)?;
    draw_box_bottom(handle as &mut dyn Write)?;
    writeln!(handle)?;

    // CI/CD Pipelines
    draw_box_section(handle as &mut dyn Write, "CI/CD PIPELINES")?;
    writeln!(handle, "{}│{}│{}", amber, " ".repeat(box_width), reset)?;

    if survey.ci_pipelines.is_empty() {
        writeln!(
            handle,
            "{}│  {}✗ No CI/CD configuration found{}{}│{}",
            amber,
            "\x1b[31m",
            reset,
            " ".repeat(box_width - 36),
            reset
        )?;
    } else {
        for ci in &survey.ci_pipelines {
            let line = format!("  {}", ci.platform);
            let config = &ci.config_file;
            let pad = box_width - line.len() - config.len() - 4;
            writeln!(
                handle,
                "{}│{}{}{}{}  {}│{}",
                amber,
                bold,
                line,
                reset,
                " ".repeat(pad),
                config.dimmed(),
                reset
            )?;
        }
    }

    writeln!(handle, "{}│{}│{}", amber, " ".repeat(box_width), reset)?;
    draw_box_bottom(handle as &mut dyn Write)?;
    writeln!(handle)?;

    // Pre-commit Hooks
    draw_box_section(handle as &mut dyn Write, "PRE-COMMIT HOOKS")?;
    writeln!(handle, "{}│{}│{}", amber, " ".repeat(box_width), reset)?;

    if let Some(ref config) = survey.precommit_found {
        let line = format!("  ✓ {}", config);
        let pad = box_width - line.len() - 2;
        writeln!(
            handle,
            "{}│{}\x1b[32m{}{}{}│{}",
            amber,
            "",
            line,
            reset,
            " ".repeat(pad),
            reset
        )?;
    } else {
        writeln!(
            handle,
            "{}│  {}✗ No pre-commit hooks found{}{}│{}",
            amber,
            "\x1b[31m",
            reset,
            " ".repeat(box_width - 32),
            reset
        )?;
    }

    writeln!(handle, "{}│{}│{}", amber, " ".repeat(box_width), reset)?;
    draw_box_bottom(handle as &mut dyn Write)?;
    writeln!(handle)?;

    // Recommendations
    draw_box_section(handle as &mut dyn Write, "RECOMMENDATIONS")?;
    writeln!(handle, "{}│{}│{}", amber, " ".repeat(box_width), reset)?;

    if survey.recommendations.is_empty() {
        let line = "  ✓ All recommended tools configured!";
        let pad = box_width - line.len() - 2;
        writeln!(
            handle,
            "{}│{}\x1b[32m{}{}{}│{}",
            amber,
            "",
            line,
            reset,
            " ".repeat(pad),
            reset
        )?;
    } else {
        for (lang_name, tools) in &survey.recommendations {
            let header = format!("  {}", lang_name);
            let pad = box_width - header.len() - 2;
            writeln!(
                handle,
                "{}│{}{}{}{}│{}",
                amber,
                bold,
                header,
                reset,
                " ".repeat(pad),
                reset
            )?;

            // Show top 3 recommendations per language
            for tool in tools.iter().take(3) {
                let line = format!(
                    "  • Add {} for {}",
                    tool.name,
                    tool.description.to_lowercase()
                );
                let line = if line.len() > box_width - 4 {
                    format!("{}...", &line[..box_width - 7])
                } else {
                    line
                };
                let pad = box_width - line.len() - 2;
                writeln!(
                    handle,
                    "{}│{}{}{}│{}",
                    amber,
                    line.dimmed(),
                    reset,
                    " ".repeat(pad),
                    reset
                )?;
            }
            writeln!(handle, "{}│{}│{}", amber, " ".repeat(box_width), reset)?;
        }
    }

    // Tips for empty setups
    if survey.precommit_found.is_none() && !survey.project_types.is_empty() {
        let line = "  💡 Consider adding pre-commit hooks for automated checks";
        let pad = box_width - line.len() - 2;
        writeln!(
            handle,
            "{}│{}{}{}│{}",
            amber,
            line.dimmed(),
            reset,
            " ".repeat(pad),
            reset
        )?;
    }

    if survey.ci_pipelines.is_empty() && !survey.project_types.is_empty() {
        let line = "  💡 Consider adding GitHub Actions or other CI/CD";
        let pad = box_width - line.len() - 2;
        writeln!(
            handle,
            "{}│{}{}{}│{}",
            amber,
            line.dimmed(),
            reset,
            " ".repeat(pad),
            reset
        )?;
    }

    draw_box_bottom(handle as &mut dyn Write)?;

    Ok(())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_tools_config() {
        let config = load_tools_config();
        // Should have multiple languages
        assert!(config.languages.len() >= 10);
        // Should have detection patterns
        assert!(!config.detection_patterns.is_empty());
    }

    #[test]
    fn test_rust_tools_present() {
        let config = load_tools_config();
        let rust = config.languages.get("rust").expect("Should have Rust");
        assert_eq!(rust.name, "Rust");
        assert!(!rust.linters.is_empty());
        assert!(!rust.formatters.is_empty());

        // Check clippy is present
        assert!(rust.linters.iter().any(|t| t.name == "clippy"));
        // Check rustfmt is present
        assert!(rust.formatters.iter().any(|t| t.name == "rustfmt"));
    }

    #[test]
    fn test_python_tools_present() {
        let config = load_tools_config();
        let python = config.languages.get("python").expect("Should have Python");
        assert_eq!(python.name, "Python");

        // Check key Python tools
        assert!(python.linters.iter().any(|t| t.name == "ruff"));
        assert!(python.linters.iter().any(|t| t.name == "mypy"));
        assert!(python.linters.iter().any(|t| t.name == "pylint"));
        assert!(python.formatters.iter().any(|t| t.name == "black"));
        assert!(python.formatters.iter().any(|t| t.name == "isort"));
    }

    #[test]
    fn test_tool_creation() {
        let tool = Tool::new("test", "install cmd", "description");
        assert_eq!(tool.name, "test");
        assert!(!tool.found);
    }
}
