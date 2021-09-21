/// Provides available decoding speed options which represent the number of bits
/// that the decoder can read at a time.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DecoderSpeed {
    /// Indicates that a decoder should read 1-bit at a time.
    OneBit = 1,

    /// Indicates that a decoder should read 2-bits at a time.
    TwoBits = 2,

    /// Indicates that a decoder should read 3-bits at a time.
    ThreeBits = 3,

    /// Indicates that a decoder should read 4-bits at a time.
    FourBits = 4,

    /// Indicates that a decoder should read 5-bits at a time.
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
