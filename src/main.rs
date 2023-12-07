use std::str::FromStr;

use dissertation::{bnb, instance::Instance, visual::visualise};

fn main() {
    let mut instance = Instance::from_str(include_str!("../instances/instance.csv")).unwrap();
    instance.randomize_times(&mut rand::thread_rng());

    let sequence = bnb::branch_and_bound(&instance);
    println!("{:#?}", sequence);

    let doc = visualise(&sequence, &instance).unwrap();
    svg::save("visuals/instance.svg", &doc).unwrap();
}
