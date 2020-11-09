/// An enum representing encoder input formats.
#[derive(Debug)]
pub enum EncoderInput {
    /// Represents a fully indexed header field.
    Indexed(u32),

    /// Represents a header field where name is represented by an index and the
    /// value is provided in bytes. This format can hold configuration flags.
    IndexedName(u32, Vec<u8>, u8),

    /// Represents a header field where name and value are provided in bytes.
    /// This format can hold configuration flags.
    Literal(Vec<u8>, Vec<u8>, u8),
}

impl<'a> From<u32> for EncoderInput {
    fn from(field: u32) -> Self {
        EncoderInput::Indexed(field)
    }
}

impl<'a> From<(u32, Vec<u8>, u8)> for EncoderInput {
    fn from(field: (u32, Vec<u8>, u8)) -> Self {
        EncoderInput::IndexedName(field.0, field.1, field.2)
    }
}

impl<'a> From<(Vec<u8>, Vec<u8>, u8)> for EncoderInput {
    fn from(field: (Vec<u8>, Vec<u8>, u8)) -> Self {
        EncoderInput::Literal(field.0, field.1, field.2)
    }
}
