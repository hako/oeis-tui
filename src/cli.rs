use crate::api::{OEISClient, SearchQuery, Sequence};
use anyhow::{anyhow, Result};
use clap::{builder::Styles, Parser, Subcommand, ValueEnum};
use owo_colors::OwoColorize;

/// Create custom color styles for help output
fn styles() -> Styles {
    Styles::styled()
        .header(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::BrightCyan))),
        )
        .usage(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::BrightCyan))),
        )
        .literal(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::BrightGreen))),
        )
        .placeholder(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::BrightYellow))),
        )
        .valid(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::BrightMagenta))),
        )
        .invalid(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::BrightRed))),
        )
}

#[derive(Parser, Debug)]
#[command(
    name = "oeis",
    about = "A TUI and CLI for browsing the On-Line Encyclopedia of Integer Sequences (OEIS) in the terminal.",
    long_about = "a terminal user interface for exploring the On-Line Encyclopedia of Integer Sequences with advanced search capabilities and visualization.",
    styles = styles(),
    after_help = "EXAMPLES:\n  \
    # Launch interactive TUI\n  \
    oeis\n\n  \
    # Search and view results\n  \
    oeis search \"fibonacci\"\n  \
    oeis search \"1,2,3,5,8\" --limit 5\n\n  \
    # Fetch sequence in different formats\n  \
    oeis fetch A000045                    # Human-readable\n  \
    oeis fetch A000045 -f csv             # CSV format\n  \
    oeis fetch A000045 -f values -q       # Just numbers\n\n  \
    # Pipe to external tools\n  \
    oeis fetch A000045 -f tsv -q | gnuplot -p -e \"plot '-' with lines\"\n  \
    oeis search \"prime\" -f values | head -3 | xargs -n1 oeis fetch\n\n  \
    # Random sequence\n  \
    oeis random -f json | jq .name\n\n\
    For more information, visit: https://github.com/hako/oeis-tui",
    version,
    long_version = concat!(
        env!("CARGO_PKG_VERSION"), "\n",
        "Author: Wesley Hill\n",
        "License: MIT\n",
        "Repository: https://github.com/hako/oeis-tui"
    ),
    author
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum OutputFormat {
    /// Human-readable plain text (default)
    Plain,
    /// JSON format (complete data)
    Json,
    /// CSV format with index,value pairs
    Csv,
    /// TSV format with index,value pairs (tab-separated)
    Tsv,
    /// Just sequence values, one per line
    Values,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Search OEIS for a query string (sequence terms, keyword, etc.)
    #[command(after_help = "EXAMPLES:\n  \
    # Search by sequence terms\n  \
    oeis search \"1,2,3,5,8\"\n  \
    oeis search \"1,1,2,3,5,8,13,21\"\n\n  \
    # Search by keyword or attribute\n  \
    oeis search \"keyword:prime\"\n  \
    oeis search \"author:Sloane\"\n  \
    oeis search \"name:fibonacci\"\n\n  \
    # Output formats for piping\n  \
    oeis search \"fibonacci\" -f values        # Just A-numbers\n  \
    oeis search \"prime\" -f values -v         # A-numbers + names\n  \
    oeis search \"triangle\" -f json           # Full JSON\n\n  \
    # Limit results\n  \
    oeis search \"prime\" --limit 5")]
    Search {
        /// Query to search for (e.g., "1,2,3,4" or "keyword:prime")
        query: String,
        /// Maximum number of results to display (1-50)
        #[arg(short, long, default_value_t = 10)]
        limit: usize,
        /// Output format
        #[arg(short, long, value_enum, default_value = "plain")]
        format: OutputFormat,
        /// Verbose output (include names with A-numbers)
        #[arg(short, long)]
        verbose: bool,
    },
    /// Fetch a sequence by A-number (e.g., A000045)
    #[command(after_help = "EXAMPLES:\n  \
    # Fetch in different formats\n  \
    oeis fetch A000045                        # Human-readable detail\n  \
    oeis fetch A000045 -q                     # Just the data\n  \
    oeis fetch A000045 -f json                # Full JSON\n  \
    oeis fetch A000045 -f csv                 # CSV (index,value)\n  \
    oeis fetch A000045 -f tsv                 # TSV for gnuplot\n  \
    oeis fetch A000045 -f values              # Just numbers\n\n  \
    # Piping to external tools\n  \
    oeis fetch A000045 -f values -q | head -20\n  \
    oeis fetch A000045 -f json | jq '.name'\n  \
    oeis fetch A000045 -f tsv -q | gnuplot -p -e \"plot '-' with lines\"\n\n  \
    # Export to file\n  \
    oeis fetch A000045 -f csv > fibonacci.csv")]
    Fetch {
        /// A-number of the sequence
        id: String,
        /// Output format
        #[arg(short, long, value_enum, default_value = "plain")]
        format: OutputFormat,
        /// Quiet mode (minimal output)
        #[arg(short, long)]
        quiet: bool,
    },
    /// Display a random OEIS sequence
    #[command(after_help = "EXAMPLES:\n  \
    # Get a random sequence\n  \
    oeis random\n  \
    oeis random -q                            # Just the data\n  \
    oeis random -f json                       # JSON format\n\n  \
    # Fun with random sequences\n  \
    oeis random -f json | jq '.name'\n  \
    oeis random -q > sequence-of-the-day.txt\n  \
    oeis random -f values -q | head -10       # First 10 terms")]
    Random {
        /// Output format
        #[arg(short, long, value_enum, default_value = "plain")]
        format: OutputFormat,
        /// Quiet mode (minimal output)
        #[arg(short, long)]
        quiet: bool,
    },
}

