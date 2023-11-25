use std::str::FromStr;

use dissertation::{bnb, instance::Instance, visual::visualise};

fn main() {
    let instance = Instance::from_str(include_str!("../instances/instance.csv")).unwrap();

    let sequence = bnb::branch_and_bound(&instance);
    println!("{:#?}", sequence);

    let doc = visualise(&sequence, &instance).unwrap();
    svg::save("visuals/instance.svg", &doc).unwrap();
}
