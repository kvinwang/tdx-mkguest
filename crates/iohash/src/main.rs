//! Stream compute hash digest of stdin and output the original data to stdout. The digest is output to stderr.
use std::io::{Read as _, Write as _};

use anyhow::{Context, Result};
use blake2::{
    digest::consts::{U32, U64},
    Blake2b, Blake2s,
};
use clap::{Parser, Subcommand};
use sha2::{Digest, Sha224, Sha256, Sha384, Sha512, Sha512_224, Sha512_256};
use sha3::{Keccak224, Keccak256, Keccak384, Keccak512, Sha3_224, Sha3_256, Sha3_384, Sha3_512};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct App {
    #[command(subcommand)]
    command: Command,
    /// Binary output
    #[arg(short, long)]
    binary: bool,
}

#[derive(Subcommand, Debug)]
enum Command {
    Sha256,
    Sha512,
    Sha384,
    Sha512_224,
    Sha512_256,
    Sha224,
    Keccak224,
    Keccak256,
    Keccak384,
    Keccak512,
    Sha3_224,
    Sha3_256,
    Sha3_384,
    Sha3_512,
    Blake2b256,
    Blake2b512,
    Blake2s256,
}

impl App {
    fn compute<Hasher: Digest>(&self) -> Result<()> {
        let mut hasher = Hasher::new();
        let mut buffer = [0u8; 1024];
        loop {
            let len = std::io::stdin()
                .read(&mut buffer)
                .context("Failed to read from stdin")?;
            if len == 0 {
                break;
            }
            hasher.update(&buffer[..len]);
            std::io::stdout()
                .write_all(&buffer[..len])
                .context("Failed to write to stdout")?;
        }
        let digest = hasher.finalize();
        if self.binary {
            std::io::stderr()
                .write_all(&digest)
                .context("Failed to write to stderr")?;
        } else {
            eprintln!("{}", hex_fmt::HexFmt(&digest));
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let app = App::parse();
    match app.command {
        Command::Sha256 => {
            app.compute::<Sha256>()?;
        }
        Command::Sha512 => {
            app.compute::<Sha512>()?;
        }
        Command::Sha384 => {
            app.compute::<Sha384>()?;
        }
        Command::Sha512_224 => {
            app.compute::<Sha512_224>()?;
        }
        Command::Sha512_256 => {
            app.compute::<Sha512_256>()?;
        }
        Command::Sha224 => {
            app.compute::<Sha224>()?;
        }
        Command::Keccak224 => {
            app.compute::<Keccak224>()?;
        }
        Command::Keccak256 => {
            app.compute::<Keccak256>()?;
        }
        Command::Keccak384 => {
            app.compute::<Keccak384>()?;
        }
        Command::Keccak512 => {
            app.compute::<Keccak512>()?;
        }
        Command::Sha3_224 => {
            app.compute::<Sha3_224>()?;
        }
        Command::Sha3_256 => {
            app.compute::<Sha3_256>()?;
        }
        Command::Sha3_384 => {
            app.compute::<Sha3_384>()?;
        }
        Command::Sha3_512 => {
            app.compute::<Sha3_512>()?;
        }
        Command::Blake2b256 => {
            app.compute::<Blake2b<U32>>()?;
        }
        Command::Blake2b512 => {
            app.compute::<Blake2b<U64>>()?;
        }
        Command::Blake2s256 => {
            app.compute::<Blake2s<U32>>()?;
        }
    }
    Ok(())
}
