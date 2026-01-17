use libc::{c_int, c_uint};
use nix::ioctl_readwrite;
use std::{io, mem, os::fd::RawFd, thread::sleep, time::Duration};

use crate::constants::debug;

const MMC_BLOCK_MAJOR: c_int = 179;
ioctl_readwrite!(mmc_ioc_cmd, MMC_BLOCK_MAJOR, 0, MmcIocCmd);

/* Kernel Layout
struct mmc_ioc_cmd {
    /*
     * Direction of data: nonzero = write, zero = read.
     * Bit 31 selects 'Reliable Write' for RPMB.
     */
    int write_flag;

    /* Application-specific command.  true = precede with CMD55 */
    int is_acmd; // TODO: Have to check!!

    __u32 opcode;
    __u32 arg;
    __u32 response[4];  /* CMD response */
    unsigned int flags;
    unsigned int blksz;
    unsigned int blocks;

    /*
     * Sleep at least postsleep_min_us useconds, and at most
     * postsleep_max_us useconds *after* issuing command.  Needed for
     * some read commands for which cards have no other way of indicating
     * they're ready for the next command (i.e. there is no equivalent of
     * a "busy" indicator for read operations).
     */
    unsigned int postsleep_min_us;
    unsigned int postsleep_max_us;

    /*
     * Override driver-computed timeouts.  Note the difference in units!
     */
    unsigned int data_timeout_ns;
    unsigned int cmd_timeout_ms;

    /*
     * For 64-bit machines, the next member, ``__u64 data_ptr``, wants to
     * be 8-byte aligned.  Make sure this struct is the same size when
     * built for 32-bit.
     */
    __u32 __pad;

    /* DAT buffer */
    __u64 data_ptr;
};
#define mmc_ioc_cmd_set_data(ic, ptr) ic.data_ptr = (__u64)(unsigned long) ptr
 */
#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct MmcIocCmd {
    pub write_flag: c_uint,
    pub opcode: c_uint,
    pub arg: c_uint,
    pub response: [c_uint; 4],
    pub flags: c_uint,
    pub blksz: c_uint,
    pub blocks: c_uint,
    pub postsleep_min_us: c_uint,
    pub postsleep_max_us: c_uint,
    pub data_timeout_ns: c_uint,
    pub cmd_timeout_ms: c_uint,
    pub __pad: c_uint, // ensures data_ptr is 8-aligned on 64-bit
    pub data_ptr: u64, // kernel uses __u64
}

// Compile-time layout checks
const _: () = assert!(mem::size_of::<MmcIocCmd>() == 72);
const _: () = assert!(mem::align_of::<MmcIocCmd>() == 8);

impl MmcIocCmd {
    #[inline]
    pub fn new(opcode: u32, arg: u32, flags: u32) -> Self {
        Self {
            write_flag: 0,
            opcode,
            arg,
            flags,
            response: [0; 4],
            blksz: 0,
            blocks: 0,
            postsleep_min_us: 0,
            postsleep_max_us: 0,
            data_timeout_ns: 0,
            cmd_timeout_ms: 0,
            __pad: 0,
            data_ptr: 0,
        }
    }

    #[inline]
    pub fn set_timeout(mut self, cmd_ms: u32, data_ns: u32) -> Self {
        self.cmd_timeout_ms = cmd_ms;
        self.data_timeout_ns = data_ns;
        self
    }

    #[inline]
    pub fn set_data(mut self, buffer: *mut u8, blocks: u32, blksz: u32) -> Self {
        self.data_ptr = buffer as u64;
        self.blocks = blocks;
        self.blksz = blksz;
        self
    }
}

#[inline]
#[allow(dead_code)] // won't throw warnings
pub fn delay_ms(x: u32) {
    sleep(Duration::from_millis(x as u64));
}

#[inline]
#[allow(dead_code)]
pub fn inter_command_gap_data() {
    sleep(Duration::from_millis(2));
}

#[inline]
pub fn nix_to_io(e: nix::Error) -> io::Error {
    io::Error::from_raw_os_error(e as i32)
}

/// Executes IoCTL
pub fn ioctl(fd: RawFd, cmd: &mut MmcIocCmd) -> io::Result<()> {
    let d = debug();
    if d > 0 {
        eprintln!(
            "[mmc][ioctl] opcode={}, arg={:#x}, flags={:#x}, ...",
            cmd.opcode, cmd.arg, cmd.flags
        );
    }

    let res = unsafe { mmc_ioc_cmd(fd, cmd) };
    match res {
        Ok(_) => {
            if d > 1 {
                eprintln!("[mmc][ioctl][ok] response={:08x?}", cmd.response);
            }
            Ok(())
        }
        Err(e) => {
            if d > 0 {
                eprintln!("[mmc][ioctl][err] {:?} response={:08x?}", e, cmd.response);
            }
            Err(nix_to_io(e))
        }
    }
}