pub async fn run(command: Command) -> Result<()> {
    let client = OEISClient::new()?;

    match command {
        Command::Search {
            query,
            limit,
            format,
            verbose,
        } => run_search(&client, &query, limit, &format, verbose).await?,
        Command::Fetch { id, format, quiet } => run_fetch(&client, &id, &format, quiet).await?,
        Command::Random { format, quiet } => run_random(&client, &format, quiet).await?,
    }

    Ok(())
}

async fn run_search(
    client: &OEISClient,
    query: &str,
    limit: usize,
    format: &OutputFormat,
    verbose: bool,
) -> Result<()> {
    if query.trim().is_empty() {
        return Err(anyhow!("Query cannot be empty"));
    }

    let search_query = SearchQuery::new(query);
    let page_size = limit.clamp(1, 50);
    let response = client.search(&search_query, page_size).await?;

    match format {
        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(&response)?;
            println!("{}", json);
        }
        OutputFormat::Plain => {
            println!(
                "{} '{}': {} {}",
                "Results for".bright_white(),
                query.cyan(),
                response.count.to_string().bright_yellow().bold(),
                "found".bright_white()
            );
            if let Some(results) = response.results {
                for (i, seq) in results.into_iter().take(page_size).enumerate() {
                    print_sequence_summary(i + 1, &seq);
                }
            } else {
                println!("{}", "No results available (OEIS returned too many matches).".yellow());
            }
        }
        _ => {
            // For other formats, output A-numbers (pipe-friendly)
            if let Some(results) = response.results {
                for seq in results.into_iter().take(page_size) {
                    if verbose {
                        println!("{}\t{}", seq.a_number(), seq.name);
                    } else {
                        println!("{}", seq.a_number());
                    }
                }
            }
        }
    }

    Ok(())
}

async fn run_fetch(
    client: &OEISClient,
    id: &str,
    format: &OutputFormat,
    quiet: bool,
) -> Result<()> {
    let sequence = client
        .get_sequence(id)
        .await?
        .ok_or_else(|| anyhow!("Sequence {id} not found"))?;

    format_sequence_output(&sequence, format, quiet);
    Ok(())
}

async fn run_random(client: &OEISClient, format: &OutputFormat, quiet: bool) -> Result<()> {
    match client.random_sequence().await? {
        Some(sequence) => {
            if !quiet && matches!(format, OutputFormat::Plain) {
                println!("{}:", "Random sequence".bright_magenta().bold());
            }
            format_sequence_output(&sequence, format, quiet);
        }
        None => {
            if !quiet {
                println!("{}", "No random sequence available right now.".yellow());
            }
        }
    }
    Ok(())
}

fn print_sequence_summary(index: usize, seq: &Sequence) {
    println!(
        "{}{} {} {} {}",
        format!("{:>2}.", index).dimmed(),
        seq.a_number().bright_cyan().bold(),
        "-".dimmed(),
        seq.name.bright_white(),
        ""
    );
    let data_preview = seq.data.split(',').take(12).collect::<Vec<_>>().join(", ");
    println!("    {}", data_preview.green());
    if !seq.keyword.is_empty() {
        println!("    {}: {}", "keywords".yellow(), seq.keyword.dimmed());
    }
    println!();
}

fn print_sequence_detail(seq: &Sequence) {
    println!(
        "{} {} {}",
        seq.a_number().bright_cyan().bold(),
        "-".dimmed(),
        seq.name.bright_white().bold()
    );
    println!(
        "{} {} {} {}: {}",
        "Offset:".yellow(),
        seq.offset.to_string().white(),
        "|".dimmed(),
        "Keywords".yellow(),
        seq.keyword.dimmed()
    );
    if !seq.author.is_empty() {
        println!("{}: {}", "Author".yellow(), seq.author.white());
    }
    if !seq.data.is_empty() {
        println!("{}: {}", "Data".yellow(), seq.data.green());
    }
    if !seq.comment.is_empty() {
        println!("{}:", "Comments".yellow());
        for comment in &seq.comment {
            println!("  {} {}", "-".dimmed(), comment.white());
        }
    }
    println!();
}

/// Format sequence output based on the specified format
fn format_sequence_output(seq: &Sequence, format: &OutputFormat, quiet: bool) {
    match format {
        OutputFormat::Plain => {
            if quiet {
                // Quiet mode: just the data
                println!("{}", seq.data);
            } else {
                // Full detail view
                print_sequence_detail(seq);
            }
        }
        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(seq).unwrap_or_else(|_| "{}".to_string());
            println!("{}", json);
        }
        OutputFormat::Values => {
            // Just values, one per line (perfect for piping to plotters)
            let values = parse_sequence_values(&seq.data);
            for value in values {
                println!("{}", value);
            }
        }
        OutputFormat::Csv => {
            // CSV with index,value pairs
            let values = parse_sequence_values(&seq.data);
            if !quiet {
                println!("index,value");
            }
            for (i, value) in values.iter().enumerate() {
                println!("{},{}", i, value);
            }
        }
        OutputFormat::Tsv => {
            // TSV with index,value pairs (gnuplot native format)
            let values = parse_sequence_values(&seq.data);
            if !quiet {
                println!("# index\tvalue");
            }
            for (i, value) in values.iter().enumerate() {
                println!("{}\t{}", i, value);
            }
        }
    }
}

/// Parse sequence data string into individual values
fn parse_sequence_values(data: &str) -> Vec<String> {
    data.split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}
