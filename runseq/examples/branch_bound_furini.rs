use std::{fs, num::NonZeroUsize, path::Path};

use csv::Writer;

use runseq::{
    branch_bound::{self, BranchBound, DeiceStrategy},
    instance::{schedule::Schedule, Instance},
    vis::Visualiser,
};

fn main() {
    let deice_strategy = DeiceStrategy::Integrated;
    let branch_bound = BranchBound {
        horizon: NonZeroUsize::new(10),
        deice_strategy,
    };

    let vis = Visualiser::new();

    let mut csv = Writer::from_path("../stats/furini/branch-bound/deice-integrated.csv").unwrap();
    csv.write_record([
        "Instance",
        "Makespan (s)",
        "De-ice start",
        "De-ice end",
        "Obj. value",
        "Runway hold (s)",
    ])
    .unwrap();

    println!("solving using de-ice strategy = {:?}", deice_strategy);

    for id in 1..=12 {
        let instance_path = Path::new("../instances/furini/toml/").join(format!("{}.toml", id));
        let toml = fs::read_to_string(instance_path).unwrap();
        let instance = toml::from_str::<Instance>(&toml).unwrap();

        let Some(solution) = instance.solve_with(&branch_bound) else {
            println!("unable to solve instance {}", id);
            csv.write_record([&format!("FPT{:0>2}", id), "", "", "", "", ""])
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
        let makespan = end - start;

        let deice_start = solution
            .iter()
            .filter_map(Schedule::as_departure)
            .filter_map(|sched| sched.deice)
            .min()
            .unwrap()
            .time();
        let deice_end = solution
            .iter()
            .filter_map(Schedule::as_departure)
            .filter_map(|sched| sched.deice)
            .max()
            .unwrap()
            .time();

        let cost = branch_bound::solution_cost(&solution, &instance);

        let runway_hold = solution
            .iter()
            .filter_map(|sched| {
                let sched = sched.as_departure()?;
                let dep = instance.flights()[sched.flight_index].as_departure()?;

                let deice_time = sched.deice.as_ref().copied()?;
                let deice_dur = dep.deice.as_ref()?.duration;

                let runway_hold = sched.takeoff
                    - dep.lineup_duration
                    - dep.taxi_duration
                    - deice_dur
                    - deice_time;

                Some(runway_hold.num_seconds().unsigned_abs())
            })
            .sum::<u64>();

        csv.write_record([
            format!("FPT{:0>2}", id),
            makespan.num_seconds().to_string(),
            deice_start.to_string(),
            deice_end.to_string(),
            cost.as_u64().to_string(),
            runway_hold.to_string(),
        ])
        .unwrap();

        println!("cost of solution to instance {} = {:?}", id, cost);

        let doc = vis.visualise(&solution, &instance).unwrap();
        svg::save(format!("../visuals/furini/{}.svg", id), &doc).unwrap();
    }

    csv.flush().unwrap();
}
