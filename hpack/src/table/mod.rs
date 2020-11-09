mod dynamic;
mod iter;
mod r#static;

pub use iter::TableIter;
use dynamic::DynamicTable;
use r#static::{StaticTable, STATIC_TABLE};

/// This table represents a single index address space for headers where the 
/// static and the dynamic table are combined.
#[derive(Debug)]
pub struct Table<'a> {
    /// THe static table with predefined headers.
    static_table: StaticTable<'a>,

    /// The dynamic table holding custom headers.
    dynamic_table: DynamicTable,
}

impl<'a> Table<'a> {
    /// Returns a new header table instance with the provided maximum allowed
    /// size of the dynamic table.
    pub fn with_dynamic_size(max_dynamic_size: u32) -> Self {
        Self {
            static_table: STATIC_TABLE,
            dynamic_table: DynamicTable::with_size(max_dynamic_size),
        }
    }

    /// Returns the total number of headers. The result includes the sum of all
    /// entries of the static and the dynamic table combined.
    pub fn len(&self) -> usize {
        self.static_table.len() + self.dynamic_table.len()
    }

    /// Returns the total number of entries stored in the dynamic table.
    pub fn dynamic_len(&self) -> usize {
        self.dynamic_table.len()
    }

    /// Returns the total size (in octets) of all the entries stored in the
    /// dynamic table.
    pub fn dynamic_size(&self) -> u32 {
        self.dynamic_table.size()
    }

    /// Returns the maximum allowed size of the dynamic table.
    pub fn max_dynamic_size(&mut self) -> u32 {
        self.dynamic_table.max_size()
    }
    
    /// Updates the maximum allowed size of the dynamic table.
    pub fn update_max_dynamic_size(&mut self, size: u32) {
        self.dynamic_table.update_max_size(size);
    }

    /// Returns an iterator through all the headers.
    /// 
    /// It includes entries stored in the static and the dynamic table. Since
    /// the index `0` is an invalid index, the first returned item is at index
    /// `1`. The entries returned have indices ordered sequentially in the
    /// single address space (first the headers in the static table, followed by
    /// headers in the dynamic table).
    pub fn iter(&'a self) -> TableIter<'a> {
        TableIter{ index: 1, table: &self }
    }

    /// Finds a header by its index.
    /// 
    /// According to the HPACK specification, the index `0` must be treated as
    /// an invalid index number. The value for index `0` in the static table is
    /// thus missing. The index of `0` will always return `None`.
    pub fn get(&self, index: u32) -> Option<(&[u8], &[u8])> {
        let index = if index == 0 {
            return None;
        } else {
            index - 1
        };

        let static_len = self.static_table.len() as u32;
        if index < static_len {
            Some(self.static_table[index as usize])
        } else {
            self.dynamic_table.get(index - static_len)
        }
    }

    /// Searches the static and the dynamic tables for the provided header. It
    /// tries to match both the header name and value to one of the headers in
    /// the table. If no such header exists, then it falls back to the one that
    /// matched only the name. The returned match contains the index of the
    /// header in the table and a boolean indicating whether the value of the
    /// header also matched.
    pub fn find(&self, name: &[u8], value: &[u8]) -> Option<(usize, bool)> {
        let mut name_match = None;
        for (i, h) in self.iter().enumerate() {
            if name == h.0 {
                if value == h.1 {
                    return Some((i + 1, true)); // name and value matched
                } else if name_match.is_none() {
                    name_match = Some(i + 1); // only name mached
                }
            }
        }
        match name_match {
            Some(i) => Some((i, false)),
            None => None,
        }
    }

    /// Inserts a new header at the beginning of the dynamic table.
    pub fn insert(&mut self, name: Vec<u8>, value: Vec<u8>) {
        self.dynamic_table.insert(name, value);
    }
}

impl<'a> Default for Table<'a> {
    fn default() -> Self {
        Self {
            static_table: STATIC_TABLE,
            dynamic_table: DynamicTable::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// A header should be added to the beginning of the dynamic table. The
    /// first added header should have index `62`. The table should increase
    /// in size and length after each insertion.
    #[test]
    fn inserts_new_headers() {
        let mut tbl = Table::default();
        assert_eq!(tbl.len(), 61);
        assert_eq!(tbl.dynamic_len(), 0);
        assert_eq!(tbl.dynamic_size(), 0);
        tbl.insert(b"a0".to_vec(), b"b0".to_vec());
        assert_eq!(tbl.len(), 62);
        assert_eq!(tbl.dynamic_len(), 1);
        assert_eq!(tbl.dynamic_size(), 36);
        tbl.insert(b"a1".to_vec(), b"b1".to_vec());
        assert_eq!(tbl.len(), 63);
        assert_eq!(tbl.dynamic_len(), 2);
        assert_eq!(tbl.dynamic_size(), 72);
    }

    /// The returned iterator should walk through all entries in the static and
    /// the dynamic table combined.
    #[test]
    fn iters_through_all_headers() {
        let mut tbl = Table::default();
        tbl.insert(b"a0".to_vec(), b"b0".to_vec());
        let iter = tbl.iter();
        assert_eq!(iter.count(), 62); // 61 static + 1 dynamic
        let last = iter.last().unwrap();
        assert_eq!(vec![last.0, last.1], vec![b"a0", b"b0"]);
    }

    /// The table should be able to search for headers in the static or the
    /// dynamic table. Index `0` should always return `None` because it does not
    /// represent a valid index.
    #[test]
    fn find_header_by_index() {
        let mut tbl = Table::default();
        tbl.insert(b"a0".to_vec(), b"b0".to_vec());
        assert_eq!(tbl.get(0), None); // invalid index
        let h1 = tbl.get(1).unwrap();
        assert_eq!(vec![h1.0, h1.1], vec![b":authority".to_vec(), vec![]]);
        let h2 = tbl.get(2).unwrap();
        assert_eq!(vec![h2.0, h2.1], vec![b":method".to_vec(), b"GET".to_vec()]);
        let h61 = tbl.get(61).unwrap();
        assert_eq!(vec![h61.0, h61.1], vec![b"www-authenticate".to_vec(), vec![]]);
        let h62 = tbl.get(62).unwrap();
        assert_eq!(vec![h62.0, h62.1], vec![b"a0", b"b0"]);
    }

    /// The table should search the static and the dynamic tables for a possible
    /// header match. It should try to match both the header name and value to
    /// one of the headers in the table. If no such header exists, then it
    /// should fall back to the one that matched only the name.
    #[test]
    fn find_header_match() {
        let mut tbl = Table::default();
        tbl.insert(b"a".to_vec(), b"b".to_vec()); // index: 63
        tbl.insert(b"a".to_vec(), b"c".to_vec()); // index: 62
        let m = tbl.find(b":method", b"POST").unwrap(); // fully indexed
        assert_eq!(m.0, 3); // at index 3
        assert_eq!(m.1, true); // name and value mached
        let m = tbl.find(b"a", b"b").unwrap(); // fully indexed
        assert_eq!(m.0, 63); // at index 63
        assert_eq!(m.1, true); // name and value mached
        let m = tbl.find(b":method", b"DELETE").unwrap(); // indexed name
        assert_eq!(m.0, 2); // at index 2
        assert_eq!(m.1, false); // only name mached
        let m = tbl.find(b"a", b"x").unwrap(); // indexed name
        assert_eq!(m.0, 62); // at index 62
        assert_eq!(m.1, false); // only name mached
        let m = tbl.find(b"x", b"x"); // not indexed
        assert_eq!(m, None); // not found
    }
}
