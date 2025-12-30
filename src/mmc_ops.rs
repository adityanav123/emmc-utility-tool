use std::{fs::File, io, os::fd::AsRawFd};

use crate::constants::*;

#[repr(C)] // just do as C does while allocating the struct
#[derive(Debug, Copy, Clone)]
pub struct MmcIocCmd {
    write_flag: u32,
    opcode: u32,
    arg: u32,
    flags: u32,
    blksz: u32,
    blocks: u32,
    postsleep_min_us: u32,
    postsleep_max_us: u32,
    data_timeout_ns: u32,
    cmd_timeout_ms: u32,
    __pad: u32,
    response: [u32; 4],
    data_ptr: u64,
}

pub fn exec_ioctl(fd: &File, cmd: &mut MmcIocCmd) -> io::Result<()> {
    let rc = unsafe { libc::ioctl(fd.as_raw_fd(), MMC_IOC_CMD.try_into().unwrap(), cmd) };
    if rc != 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

pub fn read_extcsd(fd: &File, buf: &mut [u8; 512]) -> io::Result<()> {
    buf.fill(0);

    let mut cmd = MmcIocCmd {
        write_flag: 0,
        opcode: MMC_SEND_EXT_CSD,
        arg: 0,
        flags: MMC_RSP_SPI_R1 | MMC_RSP_R1 | MMC_CMD_ADTC,
        blksz: BLK_SZ,
        blocks: 1,
        postsleep_min_us: 0,
        postsleep_max_us: 0,
        data_timeout_ns: 0, // driver default
        cmd_timeout_ms: 0,  // driver default
        __pad: 0,
        response: [0; 4],
        data_ptr: buf.as_mut_ptr() as u64,
    };

    exec_ioctl(fd, &mut cmd)?;
    Ok(())
}
