use anyhow::{Context, Result};
use clap::Parser;
use sha2::Digest;
use std::io::{self, Read, Write};
use tdx_attest as att;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    Report,
    Quote,
    Extend(ExtendArgs),
}

#[derive(clap::Args)]
struct ExtendArgs {
    #[arg(long, default_value = "1")]
    version: u32,
    #[arg(long, default_value = "3")]
    index: u32,
    #[arg(long, default_value = "1")]
    event_type: u32,
    #[arg(long, default_value = "")]
    event_data: String,
    #[arg(long)]
    stdin: bool,
}

fn cmd_quote() -> Result<()> {
    let mut report_data = att::TdxReportData([0; 64]);
    io::stdin()
        .read_exact(&mut report_data.0)
        .context("Failed to read report data")?;
    let (_key_id, quote) = att::get_quote(&report_data, None).context("Failed to get quote")?;
    io::stdout()
        .write_all(&quote)
        .context("Failed to write quote")?;
    Ok(())
}

fn cmd_extend(extend_args: ExtendArgs) -> Result<()> {
    let event_data = if extend_args.stdin {
        let mut data = Vec::new();
        io::stdin()
            .read_to_end(&mut data)
            .context("Failed to read from stdin")?;
        data
    } else {
        extend_args.event_data.into_bytes()
    };
    let extend_data = sha384_digest(&event_data);
    let rtmr_event = att::TdxRtmrEvent {
        version: extend_args.version,
        rtmr_index: extend_args.index as u64,
        extend_data,
        event_type: extend_args.event_type,
        event_data,
    };
    att::extend_rtmr(&rtmr_event).context("Failed to extend RTMR")?;
    Ok(())
}

fn cmd_report() -> Result<()> {
    let mut report_data = att::TdxReportData([0; 64]);
    io::stdin()
        .read_exact(&mut report_data.0)
        .context("Failed to read report data")?;
    let report = att::get_report(&report_data).context("Failed to get report")?;
    io::stdout()
        .write_all(&report.0)
        .context("Failed to write report")?;
    Ok(())
}

fn sha384_digest(data: &[u8]) -> [u8; 48] {
    let mut hasher = sha2::Sha384::new();
    hasher.update(data);
    let digest = hasher.finalize();
    digest.into()
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Report => cmd_report()?,
        Commands::Quote => cmd_quote()?,
        Commands::Extend(extend_args) => {
            cmd_extend(extend_args)?;
        }
    }

    Ok(())
}
