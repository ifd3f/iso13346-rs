#![doc = include_str!("../README.md")]

mod iso9660_vd;

/// Size of a standard sector, in bytes.
pub const SECTOR_SIZE: usize = 2048;
