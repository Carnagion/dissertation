use time::Time;

pub struct Op {
    pub kind: OpKind,
    pub earliest_time: Time,
}

pub enum OpKind {
    Arrival,
    Departure,
}
