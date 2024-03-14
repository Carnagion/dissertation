use serde::Deserialize;

use irdis::{
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

    let instance_toml = include_str!("../instances/furini/toml/1.toml");
    let instance = toml::from_str::<Instance>(instance_toml).unwrap();

    let solution_toml = include_str!("../solutions/furini/test.toml");
    let solution = toml::from_str::<Solution>(solution_toml).unwrap().schedules;

    let vis = Visualiser::new();
    let doc = vis.visualise(&solution, &instance).unwrap();
    svg::save("visuals/furini/test.svg", &doc).unwrap();
}
