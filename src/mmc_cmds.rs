use crate::constants::*;
use crate::mmc_ops::*;
use std::{io, os::fd::RawFd};

/// Returns `Ok(true)` if ready , `Ok(false)` if device not ready.
fn cmd13_device_status(fd: RawFd, status_arg: u32) -> io::Result<(bool, u32)> {
    let mut cmd =
        MmcIocCmd::new(MMC_SEND_STATUS, status_arg, MMC_RSP_R1 | MMC_CMD_AC).set_timeout(1500, 0);

    ioctl(fd, &mut cmd)?;
    let r1 = cmd.response[0];

    let ready = is_ready_for_data(r1);
    let state = get_r1_state(r1);

    Ok((ready && state != R1_STATE_PRG, r1))
}

/// Fetches Extended CSD Register for the device
fn cmd8_read_extcsd(fd: RawFd, buf: &mut AlignedBuffer512B) -> io::Result<bool> {
    buf.0.fill(0);

    let mut cmd = MmcIocCmd::new(MMC_SEND_EXT_CSD, 0, MMC_RSP_R1 | MMC_CMD_ADTC)
        .set_data(buf.0.as_mut_ptr(), 1, BLK_SZ)
        .set_timeout(0, 0);

    ioctl(fd, &mut cmd)?; // Err propagates
    Ok(true)
}

/// UAPIs
pub fn fetch_extcsd(fd: RawFd) -> io::Result<AlignedBuffer512B> {
    let mut buf = AlignedBuffer512B([0u8; 512]);

    check_device_status(fd, DEV_STATUS_ARG_CMD13)?; // error propagates
    cmd8_read_extcsd(fd, &mut buf)?;
    check_device_status(fd, DEV_STATUS_ARG_CMD13)?; // error propagates

    Ok(buf)
}

pub fn check_device_status(fd: RawFd, device_status_cmd13: u32) -> io::Result<()> {
    cmd13_device_status(fd, device_status_cmd13)?;
    for _ in 0..50 {
        if cmd13_device_status(fd, device_status_cmd13)?.0 {
            return Ok(());
        }
        delay_ms(20);
    }
    Err(io::Error::new(
        io::ErrorKind::TimedOut,
        "Device not ready (CMD13 check_device_status failed after 50 tries)",
    ))
}
