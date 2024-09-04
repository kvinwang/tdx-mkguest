use anyhow::{Context, Result};
use scale::Decode;
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
    Show(ShowCommand),
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
/// Show TDX RTMRs
#[argh(subcommand, name = "show")]
struct ShowCommand {}

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

#[derive(Decode)]
struct ParsedReport {
    attributes: [u8; 8],
    xfam: [u8; 8],
    mrtd: [u8; 48],
    mrconfigid: [u8; 48],
    mrowner: [u8; 48],
    mrownerconfig: [u8; 48],
    rtmr0: [u8; 48],
    rtmr1: [u8; 48],
    rtmr2: [u8; 48],
    rtmr3: [u8; 48],
    servtd_hash: [u8; 48],
}

impl core::fmt::Debug for ParsedReport {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use hex_fmt::HexFmt as HF;

        f.debug_struct("ParsedReport")
            .field("attributes", &HF(&self.attributes))
            .field("xfam", &HF(&self.xfam))
            .field("mrtd", &HF(&self.mrtd))
            .field("mrconfigid", &HF(&self.mrconfigid))
            .field("mrowner", &HF(&self.mrowner))
            .field("mrownerconfig", &HF(&self.mrownerconfig))
            .field("rtmr0", &HF(&self.rtmr0))
            .field("rtmr1", &HF(&self.rtmr1))
            .field("rtmr2", &HF(&self.rtmr2))
            .field("rtmr3", &HF(&self.rtmr3))
            .field("servtd_hash", &HF(&self.servtd_hash))
            .finish()
    }
}

fn cmd_show() -> Result<()> {
    let report_data = att::TdxReportData([0; 64]);
    let report = att::get_report(&report_data).context("Failed to get report")?;
    let parsed_report =
        ParsedReport::decode(&mut report.0.get(512..).context("Failed to get report")?)
            .context("Failed to decode report")?;
    println!("{:#?}", parsed_report);
    Ok(())
}

fn main() -> Result<()> {
    let cli: Cli = argh::from_env();

    match cli.command {
        Commands::Report(_) => cmd_report()?,
        Commands::Quote(_) => cmd_quote()?,
        Commands::Show(_) => cmd_show()?,
        Commands::Extend(extend_args) => {
            cmd_extend(extend_args)?;
        }
    }

    Ok(())
}
