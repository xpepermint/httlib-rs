/// All the decoding speed options which represent the number of bits that the
/// decoder can read at a time.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DecoderSpeed {
    OneBit = 1,
    TwoBits = 2,
    ThreeBits = 3,
    FourBits = 4,
    FiveBits = 5,
}

impl DecoderSpeed {
    /// Returns a vector of all available decoding speed options.
    pub fn known() -> Vec<DecoderSpeed> {
        vec![
            DecoderSpeed::OneBit,
            DecoderSpeed::TwoBits,
            DecoderSpeed::ThreeBits,
            DecoderSpeed::FourBits,
            DecoderSpeed::FiveBits,
        ]
    }
}
