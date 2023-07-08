//! Basic utilities for parsing ISO9660 volume descriptors, and finding
//! UDF data on them.

use std::str::from_utf8;

use nom::{
    bytes::{complete::take, streaming::tag},
    combinator::map_res,
    sequence::tuple,
    IResult,
};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use strum::{EnumIs, EnumString};

use crate::util::Parse;

/// Header of a Volume Descriptor. Version is always 1.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VDHeader {
    pub vd_type: VDType,
    pub identifier: VDIdentifier,
}

#[derive(TryFromPrimitive, IntoPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VDType {
    Boot = 0,
    Primary = 1,
    Supplementary = 2,
    Partition = 3,
    Terminator = 255,
}

#[derive(EnumString, EnumIs, Debug, Clone, Copy, PartialEq, Eq)]
pub enum VDIdentifier {
    /// Indicates that this volume contains an ISO 9660 file system.
    #[strum(serialize = "CD001")]
    CD001,

    /// Denotes the beginning of the extended descriptor section.
    #[strum(serialize = "BEA01")]
    BEA01,

    /// Indicates that this volume contains a UDF file system.
    #[strum(serialize = "NSR02")]
    NSR02,

    /// Indicates that this volume contains a UDF file system.
    #[strum(serialize = "NSR03")]
    NSR03,

    /// Includes information concerning boot loader location and entry point address.
    #[strum(serialize = "BOOT2")]
    BOOT2,

    /// Denotes the end of the extended descriptor section.
    #[strum(serialize = "TEA01")]
    TEA01,
}

impl Parse for VDHeader {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (rest, (vd_type, identifier, _)) =
            tuple((VDType::parse, VDIdentifier::parse, tag(b"\x01")))(input)?;

        Ok((
            rest,
            Self {
                vd_type,
                identifier,
            },
        ))
    }
}

impl Parse for VDIdentifier {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map_res(take(5usize), |d| Self::try_from(d))(input)
    }
}

impl TryFrom<&[u8]> for VDIdentifier {
    type Error = ();

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match from_utf8(value) {
            Ok(v) => Self::try_from(v).map_err(|_| ()),
            Err(_) => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, os::unix::prelude::FileExt};

    use crate::{util::Parse, iso9660_vd::VDHeader};

    #[test]
    fn test() {
        let f = File::open("/home/astrid/Downloads/Win10_21H2_English_x64.iso").unwrap();
        let mut sector = vec![0u8; 2048];

        for i in 0..10 {
            f.read_at(&mut sector, 32768 + 2048 * i).unwrap();
            let vd = VDHeader::parse(&sector);
            match vd {
                Ok((_, h)) => println!("{h:x?}"),
                Err(_) => println!("Error"),
            }
        }
    }
}
