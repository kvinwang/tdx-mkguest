use anyhow::{Context, Result};
use sha2::Digest;
use std::io::{self, Read, Write};
use tdx_attest as att;

use argh::FromArgs;

#[derive(FromArgs)]
/// TDX control utility
struct Cli {
    #[argh(subcommand)]
    command: Commands,
}

#[derive(FromArgs)]
#[argh(subcommand)]
enum Commands {
    Report(ReportCommand),
    Quote(QuoteCommand),
    Extend(ExtendArgs),
}

#[derive(FromArgs)]
/// Get TDX report
#[argh(subcommand, name = "report")]
struct ReportCommand {}

#[derive(FromArgs)]
/// Get TDX quote
#[argh(subcommand, name = "quote")]
struct QuoteCommand {}

#[derive(FromArgs)]
/// Extend RTMR
#[argh(subcommand, name = "extend")]
struct ExtendArgs {
    #[argh(option, default = "1", short = 'v')]
    /// version (default: 1)
    version: u32,

    #[argh(option, default = "3", short = 'i')]
    /// RTMR index (default: 3)
    index: u32,

    #[argh(option, default = "1", short = 't')]
    /// event type (default: 1)
    event_type: u32,

    #[argh(option, default = "Default::default()", short = 'd')]
    /// event data
    event_data: String,

    #[argh(switch, short = 's')]
    /// read event data from stdin
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
    let cli: Cli = argh::from_env();

    match cli.command {
        Commands::Report(_) => cmd_report()?,
        Commands::Quote(_) => cmd_quote()?,
        Commands::Extend(extend_args) => {
            cmd_extend(extend_args)?;
        }
    }

    Ok(())
}
