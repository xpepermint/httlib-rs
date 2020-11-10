//! TODO

mod error;
mod input;
mod primitives;

pub use error::*;
pub use input::*;
use primitives::*;
use crate::table::Table;

/// An object for encoding HTTP/2 headers.
/// 
/// Since headers in HPACK can be encoded in multiple ways, the encoder provides
/// multiple methods for encoding headers. A developer is responsible to
/// carefully choose between them to achieve the best encoding performance.
#[derive(Debug)]
pub struct Encoder<'a> {
    /// A store for the static and the dynamic headers.
    table: Table<'a>,
}

impl<'a> Encoder<'a> {
    /// A flag indicating to encode header name with Huffman algorithm (`0x1`).
    pub const HUFFMAN_NAME: u8 = 0x1;

    /// A flag indicating to encode header value with Huffman algorithm (`0x2`).
    pub const HUFFMAN_VALUE: u8 = 0x2;

    /// A flag indicating to index literal header field (`0x4`).
    pub const WITH_INDEXING: u8 = 0x4;

    /// A flag indicating to never index literal header field (`0x8`).
    pub const NEVER_INDEXED: u8 = 0x8;

    /// A flag indicating to find the best literal representation by searching
    /// the indexing table (`0x10`).
    pub const BEST_FORMAT: u8 = 0x10;

    /// Returns a new encoder instance with the provided maximum allowed size of
    /// the dynamic table.
    pub fn with_dynamic_size(max_dynamic_size: u32) -> Self {
        Self {
            table: Table::with_dynamic_size(max_dynamic_size),
        }
    }
    
    /// Returns the maximum allowed size of the dynamic table.
    pub fn max_dynamic_size(&mut self) -> u32 {
        self.table.max_dynamic_size()
    }
    
    /// Encodes headers into the HPACK's header field representation format.
    /// 
    /// By default headers are represented without indexing and Huffman encoding
    /// is not enabled for literals. We can configure the encoder by providing
    /// byte `flags`:
    /// 
    /// * `0x1`: Use Huffman to encode header name.
    /// * `0x2`: Use Huffman to encode header value.
    /// * `0x4`: Literal header field with incremental indexing ([6.2.1.]).
    /// * `0x8`: Literal header field never indexed ([6.2.3.]).
    /// * `0x10`: Encode literal as the best representation.
    /// 
    /// **Example:**
    /// 
    /// ```rust
    /// use httlib_hpack::Encoder;
    /// 
    /// let mut encoder = Encoder::default();
    /// let mut dst = Vec::new();
    /// let name = b":method".to_vec();
    /// let value = b"PATCH".to_vec();
    /// let flags = 0x2 | 0x4 | 0x10;
    /// encoder.encode((name, value, flags), &mut dst).unwrap();
    /// ```
    /// 
    /// [6.2.1.]: https://tools.ietf.org/html/rfc7541#section-6.2.1
    /// [6.2.3.]: https://tools.ietf.org/html/rfc7541#section-6.2.3
    pub fn encode<F>(
        &mut self,
        field: F,
        dst: &mut Vec<u8>,
    ) -> Result<(), EncoderError>
    where
        F: Into<EncoderInput>,
    {
        match field.into() {
            EncoderInput::Indexed(index) => {
                self.encode_indexed(index, dst)
            },
            EncoderInput::IndexedName(index, value, flags) => {
                self.encode_indexed_name(index, value, flags, dst)
            },
            EncoderInput::Literal(name, value, flags) => {
                if flags & 0x10 == 0x10 {
                    match self.table.find(&name, &value) {
                        Some((index, true)) => {
                            self.encode_indexed(index as u32, dst)
                        },
                        Some((index, false)) => {
                            self.encode_indexed_name(index as u32, value, flags, dst)
                        },
                        None => {
                            self.encode_literal(name, value, flags, dst)
                        },
                    }
                } else {
                    self.encode_literal(name, value, flags, dst)
                }
            },
        }
    }

