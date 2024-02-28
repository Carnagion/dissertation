use std::num::NonZeroUsize;

use irdis::{instance::Instance, solve::BranchBound, vis::Visualiser};
use irdis_instance::Solve;

fn main() {
    let toml = include_str!("../instances/furini/converted/1.toml");
    let instance = toml::from_str::<Instance>(toml).unwrap();

    // NOTE: Using a large horizon produces better results especially when using
    //       `earliest()` instead of `target` as the objective (see `src/cost.rs`).
    let branch_bound = BranchBound::with_rolling_horizon(NonZeroUsize::new(12).unwrap());
    let schedule = branch_bound.solve(&instance).unwrap();
    println!("{:?}", schedule);

    let vis = Visualiser::new();
    let doc = vis.visualise(&schedule, &instance).unwrap();
    svg::save("visuals/furini/converted/1.svg", &doc).unwrap();
}
