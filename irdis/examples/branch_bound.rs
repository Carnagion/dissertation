use std::num::NonZeroUsize;

use irdis::{
    instance::Instance,
    solve::{BranchBound, Solve},
    vis::Visualiser,
};

fn main() {
    let toml = include_str!("../instances/furini/toml/1.toml");
    let instance = toml::from_str::<Instance>(toml).unwrap();

    let branch_bound = BranchBound {
        horizon: NonZeroUsize::new(12),
        ..BranchBound::default()
    };
    let solution = branch_bound.solve(&instance).unwrap();
    println!("{:?}", solution);

    let vis = Visualiser::new();
    let doc = vis.visualise(&solution, &instance).unwrap();
    svg::save("visuals/furini/1.svg", &doc).unwrap();
}
