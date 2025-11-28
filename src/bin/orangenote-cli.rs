use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use log::info;
use orangenote_cli::AudioDecoder;
use std::path::PathBuf;

/// OrangeNote CLI - Offline audio transcription tool
#[derive(Parser, Debug)]
#[command(name = "orangenote-cli")]
#[command(about = "Transcribe audio files using whisper.cpp in offline mode", long_about = None)]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(author = "OrangeNote Team")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Log level (trace, debug, info, warn, error)
    #[arg(short = 'L', long, global = true, value_name = "LEVEL")]
    log_level: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Transcribe an audio file
    Transcribe {
        /// Path to audio file (mp3, wav, m4a, flac, etc.)
        #[arg(value_name = "INPUT")]
        input: PathBuf,

        /// Whisper model to use (tiny, base, small, medium, large)
        #[arg(short, long, default_value = "base")]
        model: String,

        /// Language code (e.g., 'en', 'ru', 'fr'). Auto-detect if not specified
        #[arg(short, long)]
        language: Option<String>,

        /// Output format (json, txt, srt, vtt, tsv)
        #[arg(short, long, default_value = "json")]
        format: String,

        /// Output file path. If not specified, output goes to stdout
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Number of threads for processing
        #[arg(short, long, default_value = "4")]
        threads: usize,
    },

    /// Manage transcription models
    #[command(subcommand)]
    Model(ModelCommands),

    /// Show system information
    Info,
}

#[derive(Subcommand, Debug)]
enum ModelCommands {
    /// List available models
    List,

    /// Download a model
    Download {
        /// Model name (tiny, base, small, medium, large)
        #[arg(value_name = "MODEL")]
        model: String,

        /// Force re-download if model already exists
        #[arg(short, long)]
        force: bool,
    },

    /// Remove a downloaded model
    Remove {
        /// Model name to remove
        #[arg(value_name = "MODEL")]
        model: String,
    },

    /// Check model status
    Status,
}

fn init_logging(verbose: bool, log_level: Option<String>) {
    let level = if let Some(level) = log_level {
        level.to_uppercase()
    } else if verbose {
        "DEBUG".to_string()
    } else {
        "INFO".to_string()
    };

    env_logger::Builder::from_default_env()
        .filter_level(level.parse().unwrap_or(log::LevelFilter::Info))
        .format_timestamp_millis()
        .init();
}

fn validate_input_file(path: &PathBuf) -> Result<()> {
    if !path.exists() {
        anyhow::bail!("Input file does not exist: {}", path.display());
    }

    if !path.is_file() {
        anyhow::bail!("Path is not a file: {}", path.display());
    }

    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_lowercase());

    let supported_formats = vec!["mp3", "wav", "m4a", "flac", "ogg", "wma"];
    if let Some(ext) = extension {
        if !supported_formats.contains(&ext.as_str()) {
            anyhow::bail!(
                "Unsupported audio format: .{}. Supported formats: {}",
                ext,
                supported_formats.join(", ")
            );
        }
    } else {
        anyhow::bail!("Input file has no extension");
    }

    Ok(())
}

fn validate_model(model: &str) -> Result<()> {
    let valid_models = vec!["tiny", "base", "small", "medium", "large"];
    if !valid_models.contains(&model) {
        anyhow::bail!(
            "Invalid model: '{}'. Valid models: {}",
            model,
            valid_models.join(", ")
        );
    }
    Ok(())
}

async fn handle_transcribe(
    input: PathBuf,
    model: String,
    language: Option<String>,
    format: String,
    output: Option<PathBuf>,
    _threads: usize,
) -> Result<()> {
    validate_input_file(&input).context("Input file validation failed")?;
    validate_model(&model).context("Model validation failed")?;

    info!("Starting transcription...");
    info!("Input file: {}", input.display());
    info!("Model: {}", model);
    if let Some(lang) = &language {
        info!("Language: {}", lang);
    } else {
        info!("Language: auto-detect");
    }
    info!("Output format: {}", format);

    // Step A2: Extract audio metadata using AudioDecoder
    let decoder = AudioDecoder::new(&input).context("Failed to create audio decoder")?;
    let metadata = decoder
        .get_metadata()
        .context("Failed to extract audio metadata")?;

    // Display audio information
    println!("\n📄 Audio File Information:");
    println!("  File: {}", input.display());
    println!("  Format: {}", metadata.format.as_str());
    println!("  Size: {}", metadata.file_size_human());
    println!("  {}", metadata.format_info());

    // TODO: Implement actual transcription backend integration
    println!("\n✓ Transcription pipeline ready!");
    println!("  Input: {}", input.display());
    println!("  Model: {}", model);
    println!(
        "  Language: {}",
        language.unwrap_or_else(|| "auto".to_string())
    );
    println!("  Output format: {}", format);
    if let Some(out) = &output {
        println!("  Output file: {}", out.display());
    }

    println!("\n[Note] Backend integration coming in next steps...\n");

    Ok(())
}

async fn handle_model_list() -> Result<()> {
    info!("Listing available models...");
    println!("Available Whisper models:");
    println!("  • tiny   (39M)   - Fastest");
    println!("  • base   (140M)  - Default");
    println!("  • small  (466M)  - Balanced");
    println!("  • medium (1.5G)  - Better accuracy");
    println!("  • large  (2.9G)  - Best accuracy");
    Ok(())
}

async fn handle_model_download(model: String, force: bool) -> Result<()> {
    validate_model(&model).context("Model validation failed")?;
    info!("Downloading model: {} (force: {})", model, force);
    println!(
        "[CLI Prototype] Model download not yet implemented: {}",
        model
    );
    println!("Force: {}", force);
    Ok(())
}

async fn handle_model_remove(model: String) -> Result<()> {
    validate_model(&model).context("Model validation failed")?;
    info!("Removing model: {}", model);
    println!(
        "[CLI Prototype] Model removal not yet implemented: {}",
        model
    );
    Ok(())
}

async fn handle_model_status() -> Result<()> {
    info!("Checking model status...");
    println!("[CLI Prototype] Model status check not yet implemented");
    Ok(())
}

async fn handle_info() -> Result<()> {
    info!("Displaying system information...");
    println!("OrangeNote CLI v{}", env!("CARGO_PKG_VERSION"));
    println!("System Information:");
    println!("  • OS: {}", std::env::consts::OS);
    println!("  • Arch: {}", std::env::consts::ARCH);
    println!("  • Family: {}", std::env::consts::FAMILY);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    init_logging(cli.verbose, cli.log_level);

    info!("OrangeNote CLI started");

    match cli.command {
        Some(Commands::Transcribe {
            input,
            model,
            language,
            format,
            output,
            threads,
        }) => {
            handle_transcribe(input, model, language, format, output, threads).await?;
        }
        Some(Commands::Model(ModelCommands::List)) => {
            handle_model_list().await?;
        }
        Some(Commands::Model(ModelCommands::Download { model, force })) => {
            handle_model_download(model, force).await?;
        }
        Some(Commands::Model(ModelCommands::Remove { model })) => {
            handle_model_remove(model).await?;
        }
        Some(Commands::Model(ModelCommands::Status)) => {
            handle_model_status().await?;
        }
        Some(Commands::Info) => {
            handle_info().await?;
        }
        None => {
            println!("OrangeNote CLI v{}", env!("CARGO_PKG_VERSION"));
            println!("\nNo command specified. Use --help for usage information.");
        }
    }

    Ok(())
}
