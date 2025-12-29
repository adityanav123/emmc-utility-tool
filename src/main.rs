use mmc_ops::read_extcsd;
use rhexdump::prelude::*;
use std::fs::File;
use std::io;

mod constants;
mod mmc_ops;

fn main() -> io::Result<()> {
    let fd = File::open("/dev/block/mmcblk0")?;
    let mut buff = [0u8; 512];

    // Reading Extcsd
    read_extcsd(&fd, &mut buff)?;
    println!("EXTCSD:");

    rhexdump!(&buff);

    Ok(())
}