    /// Encodes a header that exists at `index` in the indexing table.
    /// 
    /// The function converts the header index into HPACK's indexed header field
    /// representation and writes it into the `dst` buffer.
    /// 
    /// **Indexed header field representation ([6.1.], figure 5):**
    /// 
    /// ```txt
    ///   0   1   2   3   4   5   6   7
    /// +---+---+---+---+---+---+---+---+
    /// | 1 |        Index (7+)         |
    /// +---+---------------------------+
    /// ```
    /// 
    /// [6.1.]: https://tools.ietf.org/html/rfc7541#section-6.1
    pub fn encode_indexed(
        &self,
        index: u32,
        dst: &mut Vec<u8>,
    ) -> Result<(), EncoderError> {

        if self.table.get(index).is_none() {
            return Err(EncoderError::InvalidIndex);
        }

        encode_integer(index, 0x80, 7, dst)
    }
    
    /// Encodes a header where its name is represented with an `index` from the
    /// indexing table and the `value` is provided in bytes.
    /// 
    /// This function converts the header into HPACK's literal header field
    /// representation and writes it into the `dst` buffer.
    /// 
    /// **Literal header field with incremental indexing ([6.2.1.], figure 6):**
    /// 
    /// ```txt
    ///   0   1   2   3   4   5   6   7
    /// +---+---+---+---+---+---+---+---+
    /// | 0 | 1 |      Index (6+)       |
    /// +---+---+-----------------------+
    /// | H |     Value Length (7+)     |
    /// +---+---------------------------+
    /// | Value String (Length octets)  |
    /// +-------------------------------+
    /// ```
    /// 
    /// **Literal header field without indexing ([6.2.2.], figure 8):**
    /// 
    /// ```txt
    ///   0   1   2   3   4   5   6   7
    /// +---+---+---+---+---+---+---+---+
    /// | 0 | 0 | 0 | 0 |  Index (4+)   |
    /// +---+---+-----------------------+
    /// | H |     Value Length (7+)     |
    /// +---+---------------------------+
    /// | Value String (Length octets)  |
    /// +-------------------------------+
    /// ```
    /// 
    /// **Literal header field never indexed ([6.2.3.], figure 10):**
    /// 
    /// ```txt
    ///   0   1   2   3   4   5   6   7
    /// +---+---+---+---+---+---+---+---+
    /// | 0 | 0 | 0 | 1 |  Index (4+)   |
    /// +---+---+-----------------------+
    /// | H |     Value Length (7+)     |
    /// +---+---------------------------+
    /// | Value String (Length octets)  |
    /// +-------------------------------+
    /// ```
    /// 
    /// By default headers are represented as literals without indexing and
    /// header's value is encoded as a string. We can configure the encoder by
    /// providing byte `flags`:
    /// 
    /// * `0x2`: Use Huffman to encode header value.
    /// * `0x4`: Literal header field with incremental indexing ([6.2.1.]).
    /// * `0x8`: Literal header field never indexed ([6.2.3.]).
    /// 
    /// [6.2.1.]: https://tools.ietf.org/html/rfc7541#section-6.2.1
    /// [6.2.2.]: https://tools.ietf.org/html/rfc7541#section-6.2.2
    /// [6.2.3.]: https://tools.ietf.org/html/rfc7541#section-6.2.3
    pub fn encode_indexed_name(
        &mut self,
        index: u32,
        value: Vec<u8>,
        flags: u8,
        dst: &mut Vec<u8>,
    ) -> Result<(), EncoderError> {

        let name = if let Some(entry) = self.table.get(index) {
            entry.0.to_vec()
        } else {
            return Err(EncoderError::InvalidIndex);
        };

        if flags & 0x4 == 0x4 {
            self.table.insert(name, value.clone());
            encode_integer(index, 0x40, 6, dst)?;
        } else if flags & 0x8 == 0x8 {
            encode_integer(index, 0b00010000, 4, dst)?;
        } else { // without indexing
            encode_integer(index, 0x0, 4, dst)?;
        }

        encode_string(value, flags & 0x2 == 0x2, dst)
    }

