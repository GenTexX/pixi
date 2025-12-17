#[derive(Clone)]
pub struct DocId(pub u64);

impl From<DocId> for u64 {
    fn from(id: DocId) -> Self {
        id.0
    }
}

impl Copy for DocId {}