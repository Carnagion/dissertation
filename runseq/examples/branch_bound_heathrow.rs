use std::{fs, num::NonZeroUsize, path::Path};

use csv::Writer;

use runseq::{
    branch_bound::{self, BranchBound, DeiceStrategy},
    instance::{schedule::Schedule, Instance},
    vis::Visualiser,
};

const FMT: &str = "%F %T";

fn main() {
    let deice_strategy = DeiceStrategy::Integrated;
    let branch_bound = BranchBound {
        horizon: NonZeroUsize::new(10),
        deice_strategy,
    };

    let vis = Visualiser::new();

    let mut csv = Writer::from_path("../stats/heathrow/deice-integrated.csv").unwrap();
    csv.write_record([
        "Instance",
        "Start",
        "End",
        "De-ice start",
        "De-ice end",
        "Obj. value",
    ])
    .unwrap();

    println!("solving using de-ice strategy = {:?}", deice_strategy);

    for id in 1..=30 {
        let instance_path = Path::new("../instances/heathrow/toml/").join(format!("{}.toml", id));
        let toml = fs::read_to_string(instance_path).unwrap();
        let instance = toml::from_str::<Instance>(&toml).unwrap();

        let Some(solution) = instance.solve_with(&branch_bound) else {
            println!("unable to solve instance {}", id);
            csv.write_record([
                id.to_string(),
                "".to_owned(),
                "".to_owned(),
                "".to_owned(),
                "".to_owned(),
                "".to_owned(),
            ])
            .unwrap();
            continue;
        };

        let start = solution
            .iter()
            .map(|sched| sched.flight_time())
            .min()
            .unwrap();
        let end = solution
            .iter()
            .map(|sched| sched.flight_time())
            .max()
            .unwrap();

        let deice_start = solution
            .iter()
            .filter_map(Schedule::as_departure)
            .filter_map(|sched| sched.deice)
            .min();
        let deice_end = solution
            .iter()
            .filter_map(Schedule::as_departure)
            .filter_map(|sched| sched.deice)
            .max();

        let cost = branch_bound::solution_cost(&solution, &instance);

        csv.write_record([
            id.to_string(),
            start.format(FMT).to_string(),
            end.format(FMT).to_string(),
            deice_start
                .map(|deice_start| deice_start.format(FMT).to_string())
                .unwrap_or_default(),
            deice_end
                .map(|deice_end| deice_end.format(FMT).to_string())
                .unwrap_or_default(),
            cost.as_u64().to_string(),
        ])
        .unwrap();

        println!("cost of solution to instance {} = {:?}", id, cost);

        let doc = vis.visualise(&solution, &instance).unwrap();
        svg::save(format!("../visuals/heathrow/{}.svg", id), &doc).unwrap();
    }

    csv.flush().unwrap();
}