    /// Encodes a header where its name and value are provided in bytes.
    /// 
    /// This function converts the header into HPACK's literal header field
    /// representation and writes it into the `dst` buffer.
    /// 
    /// **Literal header field with incremental indexing ([6.2.1.], figure 7):**
    /// 
    /// ```txt
    ///   0   1   2   3   4   5   6   7
    /// +---+---+---+---+---+---+---+---+
    /// | 0 | 1 |           0           |
    /// +---+---+-----------------------+
    /// | H |     Name Length (7+)      |
    /// +---+---------------------------+
    /// |  Name String (Length octets)  |
    /// +---+---------------------------+
    /// | H |     Value Length (7+)     |
    /// +---+---------------------------+
    /// | Value String (Length octets)  |
    /// +-------------------------------+
    /// ```
    /// 
    /// **Literal header field without indexing ([6.2.2.], figure 9):**
    /// 
    /// ```txt
    ///   0   1   2   3   4   5   6   7
    /// +---+---+---+---+---+---+---+---+
    /// | 0 | 0 | 0 | 0 |       0       |
    /// +---+---+-----------------------+
    /// | H |     Name Length (7+)      |
    /// +---+---------------------------+
    /// |  Name String (Length octets)  |
    /// +---+---------------------------+
    /// | H |     Value Length (7+)     |
    /// +---+---------------------------+
    /// | Value String (Length octets)  |
    /// +-------------------------------+
    /// ```
    /// 
    /// **Literal header field never indexed ([6.2.3.], figure 11):**
    /// 
    /// ```txt
    ///   0   1   2   3   4   5   6   7
    /// +---+---+---+---+---+---+---+---+
    /// | 0 | 0 | 0 | 1 |       0       |
    /// +---+---+-----------------------+
    /// | H |     Name Length (7+)      |
    /// +---+---------------------------+
    /// |  Name String (Length octets)  |
    /// +---+---------------------------+
    /// | H |     Value Length (7+)     |
    /// +---+---------------------------+
    /// | Value String (Length octets)  |
    /// +-------------------------------+
    /// ```
    /// 
    /// By default headers are represented as literals without indexing. Heder's
    /// name and value are encoded as a string. We can configure the encoder by
    /// providing byte `flags`:
    /// 
    /// * `0x1`: Use Huffman to encode header name.
    /// * `0x2`: Use Huffman to encode header value.
    /// * `0x4`: Literal header field with incremental indexing ([6.2.1.]).
    /// * `0x8`: Literal header field never indexed ([6.2.3.]).
    /// 
    /// [6.2.1]: https://tools.ietf.org/html/rfc7541#section-6.2.1
    /// [6.2.2]: https://tools.ietf.org/html/rfc7541#section-6.2.2
    /// [6.2.3]: https://tools.ietf.org/html/rfc7541#section-6.2.3
    pub fn encode_literal(
        &mut self,
        name: Vec<u8>,
        value: Vec<u8>,
        flags: u8,
        dst: &mut Vec<u8>,
    ) -> Result<(), EncoderError> {

        if flags & 0x4 == 0x4 {
            dst.push(0x40);
            self.table.insert(name.clone(), value.clone());
        } else if flags & 0x8 == 0x8 {
            dst.push(0b00010000);
        } else { // without indexing
            dst.push(0x0);
        }

        encode_string(name, flags & 0x1 == 0x1, dst)?;
        encode_string(value, flags & 0x2 == 0x2, dst)
    }

    /// Updates the maximum size of the dynamic table and encodes the new size
    /// into a dynamic table size signal.
    /// 
    /// The new maximum size MUST be lower than or equal to the limit determined
    /// by the protocol using HPACK. In HTTP/2, this limit is the last value of
    /// the `SETTINGS_HEADER_TABLE_SIZE` received from the decoder and
    /// acknowledged by the encoder.
    /// 
    /// **Maximum Dynamic table size change ([6.3.], figure 12):**
    /// 
    /// ```txt
    ///   0   1   2   3   4   5   6   7
    /// +---+---+---+---+---+---+---+---+
    /// | 0 | 0 | 1 |   Max size (5+)   |
    /// +---+---------------------------+
    /// ```
    /// 
    /// [6.3]: https://tools.ietf.org/html/rfc7541#section-6.3
    pub fn update_max_dynamic_size(
        &mut self,
        size: u32,
        dst: &mut Vec<u8>,
    ) -> Result<(), EncoderError> {
        self.table.update_max_dynamic_size(size);
        encode_integer(size, 0b00100000, 5, dst)
    }
}

