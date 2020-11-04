//! TODO

mod error;
mod primitives;

use primitives::*;
use crate::table::Table;
pub use error::*;

/// An object for handles HTTP header encoding.
/// 
/// Since headers in HPACK can be encoded in multiple ways, the encoder provides
/// multiple methods for encoding headers. A developer is responsible to
/// carefully choose between them to achieve the best encoding performance.
pub struct Encoder<'a> {
    /// A store for the static and the dynamic headers.
    table: Table<'a>,
}

impl<'a> Encoder<'a> {
    /// TODO
    pub fn encode(&mut self, name: Vec<u8>, value: Vec<u8>, dst: &mut Vec<u8>) -> Result<(), EncoderError> {
        // match self.table.find(&name, &value) {
        //     Some((index, true)) => { // full header found in the table
        //         self.encode_index(index as u32, dst)?;
        //     },
        //     Some((index, false)) => { // only the name was found in the table
        //         // self.encode_index_name(index as u32, value, false, dst);
        //     },
        //     None => { // not found in the table
        //         // self.encode_literal(name.clone(), value.clone(), true, dst);
        //         // self.table.insert(name, value);
        //     },
        // };
        Ok(())
    }

    /// Encodes a fully indexed header that already exists in the table at the
    /// provided `index` ([6.1.]).
    /// 
    /// Since the static headers permanently exist at their index values, using
    /// this method for encoding such headers is the fastest and should be used
    /// when possible.
    /// 
    /// [6.1.]: https://tools.ietf.org/html/rfc7541#section-6.1
    pub fn encode_index(&self, index: u32, dst: &mut Vec<u8>) -> Result<(), EncoderError> {
        if self.table.get(index).is_none() {
            return Err(EncoderError::InvalidIndex);
        }
        encode_integer(index, 0x80, 7, dst)
    }

    /// Encodes a header whose name is represented with an index of a header
    /// from the table but its value is not indexed and is encoded as a string
    /// literal ([6.2.1], Figure 6). This inserts a new header in to the dynamic
    /// table.
    /// 
    /// Passing `huffman` set to `true` will cause the `value` to be encoded
    /// using the Huffman algorithm.
    /// 
    /// [6.2.1]: https://tools.ietf.org/html/rfc7541#section-6.2.1
    pub fn encode_index_with_indexing(&mut self, index: u32, value: Vec<u8>, huffman: bool, dst: &mut Vec<u8> ) -> Result<(), EncoderError> {
        let name = if let Some(entry) = self.table.get(index) {
            entry.0.to_vec()
        } else {
            return Err(EncoderError::InvalidIndex);
        };

        self.table.insert(name, value.clone());

        encode_integer(index, 0x40, 6, dst)?;
        encode_string(value, huffman, dst)
    }

    /// Encodes a header whose name is represented with an index of a header
    /// from the table but its value is not indexed and is encoded as a string
    /// literal ([6.2.2], Figure 8). The indexing table is not altered.
    /// 
    /// Passing `huffman` set to `true` will cause the `value` to be encoded
    /// using the Huffman algorithm.
    /// 
    /// [6.2.2]: https://tools.ietf.org/html/rfc7541#section-6.2.1
    pub fn encode_index_without_indexing(&mut self, index: u32, value: Vec<u8>, huffman: bool, dst: &mut Vec<u8> ) -> Result<(), EncoderError> {
        if self.table.get(index).is_none() {
            return Err(EncoderError::InvalidIndex);
        }
        encode_integer(index, 0x0, 4, dst)?;
        encode_string(value.clone(), huffman, dst)
    }

    /// Encodes a header whose name is represented with an index of a header
    /// from the table but its value is not indexed and is encoded as a string
    /// literal ([6.2.3], Figure 10). The decoder will be informed that this 
    /// header should never be stored in the indexing table.
    /// 
    /// Passing `huffman` set to `true` will cause the `value` to be encoded
    /// using the Huffman algorithm.
    /// 
    /// [6.2.2]: https://tools.ietf.org/html/rfc7541#section-6.2.1
    pub fn encode_index_never_indexed(&mut self, index: u32, value: Vec<u8>, huffman: bool, dst: &mut Vec<u8> ) -> Result<(), EncoderError> {
        if self.table.get(index).is_none() {
            return Err(EncoderError::InvalidIndex);
        }
        encode_integer(index, 0b00010000, 4, dst)?;
        encode_string(value.clone(), huffman, dst)
    }

