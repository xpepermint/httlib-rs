use std::collections::VecDeque;

/// The [dynamic table] holding a list of header fields maintained in first-in,
/// first-out order.
/// 
/// [dynamic table]: https://tools.ietf.org/html/rfc7541#section-2.3.2
#[derive(Debug)]
pub struct DynamicTable {
    /// A sequential list of dynamic headers where the newest entry in at the
    /// lowest index. It can contain duplicate entries.
    inner: VecDeque<(Vec<u8>, Vec<u8>)>,

    /// The sum of the size of its entries in the table. The size of an entry is
    /// the sum of its name and value in octets without any Huffman encoding
    /// applied, and 32.
    size: usize,

    /// The maximum size that the encoder is permitted to use for the dynamic
    /// table. In HTTP/2, this value is advertised through the SETTINGS frame by
    /// the SETTINGS_HEADER_TABLE_SIZE field. The encoder can use less than or 
    /// equal to this value.
    max_size: u32,
}

impl DynamicTable {
    /// Return a new instance of the dynamic table. The function expects a
    /// parameter which will set the maximum allowed table size.
    pub fn with_size(max_size: u32) -> Self {
        Self {
            inner: VecDeque::new(),
            size: 0,
            max_size,
        }
    }

    /// Returns the total number of entries.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns the total size (in octets) of the table.
    pub fn size(&self) -> u32 {
        self.size as u32
    }

    /// Returns the maximum allowed table size.
    pub fn max_size(&self) -> u32 {
        self.max_size
    }

    /// Updates the maximum allowed table size.
    /// 
    /// Whenever the maximum size is reduced, entries are evicted from the end
    /// of the table until the size of the table is less than or equal to the
    /// maximum size.
    pub fn update_max_size(&mut self, size: u32) {
        self.max_size = size;

        self.consolidate(); // evict entries if necessary
    }

    /// Finds a header by its index.
    pub fn get(&self, index: u32) -> Option<(&[u8], &[u8])> {
        match self.inner.get(index as usize) {
            Some(h) => Some((&h.0, &h.1)),
            None => None,
        }
    }

    /// Inserts a new header at the beginning of the table.
    /// 
    /// When the header is added, the table size is automatically increased. The
    /// size of an entry is the sum of its name and value in octets without any
    /// Huffman encoding applied, and 32 ([4.1.]).
    /// 
    /// Before a new entry is added to the dynamic table, entries are evicted
    /// from the end of the table until the size of the table is less than or
    /// equal to the maximum allowed size or until the table is empty.
    ///
    /// If the size of the new entry is less than or equal to the maximum size,
    /// that entry is added to the table. Adding an entry larger than the
    /// maximum size causes the table to be emptied.
    /// 
    /// [4.1.]: https://tools.ietf.org/html/rfc7541#section-4.1
    pub fn insert(&mut self, name: Vec<u8>, value: Vec<u8>) {
        self.size += name.len() + value.len() + 32;
        self.inner.push_front((name, value));

        self.consolidate(); // evict entries if necessary
    }

    /// Consolidates the table entries so that the table size is below the
    /// maximum allowed size, by evicting headers from the table in a FIFO
    /// fashion.
    fn consolidate(&mut self) {
        while self.size > self.max_size as usize {
            if let Some(header) = self.inner.pop_back() {
                self.size -= header.0.len() + header.1.len() + 32;
            }
        }
    }
}

impl Default for DynamicTable {
    fn default() -> Self {
        Self::with_size(4096)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// A header should be added to the beginning of the table and the existing
    /// header indexes should be moved. Each added entry should increase the
    /// table size and length.
    #[test]
    fn inserts_new_headers() {
        let mut tbl = DynamicTable::default();
        assert_eq!(tbl.len(), 0);
        assert_eq!(tbl.size(), 0);
        tbl.insert(b"a0".to_vec(), b"b0".to_vec());
        assert_eq!(tbl.len(), 1);
        assert_eq!(tbl.size(), 36);
        tbl.insert(b"a1".to_vec(), b"b1".to_vec());
        assert_eq!(tbl.len(), 2);
        assert_eq!(tbl.size(), 72);
        let h1 = tbl.get(0).unwrap();
        let h0 = tbl.get(1).unwrap();
        assert_eq!(vec![h1.0, h1.1], vec![b"a1", b"b1"]);
        assert_eq!(vec![h0.0, h0.1], vec![b"a0", b"b0"]);
    }

    /// When the maximum table capacity is reached while adding a new header,
    /// entries should be evicted from the end of the table until the size of
    /// the table is less than or equal to the maximum allowed size or until the
    /// table is empty. Adding an entry larger than the maximum size should
    /// cause the table to be emptied.
    #[test]
    fn evicts_headers_on_insert() {
        let mut tbl = DynamicTable::with_size(36); // 36 is the size of the header ("xx", "xx")
        tbl.insert(b"a0".to_vec(), b"b0".to_vec()); // should be added
        assert_eq!(tbl.len(), 1);
        assert_eq!(tbl.size(), 36);
        tbl.insert(b"a1".to_vec(), b"b1".to_vec()); // should evict the previous entry
        assert_eq!(tbl.len(), 1);
        assert_eq!(tbl.size(), 36);
        tbl.insert(b"big".to_vec(), b"big".to_vec()); // big header should empty the table
        assert_eq!(tbl.len(), 0);
        assert_eq!(tbl.size(), 0);
    }

    /// When the maximum table capacity is decreased, entries should be evicted
    /// from the end of the table until the size of the table is less than or
    /// equal to the new allowed size or until the table is empty.
    #[test]
    fn evicts_headers_on_size_update() {
        let mut tbl = DynamicTable::with_size(108); // 108 can hold up to 3 headers
        tbl.insert(b"a0".to_vec(), b"b0".to_vec());
        tbl.insert(b"a1".to_vec(), b"b1".to_vec());
        tbl.insert(b"a2".to_vec(), b"b2".to_vec());
        assert_eq!(tbl.len(), 3);
        assert_eq!(tbl.size(), 108);
        tbl.update_max_size(40); // remove last 2 headers
        assert_eq!(tbl.len(), 1);
        assert_eq!(tbl.size(), 36);
        let h2 = tbl.get(0).unwrap();
        assert_eq!(vec![h2.0, h2.1], vec![b"a2", b"b2"]);
    }
}
