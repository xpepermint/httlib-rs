//! TODO

mod error;
mod primitives;

pub use error::*;
pub use primitives::*;
pub use httlib_huffman::DecoderSpeed;
use super::Table;

/// An object for decoding HTTP/2 headers.
#[derive(Debug)]
pub struct Decoder<'a> {
    /// The number of bits to read at a time while decoding Huffman sequence.
    /// More bits at a time mean faster decoding but at the same time a higher
    /// memory footprint.
    speed: DecoderSpeed,

    /// The external protocol maximum allowed size of the dynamic table.
    max_dynamic_size: u32,

    /// A store for the static and the dynamic headers.
    table: Table<'a>,
}

impl<'a> Decoder<'a> {
    /// A flag indicating that a new header entry has been inserted into the
    /// indexing table ([6.2.1.]).
    pub const WITH_INDEXING: u8 = 0x4;

    /// A flag indicating a sensitive header field ([6.2.3.]).
    pub const NEVER_INDEXED: u8 = 0x8;

    /// Returns a new decoder instance with a desired maximum allowed size of
    /// the dynamic table.
    pub fn with_dynamic_size(max_dynamic_size: u32) -> Self {
        Self {
            speed: DecoderSpeed::FiveBits,
            max_dynamic_size,
            table: Table::with_dynamic_size(max_dynamic_size),
        }
    }

    /// Returns the maximum allowed size of the dynamic table.
    /// 
    /// Note that the dynamic table could actually be of different size. This
    /// size is just a hard limit set by the external protocol.
    pub fn max_dynamic_size(&self) -> u32 {
        self.table.max_dynamic_size()
    }

    /// Sets the maximum allowed size of the dynamic table.
    /// 
    /// This size is just a hard limit that should be set by the external
    /// protocol. Changing the size will not change the size of the actual
    /// underlaying table. The table will be updated through the size update
    /// signal when decoding.
    pub fn set_max_dynamic_size(&mut self, size: u32) {
        self.max_dynamic_size = size;
    }

    /// Decodes headers provided in HPACK's header field representation format.
    /// 
    /// The functions consumes the `buf` of bytes and writes header results to 
    /// `dst`. Each item contains header name, value and flags. The decoder will
    /// not index fields unless `0x4` flag is returned. When the `0x8` flag is
    /// present, the header field should be treated with caution.
    /// 
    /// **Example:**
    /// 
    /// ```rs
    /// let mut decoder = Decoder::default();
    /// let mut dst = Vec::new();
    /// let mut buf = vec![...];
    /// decoder.decode(&mut buf, &mut dst)?;
    /// ```
    pub fn decode(
        &mut self,
        buf: &mut Vec<u8>,
        dst: &mut Vec<(Vec<u8>, Vec<u8>, u8)>,
    ) -> Result<usize, DecoderError> {
        let mut total = 0;
        loop {
            if buf.is_empty() {
                return Ok(total);
            }
            let mut data = Vec::with_capacity(1);
            total += self.decode_exact(buf, &mut data)?;
            dst.append(&mut data);
        }
    }

    /// Decodes the exact number of headers from the provided HPACK's sequence,
    /// based on the available vector capacity.
    /// 
    /// The functions consumes the `buf` of bytes and writes header results to 
    /// `dst`. Each item contains header name, value and flags. The decoder will
    /// not index fields unless `0x4` flag is returned. When the `0x8` flag is
    /// present, the header field should be treated with caution.
    /// 
    /// **Example:**
    /// 
    /// ```rs
    /// let mut decoder = Decoder::default();
    /// let mut dst = Vec::with_capacity(2);
    /// let mut buf = vec![...];
    /// decoder.decode_exact(&mut buf, &mut dst)?;
    /// ```
    pub fn decode_exact(
        &mut self,
        buf: &mut Vec<u8>,
        dst: &mut Vec<(Vec<u8>, Vec<u8>, u8)>,
    ) -> Result<usize, DecoderError> {
        let mut total = 0;
        let mut limit = dst.capacity();
        loop {
            if buf.is_empty() || limit == 0 {
                return Ok(total);
            } else {
                limit -= 1;
            }

            let octet = buf[0];
            if octet & 128 == 128 { // indexed
                total += self.decode_indexed(buf, dst)?;
            } else if octet & 64 == 64 { // with indexing
                total += self.decode_literal(buf, dst)?;
            } else if octet & 32 == 32 {
                self.update_max_dynamic_size(buf)?;
            } else if octet & 16 == 16 { // never indexed
                total += self.decode_literal(buf, dst)?;
            } else { // without indexing
                total += self.decode_literal(buf, dst)?;
            }
        }
    }

