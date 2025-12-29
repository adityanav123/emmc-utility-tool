// eMMC commands
pub const MMC_SEND_EXT_CSD: u32 = 8;

// Response types
pub const MMC_RSP_NONE: u32 = 0;
pub const MMC_RSP_PRESENT: u32 = 1 << 0;
pub const MMC_RSP_136: u32 = 1 << 1;
pub const MMC_RSP_CRC: u32 = 1 << 2;
pub const MMC_RSP_BUSY: u32 = 1 << 3;
pub const MMC_RSP_OPCODE: u32 = 1 << 4;

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

pub const MMC_IOC_CMD: u32 = 0; // arch dependent
