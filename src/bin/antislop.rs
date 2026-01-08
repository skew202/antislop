//! AntiSlop - AI Slop Linter
//!
//! A blazing-fast, multi-language linter for detecting AI-generated code slop.

use antislop::{
    Config, FilenameCheckConfig, FilenameChecker, Format, Profile, ProfileLoader, ProfileSource,
    Reporter, Scanner, Walker, CONFIG_FILES, VERSION,
};
use anyhow::{Context, Result};
use clap::{CommandFactory, Parser};
use clap_complete::{generate, Shell};
use std::fs;
use std::io;
use std::path::PathBuf;

/// AntiSlop - A blazing-fast linter for detecting AI-generated code slop.
#[derive(Parser, Debug)]
#[command(name = "antislop")]
#[command(author = "AntiSlop Contributors")]
#[command(version = VERSION)]
#[command(about = "Detect AI-generated code slop: placeholders, hedging, stubs, and deferrals", long_about = None)]
#[command(propagate_version = true)]
struct Args {
    /// Path(s) to scan (defaults to current directory)
    #[arg(value_name = "PATH", default_value = ".")]
    paths: Vec<PathBuf>,

    /// Path to config file
    #[arg(short, long, value_name = "FILE", global = false)]
    config: Option<PathBuf>,

    /// Output in JSON format
    #[arg(long)]
    json: bool,

    /// Maximum file size to scan (KB)
    #[arg(short, long, default_value = "1024", global = false)]
    max_size: u64,

    /// File extensions to scan (comma-separated)
    #[arg(short, long, value_delimiter = ',')]
    extensions: Option<Vec<String>>,

    /// Verbose output (-v, -vv, -vvv)
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// Generate shell completions
    #[arg(long, value_name = "SHELL")]
    completions: Option<Shell>,

    /// List all supported languages
    #[arg(long)]
    list_languages: bool,

    /// Output format (human, json, sarif)
    #[arg(long, value_name = "FORMAT")]
    format: Option<String>,

    /// Print default configuration
    #[arg(long)]
    print_config: bool,

    /// Disable filename convention checking
    #[arg(long)]
    no_filename_check: bool,

    /// Load a community profile (file path, URL, or profile name)
    #[arg(long, value_name = "PROFILE")]
    profile: Option<String>,

    /// Print available profiles
    #[arg(long)]
    list_profiles: bool,

    /// Disable pattern categories (comma-separated: placeholder,stub,deferral,hedging)
    #[arg(long, value_delimiter = ',', value_name = "CATEGORIES")]
    disable: Option<Vec<String>>,

    /// Only enable specific categories (comma-separated: placeholder,stub,deferral,hedging)
    #[arg(long, value_delimiter = ',', value_name = "CATEGORIES")]
    only: Option<Vec<String>>,

    /// Run a code hygiene survey (detect project types, suggest linters/formatters)
    #[arg(long)]
    hygiene_survey: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.list_languages {
        print_languages();
        return Ok(());
    }

    if args.print_config {
        print_default_config();
        return Ok(());
    }

    if args.list_profiles {
        print_profiles()?;
        return Ok(());
    }

    if let Some(shell) = args.completions {
        generate_completions(shell);
        return Ok(());
    }

    // Run hygiene survey if requested
    if args.hygiene_survey {
        let survey = antislop::hygiene::run_survey(&args.paths);
        antislop::hygiene::print_report(&survey);
        return Ok(());
    }

    init_tracing(args.verbose);

    let mut config = load_config(&args.config)?;

    if let Some(extensions) = args.extensions {
        config.file_extensions = extensions;
    }
    config.max_file_size_kb = args.max_size;

    config
        .validate_patterns()
        .context("Invalid pattern in configuration")?;

    // Load and merge profile if specified
    if let Some(ref profile_source) = args.profile {
        let profile = load_profile(profile_source)?;
        let pattern_count = profile.patterns.len();
        let profile_name = profile.metadata.name.clone();
        let profile_version = profile.metadata.version.clone();

        // Merge profile patterns with config patterns
        for pattern in profile.patterns {
            config.patterns.push(pattern);
        }
        if args.verbose >= 1 {
            eprintln!("Loaded profile: {} (v{})", profile_name, profile_version);
            eprintln!("  {} patterns from profile", pattern_count);
        }
    }

