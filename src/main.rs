use dissertation::{bnb, instance::Instance};

fn main() {
    let aircraft_constraints = include_str!("../instances/test.constraints");
    let separations = include_str!("../instances/sep/test.sep");
    let instance = Instance::parse(aircraft_constraints, separations).unwrap();

    bnb::branch_and_bound(&instance);
}
