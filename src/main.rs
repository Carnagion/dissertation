use std::str::FromStr;

use dissertation::{bnb, instance::Instance};

fn main() {
    let instance = Instance::from_str(include_str!("../instances/instance.csv")).unwrap();
    println!("{:#?}", bnb::branch_and_bound(&instance));
}
