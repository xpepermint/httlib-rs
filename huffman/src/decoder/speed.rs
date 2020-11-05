/// All the decoding speed options which represent the number of bits that the
/// decoder can read at a time.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DecodeSpeed {
    OneBit = 1,
    TwoBits = 2,
    ThreeBits = 3,
    FourBits = 4,
    FiveBits = 5,
}

impl DecodeSpeed {
    /// Returns a vector of all available decoding speed options.
    pub fn known() -> Vec<DecodeSpeed> {
        vec![
            DecodeSpeed::OneBit,
            DecodeSpeed::TwoBits,
            DecodeSpeed::ThreeBits,
            DecodeSpeed::FourBits,
            DecodeSpeed::FiveBits,
        ]
    }
}
