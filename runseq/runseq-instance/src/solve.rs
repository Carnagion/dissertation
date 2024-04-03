use crate::{schedule::Schedule, Instance};

pub trait Solve {
    fn solve(&self, instance: &Instance) -> Option<Vec<Schedule>>;
}
