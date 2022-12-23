use super::*;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Id(pub u64);

#[derive(Debug, Clone)]
pub struct IdGenerator {
    next_id: Id,
}

impl IdGenerator {
    pub fn new() -> Self {
        Self { next_id: Id(0) }
    }

    pub fn gen(&mut self) -> Id {
        let id = self.next_id;
        self.next_id.0 += 1;
        id
    }
}

impl Default for IdGenerator {
    fn default() -> Self {
        Self::new()
    }
}