    /// Decodes a header that exists in the indexing table.
    /// 
    /// The function reads the indexed header field representation and decodes
    /// it into the provided `dst` buffer.
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
    pub fn decode_indexed(
        &self,
        buf: &mut Vec<u8>,
        dst: &mut Vec<(Vec<u8>, Vec<u8>, u8)>,
    ) -> Result<usize, DecoderError> {
        let mut index = 0;
        let total = decode_integer(buf, &mut index, 7)?;

        let (name, value) = match self.table.get(index) {
            Some(field) => field,
            None => return Err(DecoderError::InvalidIndex),
        };
        dst.push((name.to_vec(), value.to_vec(), 0x0));

        Ok(total)
    }

    /// Decodes a header represented as a literal.
    /// 
    /// The function reads the literal header field representation and decodes
    /// it into the provided `dst` buffer.
    /// 
    /// **Literal header field with incremental indexing - indexed name
    /// ([6.2.1.], figure 6):**
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
    /// **Literal header field with incremental indexing - new name ([6.2.1.],
    /// Figure 7):**
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
    /// **Literal header field without indexing - indexed name ([6.2.2.],
    /// Figure 8):**
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
    /// **Literal header field without indexing - new name ([6.2.2.], figure 9):**
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
    /// **Literal header field never indexed - indexed name ([6.2.3.],
    /// Figure 10):**
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
    /// **Literal header field never indexed - new name ([6.2.3.], figure 11):**
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
    /// [6.2.1]: https://tools.ietf.org/html/rfc7541#section-6.2.1
    /// [6.2.2]: https://tools.ietf.org/html/rfc7541#section-6.2.2
    /// [6.2.3]: https://tools.ietf.org/html/rfc7541#section-6.2.3
    fn decode_literal(
        &mut self,
        bytes: &mut Vec<u8>,
        dst: &mut Vec<(Vec<u8>, Vec<u8>, u8)>,
    ) -> Result<usize, DecoderError> {
        let mut total = 0;
        let octet = bytes[0];

        let prefix = if octet & 64 == 64 { // with indexing
            6
        } else { // without and never indexed
            4
        };

        let mut index = 0;
        total += decode_integer(bytes, &mut index, prefix)?;

        let name = if index == 0 {
            let mut name = Vec::new();
            total += decode_string(bytes, self.speed, &mut name)?;
            name
        } else if let Some(h) = self.table.get(index) {
            h.0.to_vec()
        } else {
            return Err(DecoderError::InvalidIndex);
        };

        let mut value = Vec::new();
        total += decode_string(bytes, self.speed, &mut value)?;

        if octet & 64 == 64 {
            self.table.insert(name.clone(), value.clone());        
            dst.push((name, value, 0x4));
        } else  if octet & 16 == 16 {
            dst.push((name, value, 0x8));
        } else {
            dst.push((name, value, 0x0));
        }

        Ok(total)
    }

    /// Decodes the dynamic table size update signal and sets the new size to
    /// the dynamic table.
    /// 
    /// The new maximum size MUST be lower than or equal to the limit determined
    /// by the protocol using HPACK.
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
    fn update_max_dynamic_size(
        &mut self,
        bytes: &mut Vec<u8>,
    ) -> Result<usize, DecoderError> {
        let mut new_size = 0;
        let total = decode_integer(bytes, &mut new_size, 5)?;

        if new_size > self.max_dynamic_size {
            Err(DecoderError::InvalidMaxDynamicSize)
        } else {
            self.table.update_max_dynamic_size(new_size);
            Ok(total)
        }
    }
}

