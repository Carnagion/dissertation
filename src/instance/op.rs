use time::Time;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct OpConstraint {
    pub kind: OpKind,
    pub earliest_time: Time,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AssignedOp {
    pub aircraft_idx: usize,
    pub kind: OpKind,
    pub earliest_time: Time,
    pub time: Time,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum OpKind {
    Arrival,
    Departure,
}