    // Apply category filters (--disable and --only)
    let original_count = config.patterns.len();
    if let Some(ref only_categories) = args.only {
        // Keep only patterns matching specified categories
        let categories: Vec<_> = only_categories
            .iter()
            .filter_map(|s| parse_category(s))
            .collect();
        config.patterns.retain(|p| categories.contains(&p.category));
        if args.verbose >= 1 {
            eprintln!(
                "Filtered to {} categories: {} -> {} patterns",
                only_categories.join(","),
                original_count,
                config.patterns.len()
            );
        }
    } else if let Some(ref disable_categories) = args.disable {
        // Remove patterns matching specified categories
        let categories: Vec<_> = disable_categories
            .iter()
            .filter_map(|s| parse_category(s))
            .collect();
        config
            .patterns
            .retain(|p| !categories.contains(&p.category));
        if args.verbose >= 1 {
            eprintln!(
                "Disabled {} categories: {} -> {} patterns",
                disable_categories.join(","),
                original_count,
                config.patterns.len()
            );
        }
    }

    let scanner = Scanner::new(config.patterns.clone()).context("Failed to initialize scanner")?;

    let walker = Walker::new(&config);
    let entries = walker.walk(&args.paths);

    if entries.is_empty() {
        eprintln!("No files found to scan");
        std::process::exit(1);
    }

    let mut all_findings = Vec::new();
    let mut scan_results = Vec::new();
    let mut has_errors = false;

    // Set up filename checker for convention analysis (disabled by default)
    let filename_check_config = FilenameCheckConfig {
        check_duplicates: false,     // Requires opt-in via config
        min_files_for_convention: 5, // Need 5+ files to establish pattern
        convention_threshold: 0.7,   // 70% must follow convention
        use_language_hints: false,   // Require project convention before flagging
    };

    // Extract naming patterns for duplicate detection
    let naming_patterns: Vec<_> = config
        .patterns
        .iter()
        .filter(|p| p.category == antislop::PatternCategory::NamingConvention)
        .cloned()
        .collect();

    let mut filename_checker = if args.no_filename_check {
        None
    } else {
        Some(FilenameChecker::with_config_and_patterns(
            filename_check_config,
            &naming_patterns,
        ))
    };

    for entry in &entries {
        let path = entry.path.to_string_lossy().to_string();

        // Add to filename checker for convention analysis
        if let Some(ref mut checker) = filename_checker {
            checker.add_file(&entry.path);
        }

        let content = match fs::read_to_string(&entry.path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error reading file '{}': {}", path, e);
                has_errors = true;
                continue;
            }
        };

        if args.verbose >= 2 {
            eprintln!("Scanning: {}", entry.path.display());
        }

        let result = scanner.scan_file(&path, &content);
        for finding in &result.findings {
            all_findings.push(finding.clone());
        }
        scan_results.push(result);
    }

    // Check for naming convention violations
    let filename_findings = if let Some(ref checker) = filename_checker {
        checker.check()
    } else {
        Vec::new()
    };
    for finding in &filename_findings {
        all_findings.push(finding.clone());
    }

    // Recalculate summary including filename findings
    let summary = antislop::ScanSummary::new(&scan_results);

    // Add filename findings to the total score
    let filename_score: u32 = filename_findings.iter().map(|f| f.severity.score()).sum();
    let total_with_filenames = summary.total_score + filename_score;
    let exit_code = if total_with_filenames > 0 || has_errors {
        1
    } else {
        0
    };

    // Create a modified summary that includes filename findings
    let mut summary_with_filenames = summary.clone();
    summary_with_filenames.total_score = total_with_filenames;
    summary_with_filenames.total_findings += filename_findings.len();

    // Add filename findings to category counts
    for finding in &filename_findings {
        *summary_with_filenames
            .by_category
            .entry(finding.category.clone())
            .or_insert(0) += 1;
        *summary_with_filenames
            .by_severity
            .entry(finding.severity.clone())
            .or_insert(0) += 1;
    }

    // Update files_with_findings if filename findings exist in previously clean files
    if !filename_findings.is_empty() {
        let files_with_filename_issues: std::collections::HashSet<_> =
            filename_findings.iter().map(|f| f.file.clone()).collect();
        let files_with_content_issues: std::collections::HashSet<_> = scan_results
            .iter()
            .filter(|r| !r.findings.is_empty())
            .map(|r| r.path.clone())
            .collect();

        let total_files_with_issues = files_with_filename_issues
            .union(&files_with_content_issues)
            .count();
        summary_with_filenames.files_with_findings = total_files_with_issues;
    }

    let format = if let Some(fmt) = args.format {
        match fmt.as_str() {
            "json" => Format::Json,
            "sarif" => Format::Sarif,
            _ => Format::Human,
        }
    } else if args.json {
        Format::Json
    } else {
        Format::Human
    };

    let reporter = Reporter::new(format);

    all_findings.sort_by_key(|f| (f.file.clone(), f.line));

    reporter.report(all_findings, summary_with_filenames)?;

    if exit_code != 0 {
        std::process::exit(exit_code);
    }

    Ok(())
}

