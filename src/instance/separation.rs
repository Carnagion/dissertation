use std::time::Duration;

pub struct SeparationMatrix {
    pub rows: Vec<SeparationRow>,
}

pub struct SeparationRow {
    pub id: SeparationId,
    pub separations: Vec<Duration>,
}

pub struct SeparationId(u8);
