use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use scale::Decode;
use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};
use tdx_attest as att;

const EVENT_LOG_FILE: &str = "/var/log/tdx_mr3/tdx_events.log";

#[derive(Parser)]
/// TDX control utility
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Report,
    Quote,
    Extend(ExtendArgs),
    Show,
    Hex(HexCommand),
}

#[derive(Parser)]
/// Hex encode data
struct HexCommand {
    #[clap(value_parser)]
    /// filename to hex encode
    filename: Option<String>,
}

#[derive(Parser)]
/// Extend RTMR
struct ExtendArgs {
    #[arg(short = 'v', long, default_value_t = 1)]
    /// version (default: 1)
    version: u32,

    #[clap(short = 'i', long, default_value_t = 3)]
    /// RTMR index (default: 3)
    index: u32,

    #[clap(short = 't', long, default_value_t = 1)]
    /// event type (default: 1)
    event_type: u32,

    #[clap(short, long, default_value = "")]
    /// digest to extend to the RTMR
    digest: String,

    #[clap(short, long)]
    /// associated data of the event
    associated_data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EventLog {
    imr: u32,
    event_type: u32,
    digest: String,
    associated_data: String,
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
    let digest = hex::decode(&extend_args.digest).context("Failed to decode digest")?;

    let mut padded_digest: [u8; 48] = [0; 48];
    if digest.len() > 48 {
        bail!("Digest too long");
    }
    padded_digest[..digest.len()].copy_from_slice(&digest);
    let rtmr_event = att::TdxRtmrEvent {
        version: extend_args.version,
        rtmr_index: extend_args.index as u64,
        digest: padded_digest,
        event_type: extend_args.event_type,
    };
    att::extend_rtmr(&rtmr_event).context("Failed to extend RTMR")?;
    let hexed_digest = hex::encode(&padded_digest);

    println!("Extended RTMR {}: {}", extend_args.index, hexed_digest);

    // Append to event log
    let event_log = EventLog {
        imr: extend_args.index,
        event_type: extend_args.event_type,
        digest: hexed_digest,
        associated_data: extend_args.associated_data,
    };
    let logline = serde_json::to_string(&event_log).context("Failed to serialize event log")?;

    let logfile_path = std::path::Path::new(EVENT_LOG_FILE);
    let logfile_dir = logfile_path
        .parent()
        .context("Failed to get event log directory")?;
    std::fs::create_dir_all(logfile_dir).context("Failed to create event log directory")?;

    let mut logfile = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(logfile_path)
        .context("Failed to open event log file")?;
    logfile
        .write_all(logline.as_bytes())
        .context("Failed to write to event log file")?;
    logfile
        .write_all(b"\n")
        .context("Failed to write to event log file")?;
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

fn cmd_hex(hex_args: HexCommand) -> Result<()> {
    fn hex_encode_io(io: &mut impl Read) -> Result<()> {
        loop {
            let mut buf = [0; 1024];
            let n = io.read(&mut buf).context("Failed to read from stdin")?;
            if n == 0 {
                break;
            }
            print!("{}", hex_fmt::HexFmt(&buf[..n]));
        }
        Ok(())
    }
    if let Some(filename) = hex_args.filename {
        let mut input =
            std::fs::File::open(&filename).context(format!("Failed to open {}", filename))?;
        hex_encode_io(&mut input)?;
    } else {
        hex_encode_io(&mut io::stdin())?;
    };
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Report => cmd_report()?,
        Commands::Quote => cmd_quote()?,
        Commands::Show => cmd_show()?,
        Commands::Extend(extend_args) => {
            cmd_extend(extend_args)?;
        }
        Commands::Hex(hex_args) => {
            cmd_hex(hex_args)?;
        }
    }

    Ok(())
}
