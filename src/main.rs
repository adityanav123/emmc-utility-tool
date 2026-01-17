use clap::{Parser, Subcommand};
use constants::set_debug_level;
use rhexdump::prelude::*;
use std::io;

use nix::fcntl::{OFlag, open};
use nix::sys::stat::Mode;
use std::os::fd::{AsRawFd, OwnedFd};

use crate::constants::{AlignedBuffer512B, DEFAULT_DEV_PATH, DEV_STATUS_ARG_CMD13};
use crate::mmc_cmds::{check_device_status, fetch_extcsd};

mod constants;
mod mmc_cmds;
mod mmc_ops;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Cli {
    ///  Device Path  (--dev, -p)
    #[arg(short = 'p', long)]
    dev: Option<String>,

    /// emmc commands
    #[command(subcommand)]
    operation: MMCOperations,

    /// Turn on Debugging Info (--debug, -d)
    #[arg(short='d', long, action=clap::ArgAction::Count)]
    debug: u8,
}

#[derive(Subcommand, Clone, Debug)]
enum MMCOperations {
    /// Read Extended CSD Register
    ExtCsd,
    /// Check Device Status
    DeviceStatus,
    /// Read CID Register
    CID,
    /// Firmware Field Upgrade
    FFU,
    /// Logical Blk Address -to- Physical Blk Address Conversion
    L2P,
}

/// Open e.MMC Device Handle
pub fn open_mmc_rw(path: &str) -> io::Result<OwnedFd> {
    open(path, OFlag::O_RDWR | OFlag::O_CLOEXEC, Mode::empty())
        .map_err(|e| io::Error::from_raw_os_error(e as i32))
}

/// MAIN flow
fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let dev = cli.dev.as_deref().unwrap_or(DEFAULT_DEV_PATH);
    // debug flow
    if cli.debug > 0 {
        eprintln!(
            "[debug] dev={}, op={:?}, debug_level={}",
            dev, cli.operation, cli.debug
        );
        set_debug_level(cli.debug);
    }

    match cli.operation {
        MMCOperations::ExtCsd => {
            let fd = open_mmc_rw(dev)?;
            println!("[mmc][op] : fetching ext-csd register!");
            let buff: AlignedBuffer512B = fetch_extcsd(fd.as_raw_fd())?;

            println!("EXT_CSD ({dev}):");
            rhexdump!(&buff.0); // print hexdump
        }
        MMCOperations::DeviceStatus => {
            let fd = open_mmc_rw(dev)?;
            println!("[mmc][op] : checking device status!");
            match check_device_status(fd.as_raw_fd(), DEV_STATUS_ARG_CMD13) {
                Ok(_) => println!("[mmc][chk-status] device in ready state!"),
                Err(e) => {
                    eprintln!("[mmc][chk-status] device not in ready state!");
                    if cli.debug > 0 {
                        eprintln!("[debug] check-status error: {e}");
                    }
                }
            }
        }
        _ => println!("Under Development!"),
    }

    Ok(())
}
