//! Basic utilities for parsing ISO9660 volume descriptors, and finding
//! UDF data on them.

use std::str::from_utf8;

use nom::{
    bytes::{complete::take, streaming::tag},
    combinator::map_res,
    sequence::tuple,
    IResult,
};
use nom_derive::{Nom, Parse};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use strum::{EnumIs, EnumString};

/// Header of a Volume Descriptor. Version is always 1.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VDHeader {
    pub vd_type: VDType,
    pub identifier: VDIdentifier,
}

#[derive(Nom, TryFromPrimitive, IntoPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
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

impl Parse<&[u8]> for VDHeader {
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

impl Parse<&[u8]> for VDIdentifier {
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
    use pretty_assertions::assert_eq;

    use rstest::rstest;

    use crate::iso9660_vd::*;

    const MSINSTALL_SECTOR16_26: &[u8] =
        include_bytes!("../resources/sector16/msinstall-16-26.dat");

    #[rstest]
    #[case(
        &MSINSTALL_SECTOR16_26[0..2048] ,
        VDHeader {
            vd_type: VDType::Primary,
            identifier: VDIdentifier::CD001
        }
    )]
    #[case(
        &MSINSTALL_SECTOR16_26[2048..2048*2],
        VDHeader {
            vd_type: VDType::Boot,
            identifier: VDIdentifier::CD001
        }
    )]
    #[case(
        &MSINSTALL_SECTOR16_26[2048*2..2048*3],
        VDHeader {
            vd_type: VDType::Terminator,
            identifier: VDIdentifier::CD001
        }
    )]
    #[case(
        &MSINSTALL_SECTOR16_26[2048*3..2048*4] ,
        VDHeader {
            vd_type: VDType::Boot,
            identifier: VDIdentifier::BEA01
        }
    )]
    #[case(
        &MSINSTALL_SECTOR16_26[2048*4..2048*5] ,
        VDHeader {
            vd_type: VDType::Boot,
            identifier: VDIdentifier::NSR02
        }
    )]
    #[case(
        &MSINSTALL_SECTOR16_26[2048*5..2048*6] ,
        VDHeader {
            vd_type: VDType::Boot,
            identifier: VDIdentifier::TEA01,
        }
    )]
    fn parse_valid_vdheader(#[case] input: &[u8], #[case] expected: VDHeader) {
        let (rest, vdh) = VDHeader::parse(input).expect("Failed to parse");
        assert_eq!(vdh, expected);
        assert_eq!(rest.len(), 2041);
    }
}
