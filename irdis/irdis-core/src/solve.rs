use crate::instance::{schedule::RunwaySchedule, Instance};

pub trait Solve {
    fn solve(&self, instance: &Instance) -> RunwaySchedule;
}
