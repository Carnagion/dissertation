use irdis::{instance::Instance, solve::BranchBound, vis::Visualiser};

fn main() {
    let toml = include_str!("../instances/furini/limited/1.toml");
    let instance = toml::from_str::<Instance>(toml).unwrap();
    let schedule = instance.solve::<BranchBound>();
    let vis = Visualiser::new();
    let doc = vis.visualise(&schedule, &instance).unwrap();
    svg::save("visuals/furini/limited/1.svg", &doc).unwrap();
}