    /// Encodes a header as a literal where both the name and the value are
    /// encoded as a string literal ([6.2.1], Figure 7). This inserts a new
    /// header in to the dynamic table.
    /// 
    /// Passing `name_huffman` or `value_huffman` set to `true` will cause the
    /// `name or the `value` to be encoded using the Huffman algorithm.
    /// 
    /// [6.2.1]: https://tools.ietf.org/html/rfc7541#section-6.2.1
    pub fn encode_with_indexing(&mut self, name: Vec<u8>, value: Vec<u8>, name_huffman: bool, value_huffman: bool, dst: &mut Vec<u8>) -> Result<(), EncoderError> {
        dst.push(0x40);

        self.table.insert(name.clone(), value.clone());

        encode_string(name, name_huffman, dst)?;
        encode_string(value, value_huffman, dst)
    }

    /// Encodes a header as a literal where both the name and the value are
    /// encoded as a string literal ([6.2.1], Figure 7). The indexing table is
    /// not altered.
    /// 
    /// Passing `name_huffman` or `value_huffman` set to `true` will cause the
    /// `name or the `value` to be encoded using the Huffman algorithm.
    /// 
    /// [6.2.1]: https://tools.ietf.org/html/rfc7541#section-6.2.1
    pub fn encode_without_indexing(&mut self, name: Vec<u8>, value: Vec<u8>, name_huffman: bool, value_huffman: bool, dst: &mut Vec<u8>) -> Result<(), EncoderError> {
        dst.push(0x0);
        encode_string(name, name_huffman, dst)?;
        encode_string(value, value_huffman, dst)
    }

    /// Encodes a header as a literal where both the name and the value are
    /// encoded as a string literal ([6.2.3], Figure 11). The decoder will be
    /// informed that this header should never be stored in the indexing table.
    /// 
    /// Passing `name_huffman` or `value_huffman` set to `true` will cause the
    /// `name or the `value` to be encoded using the Huffman algorithm.
    /// 
    /// [6.2.3]: https://tools.ietf.org/html/rfc7541#section-6.2.1
    pub fn encode_never_indexed(&mut self, name: Vec<u8>, value: Vec<u8>, name_huffman: bool, value_huffman: bool, dst: &mut Vec<u8>) -> Result<(), EncoderError> {
        dst.push(0b00010000);
        encode_string(name, name_huffman, dst)?;
        encode_string(value, value_huffman, dst)
    }

