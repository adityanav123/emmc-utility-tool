use clap::{Parser, Subcommand};
use constants::DEFAULT_DEV_PATH;
use mmc_ops::read_extcsd;
use rhexdump::prelude::*;
use std::fs::File;
use std::io;

mod constants;
mod mmc_ops;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Cli {
    ///Custom Device Path
    #[arg(short, long)]
    pdev: Option<String>,

    /// emmc commands
    #[command(subcommand)]
    operation: Command,

    /// Turn on Debugging Info (-d, -dd, -ddd....)
    #[arg(short, long,  action=clap::ArgAction::Count)]
    debug: u8,
}

#[derive(Subcommand, Clone, Debug)]
enum Command {
    /// Read Extended CSD Register (512B) and print hex-dump
    ExtCsd,
    /// Read CID Register and print hex-dump
    CID,
    /// Perform Firmware Field Upgrade
    FFU,
    /// Perform Logical Blk Address -to- Physical Blk Address Conversion
    L2P,
    /// Fetch Smart Report
    SR,
    /// Fetch Secure Smart Report
    SSR,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let dev = cli.pdev.as_deref().unwrap_or(DEFAULT_DEV_PATH);

    if cli.debug > 0 {
        eprintln!(
            "[debug] dev={}, op={:?}, debug_level={}",
            dev, cli.operation, cli.debug
        );
    }

    match cli.operation {
        Command::ExtCsd => {
            let fd = File::open(dev)?;
            let mut buf = [0u8; 512];
            read_extcsd(&fd, &mut buf)?;

            println!("EXT_CSD ({dev}):");
            rhexdump!(&buf);
        }
        _ => {
            println!("Under Development!")
        }
    }

    Ok(())
}