impl<'a> Default for Encoder<'a> {
    fn default() -> Self {
        Self {
            table: Table::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Should encode a header that exists in the indexing table into HPACK's
    /// indexed header field representation ([6.1.], figure 5).
    /// 
    /// [6.1.]: https://tools.ietf.org/html/rfc7541#section-6.1
    #[test]
    fn encodes_indexed() {
        let mut encoder = Encoder::default();
        encoder.table.insert(b"name62".to_vec(), b"value62".to_vec()); // add dynamic header
        let fields = vec![
            (2, vec![0x80 | 2]), // (:method, GET)
            (3, vec![0x80 | 3]), // (:method, POST)
            (14, vec![0x80 | 14]), // (:status, 500)
            (62, vec![0x80 | 62]), // (name62, value62)
        ];
        for (index, res) in fields {
            let mut dst = Vec::new();
            encoder.encode(index, &mut dst).unwrap();
            assert_eq!(dst, res);
        }
        assert_eq!(encoder.table.len(), 62); // only one header in dynamic table
    }

    /// Should encode a header, where its name is represented with an index and
    /// the value is provided in bytes, into a literal header field
    /// representation with incremental indexing ([6.2.1.], figure 6).
    /// 
    /// [6.2.1.]: https://tools.ietf.org/html/rfc7541#section-6.2.1
    #[test]
    fn encodes_indexed_name_with_indexing() {
        let mut encoder = Encoder::default();
        let mut dst = Vec::new();
        let field = (
            2, // index
            b"PATCH".to_vec(),
            0x2 | 0x4,
        );
        encoder.encode(field, &mut dst).unwrap(); // (:method, PATCH), Huffman
        assert_eq!(dst[0] & 0b01000000, 64); // with incremental indexing
        assert_eq!(dst[1] & 0b10000000, 128); // value encoded with Huffman
        assert_eq!(&dst[2..], vec![215, 14, 251, 216, 255]); // value as huffman sequence
        assert_eq!(encoder.table.len(), 62); // inserted into indexing table
        let entry = encoder.table.get(62).unwrap();
        assert_eq!(entry.0, b":method"); // indexed name
        assert_eq!(entry.1, b"PATCH"); // indexed value
    }

    /// Should encode a header, where its name and value are provided in bytes,
    /// into a literal header field representation with incremental indexing
    /// ([6.2.1.], figure 7).
    /// 
    /// [6.2.1.]: https://tools.ietf.org/html/rfc7541#section-6.2.1
    #[test]
    fn encodes_literal_with_indexing() {
        let mut encoder = Encoder::default();
        let mut dst = Vec::new();
        let field = (
            b"foo".to_vec(),
            b"bar".to_vec(),
            0x4 | 0x1 | 0x2,
        );
        encoder.encode(field, &mut dst).unwrap(); // (huffman(foo), huffman(bar))
        assert_eq!(dst[0], 0b01000000); // with incremental indexing
        assert_eq!(&dst[1..4], vec![130, 148, 231]); // name as huffman sequence
        assert_eq!(&dst[4..], vec![131, 140, 118, 127]); // value as huffman sequence
        assert_eq!(encoder.table.len(), 62); // inserted into indexing table
        let entry = encoder.table.get(62).unwrap();
        assert_eq!(entry.0, b"foo"); // indexed name
        assert_eq!(entry.1, b"bar"); // indexed value
    }

    /// Should encode a header, where its name is represented with an index and
    /// the value is provided in bytes, into a literal header field
    /// representation without indexing ([6.2.2.], figure 8). The indexing table
    /// should not be altered.
    /// 
    /// [6.2.2.]: https://tools.ietf.org/html/rfc7541#section-6.2.2
    #[test]
    fn encodes_indexed_name_without_indexing() {
        let mut encoder = Encoder::default();
        let mut dst = Vec::new();
        let field = (13, b"PATCH".to_vec(), 0x0);
        encoder.encode(field, &mut dst).unwrap(); // (:status, PATCH)
        assert_eq!(dst[0], 13); // without indexing (matches index value)
        assert_eq!(&dst[1..], vec![5, 80, 65, 84, 67, 72]); // value as string
        assert_eq!(encoder.table.len(), 61); // table not altered
    }

    /// Should encode a header, where its name and value are provided in bytes,
    /// into a literal header field representation without indexing ([6.2.2.],
    /// figure 9). The indexing table should not be altered.
    /// 
    /// [6.2.2.]: https://tools.ietf.org/html/rfc7541#section-6.2.2
    #[test]
    fn encodes_literal_without_indexing() {
        let mut encoder = Encoder::default();
        let mut dst = Vec::new();
        let field = (b"foo".to_vec(), b"bar".to_vec(), 0x1);
        encoder.encode(field, &mut dst).unwrap(); // (huffman(foo), bar)
        assert_eq!(dst[0], 0); // without indexing
        assert_eq!(&dst[2..4], vec![148, 231]); // name as string
        assert_eq!(&dst[4..], vec![3, 98, 97, 114]); // value as string
        assert_eq!(encoder.table.len(), 61); // table not altered
    }

    /// Should encode a header, where its name is represented with an index and
    /// the value is provided in bytes, into a never indexed literal header
    /// field representation ([6.2.3.], figure 10). The indexing table should
    /// not be altered.
    /// 
    /// [6.2.3.]: https://tools.ietf.org/html/rfc7541#section-6.2.3
    #[test]
    fn encodes_indexed_name_never_indexed() {
        let mut encoder = Encoder::default();
        let mut dst = Vec::new();
        let field = (13, b"PATCH".to_vec(), 0x8);
        encoder.encode(field, &mut dst).unwrap(); // (:status, 501)
        assert_eq!(dst[0] & 0b00010000, 16); // never indexed
        assert_eq!(&dst[1..], vec![5, 80, 65, 84, 67, 72]); // value as string
        assert_eq!(encoder.table.len(), 61); // table not altered
    }

    /// Should encode a header, where its name and value are provided in bytes,
    /// into a never indexed literal header field representation ([6.2.3.],
    /// figure 11). The indexing table should not be altered.
    /// 
    /// [6.2.3.]: https://tools.ietf.org/html/rfc7541#section-6.2.3
    #[test]
    fn encodes_literal_never_indexed() {
        let mut encoder = Encoder::default();
        let mut dst = Vec::new();
        let field = (b"foo".to_vec(), b"bar".to_vec(), 0x8);
        encoder.encode(field, &mut dst).unwrap(); // (foo, bar)
        assert_eq!(dst[0], 0b00010000); // never indexed
        assert_eq!(&dst[1..5], vec![3, 102, 111, 111]); // name as string
        assert_eq!(&dst[5..], vec![3, 98, 97, 114]); // value as string
        assert_eq!(encoder.table.len(), 61); // table not altered
    }

    /// Should encode a header, where its name and value are provided in bytes,
    /// into the best header field representation.
    #[test]
    fn encodes_literal_automatically() {
        let mut encoder = Encoder::default();
        let fields = vec![
            ((b":method".to_vec(), b"GET".to_vec(), 0x10), vec![130]), // (:method, GET) => index(2)
            ((b":method".to_vec(), b"DELETE".to_vec(), 0x10 | 0x4), vec![66, 6, 68, 69, 76, 69, 84, 69]), // (:method, DELETE) => (index(2), DELETE)
            ((b"a".to_vec(), b"b".to_vec(), 0x10 | 0x1), vec![0, 129, 31, 1, 98]), // (a, b) => (huffman(a), b)
        ];
        for (field, res) in fields {
            let mut dst = Vec::new();
            encoder.encode(field, &mut dst).unwrap();
            assert_eq!(dst, res);
        }
        assert_eq!(encoder.table.len(), 62); // table altered only once
    }

    /// Should encode a dynamic table size update signal.
    #[test]
    fn updates_max_dynamic_size() {
        let mut encoder = Encoder::with_dynamic_size(70);
        encoder.table.insert(b"a".to_vec(), b"a".to_vec()); // size: +34
        encoder.table.insert(b"b".to_vec(), b"b".to_vec()); // size: +34
        let mut dst = Vec::new();
        encoder.update_max_dynamic_size(50, &mut dst).unwrap();
        assert_eq!(dst[0] & 0b00100000, 32); // size update
        assert_eq!(dst, vec![63, 19]); // encoded size
        assert_eq!(encoder.table.dynamic_len(), 1); // 1 header evicted
    }
}
