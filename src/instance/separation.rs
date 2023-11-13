use std::time::Duration;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SeparationMatrix {
    pub rows: Vec<SeparationRow>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SeparationRow {
    pub id: SeparationId,
    pub separations: Vec<Duration>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SeparationId(u8);