impl<'a> Default for Decoder<'a> {
    fn default() -> Self {
        let table = Table::default();
        Self {
            speed: DecoderSpeed::FiveBits, // fast decoding
            max_dynamic_size: table.max_dynamic_size(),
            table,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Should decode the HPACK's indexed header field representation into a
    /// header that exists in the indexing table ([6.1.], figure 5).
    /// 
    /// [6.1.]: https://tools.ietf.org/html/rfc7541#section-6.1
    #[test]
    fn decodes_indexed() {
        let mut decoder = Decoder::default();
        let mut dst = Vec::new();
        let mut buf = vec![
            0x80 | 2, // index 2
            0x80 | 14, // index 14
        ];
        decoder.decode(&mut buf, &mut dst).unwrap();
        assert_eq!(dst, vec![
            (b":method".to_vec(), b"GET".to_vec(), 0x0),
            (b":status".to_vec(), b"500".to_vec(), 0x0),
        ]);
    }

    /// Should decode the HPACK's literal header field representation with
    /// incremental indexing where header name is represented with an index
    /// ([6.2.1.], figure 6). A a new header entry should be added to the
    /// indexing table .
    /// 
    /// [6.2.1.]: https://tools.ietf.org/html/rfc7541#section-6.2.1
    #[test]
    fn decodes_indexed_name_with_indexing() {
        let mut decoder = Decoder::default();
        let mut dst = Vec::new();
        let mut buf = vec![
            66, 133, 215, 14, 251, 216, 255,    // (index(2), huffman(PATCH))
            78, 130, 108, 1                     // (index(14), huffman(501))
        ];
        decoder.decode(&mut buf, &mut dst).unwrap();
        assert_eq!(dst, vec![
            (b":method".to_vec(), b"PATCH".to_vec(), 0x4), // with indexing flag
            (b":status".to_vec(), b"501".to_vec(), 0x4), // with indexing flag
        ]);
        assert_eq!(decoder.table.len(), 63); // 2 headers inserted into indexing table
    }

    /// Should decode the HPACK's literal header field representation with
    /// incremental indexing where name and value are provided in bytes
    /// ([6.2.1.], figure 7). A new header entry should be added to the indexing
    /// table.
    /// 
    /// [6.2.1.]: https://tools.ietf.org/html/rfc7541#section-6.2.1
    #[test]
    fn decodes_literal_with_indexing() {
        let mut decoder = Decoder::default();
        let mut dst = Vec::new();
        let mut buf = vec![
            64, 131, 148, 231, 7, 131, 148, 231, 15,    // (2, huffman(PATCH))
            64, 131, 140, 118, 3, 131, 140, 118, 7      // (14, huffman(501))
        ];
        decoder.decode(&mut buf, &mut dst).unwrap();
        assert_eq!(dst, vec![
            (b"foo0".to_vec(), b"foo1".to_vec(), 0x4), // with indexing flag
            (b"bar0".to_vec(), b"bar1".to_vec(), 0x4), // with indexing flag
        ]);
        assert_eq!(decoder.table.len(), 63); // 2 headers inserted into indexing table
    }

    /// Should decode the HPACK's literal header field representation without
    /// indexing where header name is represented with an index ([6.2.2.],
    /// figure 8). The indexing table should not be altered.
    /// 
    /// [6.2.2.]: https://tools.ietf.org/html/rfc7541#section-6.2.2
    #[test]
    fn decodes_indexed_name_without_indexing() {
        let mut decoder = Decoder::default();
        let mut dst = Vec::new();
        let mut buf = vec![
            2, 5, 80, 65, 84, 67, 72,   // (2, PATCH)
            14, 130, 108, 1             // (14, huffman(501))
        ];
        decoder.decode(&mut buf, &mut dst).unwrap();
        assert_eq!(dst, vec![
            (b":method".to_vec(), b"PATCH".to_vec(), 0x0), // without flags
            (b":status".to_vec(), b"501".to_vec(), 0x0), // without flags
        ]);
        assert_eq!(decoder.table.len(), 61); // table not altered
    }

    /// Should decode the HPACK's literal header field representation without
    /// indexing where name and value are provided in bytes ([6.2.2.],
    /// figure 9). The indexing table should not be altered.
    /// 
    /// [6.2.2.]: https://tools.ietf.org/html/rfc7541#section-6.2.2
    #[test]
    fn decodes_literal_without_indexing() {
        let mut decoder = Decoder::default();
        let mut dst = Vec::new();
        let mut buf = vec![
            0, 4, 102, 111, 111, 48, 4, 98, 97, 114, 48,   // (foo0, bar0)
            0, 131, 148, 231, 15, 131, 140, 118, 7         // (huffman(foo1), huffman(bar1))
        ];
        decoder.decode(&mut buf, &mut dst).unwrap();
        assert_eq!(dst, vec![
            (b"foo0".to_vec(), b"bar0".to_vec(), 0x0), // without flags
            (b"foo1".to_vec(), b"bar1".to_vec(), 0x0), // without flags
        ]);
        assert_eq!(decoder.table.len(), 61); // table not altered
    }

    /// Should decode the HPACK's never indexed literal header field
    /// representation where header name is represented with an index ([6.2.3.],
    /// figure 10). The indexing table should not be altered.
    /// 
    /// [6.2.3.]: https://tools.ietf.org/html/rfc7541#section-6.2.3
    #[test]
    fn decodes_indexed_name_never_indexed() {
        let mut decoder = Decoder::default();
        let mut dst = Vec::new();
        let mut buf = vec![
            18, 5, 80, 65, 84, 67, 72,  // (2, PATCH)
            30, 130, 108, 1             // (14, huffman(501))
        ];
        decoder.decode(&mut buf, &mut dst).unwrap();
        assert_eq!(dst, vec![
            (b":method".to_vec(), b"PATCH".to_vec(), 0x8), // never indexed flag
            (b":status".to_vec(), b"501".to_vec(), 0x8), // never indexed flag
        ]);
        assert_eq!(decoder.table.len(), 61); // table not altered
    }

    /// Should decode the HPACK's never indexed literal header field
    /// representation where name and value are provided in bytes ([6.2.3.],
    /// figure 11). The indexing table should not be altered.
    /// 
    /// [6.2.3.]: https://tools.ietf.org/html/rfc7541#section-6.2.3
    #[test]
    fn decodes_literal_never_indexed() {
        let mut decoder = Decoder::default();
        let mut dst = Vec::new();
        let mut buf = vec![
            16, 4, 102, 111, 111, 48, 4, 98, 97, 114, 48,  // (foo0, bar0)
            16, 4, 102, 111, 111, 49, 4, 98, 97, 114, 49   // (foo1, bar1)
        ];
        decoder.decode(&mut buf, &mut dst).unwrap();
        assert_eq!(dst, vec![
            (b"foo0".to_vec(), b"bar0".to_vec(), 0x8), // never indexed flag
            (b"foo1".to_vec(), b"bar1".to_vec(), 0x8), // never indexed flag
        ]);
        assert_eq!(decoder.table.len(), 61); // table not altered
    }

    /// Should decode the exact number of headers based on vector capacity.
    #[test]
    fn decodes_exact() {
        let mut decoder = Decoder::default();
        let mut dst = Vec::with_capacity(1);
        let mut buf = vec![
            0x80 | 2, // index 2
            0x80 | 14, // index 14
        ];
        decoder.decode_exact(&mut buf, &mut dst).unwrap();
        assert_eq!(dst.len(), 1);
    }

    /// Should decode a dynamic table size update signal and set the new size
    /// to the underlaying table.
    #[test]
    fn decodes_max_dynamic_size() {
        let mut decoder = Decoder::with_dynamic_size(70);
        decoder.table.insert(b"a".to_vec(), b"a".to_vec()); // size: +34
        decoder.table.insert(b"b".to_vec(), b"b".to_vec()); // size: +34
        let mut dst = Vec::new();
        decoder.decode(&mut vec![63, 19], &mut dst).unwrap(); // set to size 50
        assert_eq!(dst, vec![]); // no items
        assert_eq!(decoder.table.max_dynamic_size(), 50); // new dynamic size is 50
        assert_eq!(decoder.table.dynamic_len(), 1); // 1 header evicted
    }
}