    /// Handles processing the `SizeUpdate` HPACK block: updates the maximum
    /// size of the underlying dynamic table, possibly causing a number of
    /// headers to be evicted from it.
    ///
    /// Assumes that the first byte in the given buffer `buf` is the first
    /// octet in the `SizeUpdate` block.
    ///
    /// Returns the number of octets consumed from the given buffer.
    /// 
    /// Ta funkcija se mora poklicat na zacetku bloka!
    pub fn update_max_size(&mut self, size: u32, dst: &mut Vec<u8>) -> Result<(), EncoderError> {
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
    use httlib_huffman;

    fn decode_huffman(src: &[u8]) -> Vec<u8> {
        let mut bytes = Vec::new();
        httlib_huffman::decode(&src, &mut bytes, 1).unwrap();
        bytes
    }

    fn decode_string(src: &[u8]) -> Vec<u8> {
        let mut bytes = Vec::new();
        crate::decoder::decode_string(&mut src.to_vec(), &mut bytes).unwrap();
        bytes
    }

    /// Should encode an already existing fully indexed header.
    #[test]
    fn encodes_index() {
        let mut encoder = Encoder::default();
        encoder.table.insert(b"name62".to_vec(), b"value62".to_vec()); // add dynamic header
        let headers = vec![
            (2, vec![0x80 | 2]), // (:method, GET)
            (3, vec![0x80 | 3]), // (:method, POST)
            (14, vec![0x80 | 14]), // (:status, 500)
            (62, vec![0x80 | 62]), // (name62, value62)
        ];
        for (index, res) in headers {
            let mut dst = Vec::new();
            encoder.encode_index(index, &mut dst).unwrap();
            assert_eq!(dst, res);
        }
        assert_eq!(encoder.table.len(), 62); // only one header in dynamic table
    }

    /// Should encode a header where its name is represented with an index of a
    /// header that exists in the table, the value is not indexed and should be
    /// encoded as a string literal. The encoded header should inserted in to
    /// the table as a new entry.
    #[test]
    fn encodes_index_with_indexing() {
        let mut encoder = Encoder::default();
        let mut dst = Vec::new();
        encoder.encode_index_with_indexing(2, b"PATCH".to_vec(), true, &mut dst).unwrap(); // (:method, PATCH), Huffman
        assert_eq!(dst[0], 0b01000000 | 2); // name pulled from table
        assert_eq!(dst[1], 0x80 | 5); // using huffman
        assert_eq!(decode_huffman(&dst[2..]), "PATCH".as_bytes()); // huffman encoded literal
        assert_eq!(encoder.table.len(), 62); // indexed
        let entry = encoder.table.get(62).unwrap();
        assert_eq!(entry.0, b":method"); // indexed name
        assert_eq!(entry.1, b"PATCH"); // indexed value
    }

    /// Should encode a header where its name is represented with an index of a
    /// header that exists in the table, the value is not indexed and should be
    /// encoded as a string literal. The table should not be altered.
    #[test]
    fn encodes_index_without_indexing() {
        let mut encoder = Encoder::default();
        let mut dst = Vec::new();
        encoder.encode_index_without_indexing(13, b"PATCH".to_vec(), false, &mut dst).unwrap(); // (:status, 501), no Huffman
        assert_eq!(dst[0], 13); // name pulled from table
        assert_eq!(decode_string(&dst[1..]), "PATCH".as_bytes()); // encoded raw string literal
        assert_eq!(encoder.table.len(), 61); // not indexed
    }

    /// Should encode a header where its name is represented with an index of a
    /// header that exists in the table, the value is not indexed and should be
    /// encoded as a string literal. The encoded data should indicate that it
    /// should never be indexed.
    #[test]
    fn encodes_index_never_indexed() {
        let mut encoder = Encoder::default();
        let mut dst = Vec::new();
        encoder.encode_index_never_indexed(13, b"PATCH".to_vec(), false, &mut dst).unwrap(); // (:status, 501), no Huffman
        assert_eq!(dst[0], dst[0] | 0b00010000); // never indexed
        assert_eq!(decode_string(&dst[1..]), "PATCH".as_bytes()); // encoded raw string literal
        assert_eq!(encoder.table.len(), 61); // not indexed
    }

    /// Should encode a new header which does not match any existing header in
    /// the table. The new header should be inserted in to the dynamic table.
    #[test]
    fn encodes_with_indexing() {
        let mut encoder = Encoder::default();
        let mut dst = Vec::new();
        encoder.encode_with_indexing(b"foo".to_vec(), b"bar".to_vec(), true, true, &mut dst).unwrap(); // (foo, bar), use Huffman
        assert_eq!(dst[0], 0b01000000); // indexing, new name
        assert_eq!(decode_string(&dst[1..4]), b"foo".to_vec()); // huffman encoded name
        assert_eq!(decode_string(&dst[4..]), b"bar".to_vec()); // huffman encoded value
        assert_eq!(encoder.table.len(), 62); // indexed
    }

    /// Should encode a new header which does not match any existing header in
    /// the table. The table should not be altered.
    #[test]
    fn encodes_without_indexing() {
        let mut encoder = Encoder::default();
        let mut dst = Vec::new();
        encoder.encode_without_indexing(b"foo".to_vec(), b"bar".to_vec(), false, false, &mut dst).unwrap(); // (foo, bar), no Huffman
        assert_eq!(dst[0], 0); // not indexed
        assert_eq!(decode_string(&dst[1..5]), b"foo".to_vec()); // string name
        assert_eq!(decode_string(&dst[5..]), b"bar".to_vec()); // string value
        assert_eq!(encoder.table.len(), 61); // indexed
    }

    /// Should encode a new header which does not match any existing header in
    /// the table. The encoded data should indicate that it should never be
    /// indexed.
    #[test]
    fn encodes_never_indexed() {
        let mut encoder = Encoder::default();
        let mut dst = Vec::new();
        encoder.encode_never_indexed(b"foo".to_vec(), b"bar".to_vec(), false, false, &mut dst).unwrap(); // (foo, bar), no Huffman
        assert_eq!(dst[0], 0b00010000); // never indexed
        assert_eq!(decode_string(&dst[1..5]), b"foo".to_vec()); // huffman encoded name
        assert_eq!(decode_string(&dst[5..]), b"bar".to_vec()); // huffman encoded value
        assert_eq!(encoder.table.len(), 61); // indexed
    }
}
