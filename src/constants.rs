use std::sync::atomic::AtomicU8;

// eMMC commands
pub const MMC_SEND_EXT_CSD: u32 = 8;
pub const MMC_SWITCH: u32 = 6;
pub const MMC_SEND_STATUS: u32 = 13;
pub const MMC_BLK_MAJOR: u8 = 179;

// Response types
pub const MMC_RSP_NONE: u32 = 0;
pub const MMC_RSP_PRESENT: u32 = 1 << 0;
pub const MMC_RSP_136: u32 = 1 << 1;
pub const MMC_RSP_CRC: u32 = 1 << 2;
pub const MMC_RSP_BUSY: u32 = 1 << 3;
pub const MMC_RSP_OPCODE: u32 = 1 << 4;

// Bit 8: Ready for Data flag
pub const R1_READY_FOR_DATA: u32 = 1 << 8;

// Card State field (Bits 12:9)
pub const R1_CURRENT_STATE_SHIFT: u32 = 9;
pub const R1_CURRENT_STATE_MASK: u32 = 0xF << R1_CURRENT_STATE_SHIFT;

pub const DEV_STATUS_ARG_CMD13: u32 = 0x00010000;

// Standard SD State Values
pub const R1_STATE_IDLE: u32 = 0;
pub const R1_STATE_READY: u32 = 1;
pub const R1_STATE_IDENT: u32 = 2;
pub const R1_STATE_STBY: u32 = 3;
pub const R1_STATE_TRAN: u32 = 4;
pub const R1_STATE_DATA: u32 = 5;
pub const R1_STATE_RCV: u32 = 6;
pub const R1_STATE_PRG: u32 = 7; // Programming State
pub const R1_STATE_DIS: u32 = 8;

/// Extracts the 4-bit state value from a raw R1 response
#[inline]
pub fn get_r1_state(r1: u32) -> u32 {
    (r1 & R1_CURRENT_STATE_MASK) >> R1_CURRENT_STATE_SHIFT
}

/// Checks if the "Ready for Data" bit is set
#[inline]
pub fn is_ready_for_data(r1: u32) -> bool {
    (r1 & R1_READY_FOR_DATA) != 0
}

// Command types
pub const MMC_CMD_AC: u32 = 0 << 5;
pub const MMC_CMD_ADTC: u32 = 1 << 5;
pub const MMC_CMD_BC: u32 = 2 << 5;

// SPI responses
pub const MMC_RSP_SPI_S1: u32 = 1 << 7;
pub const MMC_RSP_SPI_BUSY: u32 = 1 << 10;

pub const MMC_RSP_SPI_R1: u32 = MMC_RSP_SPI_S1;
pub const MMC_RSP_SPI_R1B: u32 = MMC_RSP_SPI_S1 | MMC_RSP_SPI_BUSY;

// Native (non-SPI) responses
pub const MMC_RSP_R1: u32 = MMC_RSP_PRESENT | MMC_RSP_CRC | MMC_RSP_OPCODE;
pub const MMC_RSP_R1B: u32 = MMC_RSP_PRESENT | MMC_RSP_CRC | MMC_RSP_OPCODE | MMC_RSP_BUSY;

// Extra Constants
pub const BLK_SZ: u32 = 512;
pub const DEFAULT_DEV_PATH: &str = "/dev/block/mmcblk0";

/// Buffer Aligned to 512B with 8B Alignment
#[repr(align(8))]
pub struct AlignedBuffer512B(pub [u8; 512]);

// debug state
static DEBUG_LEVEL: AtomicU8 = AtomicU8::new(0u8);

pub fn set_debug_level(lvl: u8) {
    DEBUG_LEVEL.store(lvl, std::sync::atomic::Ordering::Relaxed);
}

#[inline]
pub fn debug() -> u8 {
    DEBUG_LEVEL.load(std::sync::atomic::Ordering::Relaxed)
}