fn init_tracing(verbose: u8) {
    let level = match verbose {
        0 => "warn",
        1 => "info",
        2 => "debug",
        _ => "trace",
    };

    tracing_subscriber::fmt()
        .with_env_filter(format!("antislop={}", level))
        .with_writer(io::stderr)
        .try_init()
        .ok();
}

fn load_config(path: &Option<PathBuf>) -> Result<Config> {
    if let Some(p) = path {
        return Config::load(p).context("Failed to load config");
    }

    for name in CONFIG_FILES {
        let p = PathBuf::from(name);
        // Check if path exists AND is a file (not a directory)
        if p.exists() && p.is_file() {
            return Config::load(&p).context("Failed to load config");
        }
    }

    Ok(Config::default())
}

fn print_languages() {
    println!("Supported languages:");
    println!("  Python      (.py)");
    println!("  JavaScript  (.js, .mjs, .cjs)");
    println!("  TypeScript  (.ts)");
    println!("  JSX         (.jsx)");
    println!("  TSX         (.tsx)");
    println!("  Rust        (.rs)");
    println!("  Go          (.go)");
    println!("  Java        (.java)");
    println!("  Kotlin      (.kt, .kts)");
    println!("  C/C++       (.c, .cpp, .cc, .cxx, .h, .hpp)");
    println!("  C#          (.cs)");
    println!("  Ruby        (.rb)");
    println!("  PHP         (.php)");
    println!("  Swift       (.swift)");
    println!("  Shell       (.sh, .bash, .zsh, .fish)");
}

fn print_default_config() {
    let config = Config::default();
    let toml = toml::to_string_pretty(&config).unwrap();
    println!("{}", toml);
}

fn generate_completions(shell: Shell) {
    let mut cmd = Args::command();
    let name = "antislop".to_string();
    generate(shell, &mut cmd, name, &mut io::stdout());
}

fn load_profile(source: &str) -> Result<Profile> {
    let profile_source = ProfileSource::parse(source).context("Failed to parse profile source")?;

    let loader = ProfileLoader::new().context("Failed to initialize profile loader")?;

    loader
        .load(&profile_source)
        .context(format!("Failed to load profile from '{}'", source))
}

fn print_profiles() -> Result<()> {
    let loader = ProfileLoader::new().context("Failed to initialize profile loader")?;

    let profiles = loader.list_available();

    if profiles.is_empty() {
        println!("No profiles found.");
        println!();
        println!("Profile search locations:");
        println!("  - .antislop/profiles/*.toml (project-local)");
        println!("  - ~/.config/antislop/profiles/*.toml (user)");
        println!("  - ~/.cache/antislop/profiles/*.toml (cached)");
        println!();
        println!("You can also load profiles directly:");
        println!("  antislop --profile /path/to/profile.toml");
        println!("  antislop --profile https://example.com/profile.toml");
    } else {
        println!("Available profiles:");
        println!();
        for profile in profiles {
            println!("  {} (v{})", profile.name, profile.version);
            if !profile.description.is_empty() {
                println!("    {}", profile.description);
            }
            println!("    Source: {}", profile.source.display());
            println!();
        }
    }

    Ok(())
}

/// Parse a category string into a PatternCategory enum.
fn parse_category(s: &str) -> Option<antislop::PatternCategory> {
    use antislop::PatternCategory;
    match s.to_lowercase().as_str() {
        "placeholder" => Some(PatternCategory::Placeholder),
        "deferral" => Some(PatternCategory::Deferral),
        "hedging" => Some(PatternCategory::Hedging),
        "stub" => Some(PatternCategory::Stub),
        "namingconvention" | "naming" => Some(PatternCategory::NamingConvention),
        _ => {
            eprintln!("Warning: unknown category '{}', ignoring", s);
            None
        }
    }
}
