//! Antislop - AI Slop Linter
//!
//! A blazing-fast, multi-language linter for detecting AI-generated code slop.

use antislop::{Config, Format, Reporter, Scanner, Walker, CONFIG_FILES, VERSION};
use anyhow::{Context, Result};
use clap::{CommandFactory, Parser};
use clap_complete::{generate, Shell};
use std::fs;
use std::io;
use std::path::PathBuf;

/// Antislop - A blazing-fast linter for detecting AI-generated code slop.
#[derive(Parser, Debug)]
#[command(name = "antislop")]
#[command(author = "Antislop Contributors")]
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

    if let Some(shell) = args.completions {
        generate_completions(shell);
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

    for entry in &entries {
        let path = entry.path.to_string_lossy().to_string();

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

    let summary = antislop::ScanSummary::new(&scan_results);
    let exit_code = if summary.total_score > 0 || has_errors {
        1
    } else {
        0
    };

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

    reporter.report(all_findings, summary)?;

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
        if p.exists() {
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
