use std::num::NonZeroUsize;

use irdis::{
    instance::Instance,
    solve::{
        branch_bound::{self, BranchBound, DeiceStrategy},
        Solve,
    },
    vis::Visualiser,
};

fn main() {
    let toml = include_str!("../instances/furini/toml/1.toml");
    let instance = toml::from_str::<Instance>(toml).unwrap();

    let branch_bound = BranchBound {
        horizon: NonZeroUsize::new(12),
        deice_strategy: DeiceStrategy::Integrated,
    };
    let solution = branch_bound.solve(&instance).unwrap();
    println!("{:?}", solution);
    println!("{}", branch_bound::solution_cost(&solution, &instance));

    let vis = Visualiser::new();
    let doc = vis.visualise(&solution, &instance).unwrap();
    svg::save("visuals/furini/1.svg", &doc).unwrap();
}
