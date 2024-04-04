use std::fs;

use serde::Deserialize;

use runseq::{
    instance::{schedule::Schedule, Instance},
    vis::Visualiser,
};

fn main() {
    // NOTE: We need this since TOML files always deserialize to a document/map, and so we cannot
    //       directly deserialize into a `Vec<Schedule>`.
    #[derive(Deserialize)]
    struct Solution {
        schedules: Vec<Schedule>,
    }

    let instance_toml = fs::read_to_string("../instances/heathrow/toml/1.toml").unwrap();
    let instance = toml::from_str::<Instance>(&instance_toml).unwrap();

    let solution_toml = fs::read_to_string("../solutions/heathrow/10.toml").unwrap();
    let solution = toml::from_str::<Solution>(&solution_toml)
        .unwrap()
        .schedules;

    let vis = Visualiser::new();
    let doc = vis.visualise(&solution, &instance).unwrap();
    svg::save("../visuals/heathrow/10.svg", &doc).unwrap();
}
