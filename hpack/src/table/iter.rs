use super::Table;

/// An iterator through all the entries in the table. This iterator will first
/// walk through entries of the static table and then through entries of the
/// dynamic table.
#[derive(Clone, Copy)]
pub struct TableIter<'a> {
    pub index: u32,
    pub table: &'a Table<'a>,
}

impl<'a> Iterator for TableIter<'a> {
    type Item = (&'a [u8], &'a [u8]);
    
    fn next(&mut self) -> Option<(&'a [u8], &'a [u8])> {
        let res = self.table.get(self.index);
        self.index += 1;
        res
    }
}
