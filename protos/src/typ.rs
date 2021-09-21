use core::convert::TryFrom;

/// Provides available wire types supported by proto3.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Typ {
    /// Represents the wire type `0` which allows for encoding of data formats
    /// `int32`, `int64`, `uint32`, `uint64`, `sint32`, `sint64` and `bool`.
    Varint = 0,

    /// Represents the wire type `1` which allows for encoding of data formats
    /// `fixed64`, `sfixed64` and `double`.
    Bit64 = 1,

    /// Represents the wire type `2` which allows for encoding of data formats
    /// `string`, `bytes`, `embedded messages` and `packed repeated fields`.
    LengthDelimited = 2,

    /// Represents the wire type `5` which allows for encoding of data formats
    /// `fixed32`, `sfixed32` and `float`.
    Bit32 = 5,

    /// Represents un unknown wire type.
    Unknown = -1,
}

impl TryFrom<u64> for Typ {
    type Error = std::io::Error;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Typ::Varint),
            1 => Ok(Typ::Bit64),
            2 => Ok(Typ::LengthDelimited),
            5 => Ok(Typ::Bit32),
            _ => Ok(Typ::Unknown),
        }
    }
}
