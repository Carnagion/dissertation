use std::{fs, num::NonZeroUsize, path::Path};

use csv::Writer;

use divan::Bencher;

use irdis::{
    instance::{schedule::Schedule, Instance},
    solve::{
        branch_bound::{self, BranchBound, DeiceStrategy},
        Solve,
    },
};

fn main() {
    divan::main();

    furini::record_all();
    heathrow::record_all();
}

fn load_instance(path: impl AsRef<Path>) -> Instance {
    let toml = fs::read_to_string(path).unwrap();
    let instance = toml::from_str::<Instance>(&toml).unwrap();
    instance
}

#[divan::bench_group(sample_count = 1, sample_size = 1)]
mod furini {
    use super::*;

    const FURINI_INSTANCES: &[usize] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

    const HORIZON: Option<NonZeroUsize> = NonZeroUsize::new(10);

    // NOTE: De-icing by TOBT fails on instances FPT06 and FPT07
    #[divan::bench(args = FURINI_INSTANCES)]
    fn deice_by_tobt(bencher: Bencher, instance: usize) {
        let instance = load_instance(format!("instances/furini/toml/{}.toml", instance));

        let branch_bound = BranchBound {
            horizon: HORIZON,
            deice_strategy: DeiceStrategy::ByTobt,
        };

        bencher.bench_local(|| {
            branch_bound.solve(&instance);
        });
    }

    // NOTE: De-icing by CTOT fails on instances FPT06 and FPT07
    #[divan::bench(args = FURINI_INSTANCES)]
    fn deice_by_ctot(bencher: Bencher, instance: usize) {
        let instance = load_instance(format!("instances/furini/toml/{}.toml", instance));

        let branch_bound = BranchBound {
            horizon: HORIZON,
            deice_strategy: DeiceStrategy::ByCtot,
        };

        bencher.bench_local(|| {
            branch_bound.solve(&instance);
        });
    }

    #[divan::bench(args = FURINI_INSTANCES)]
    fn deice_integrated(bencher: Bencher, instance: usize) {
        let instance = load_instance(format!("instances/furini/toml/{}.toml", instance));

        let branch_bound = BranchBound {
            horizon: HORIZON,
            deice_strategy: DeiceStrategy::Integrated,
        };

        bencher.bench_local(|| {
            branch_bound.solve(&instance);
        });
    }

    pub fn record_all() {
        let branch_bound = BranchBound {
            horizon: HORIZON,
            deice_strategy: DeiceStrategy::ByTobt,
        };
        save_stats(
            FURINI_INSTANCES,
            &branch_bound,
            "instances/furini/toml/",
            "stats/branch-bound-decomposed-furini.csv",
        );

        let branch_bound = BranchBound {
            horizon: HORIZON,
            deice_strategy: DeiceStrategy::Integrated,
        };
        save_stats(
            FURINI_INSTANCES,
            &branch_bound,
            "instances/furini/toml/",
            "stats/branch-bound-integrated-furini.csv",
        );
    }
}

mod heathrow {
    use super::*;

    const HEATHROW_INSTANCES: &[usize] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // TODO: Increase horizon size after adding more pruning rules
    const HORIZON: Option<NonZeroUsize> = NonZeroUsize::new(7);

    #[divan::bench(args = HEATHROW_INSTANCES)]
    fn deice_by_tobt(bencher: Bencher, instance: usize) {
        let instance = load_instance(format!("instances/heathrow/toml/{}.toml", instance));

        let branch_bound = BranchBound {
            horizon: HORIZON,
            deice_strategy: DeiceStrategy::ByTobt,
        };

        bencher.bench_local(|| {
            branch_bound.solve(&instance);
        });
    }

    #[divan::bench(args = HEATHROW_INSTANCES)]
    fn deice_by_ctot(bencher: Bencher, instance: usize) {
        let instance = load_instance(format!("instances/heathrow/toml/{}.toml", instance));

        let branch_bound = BranchBound {
            horizon: HORIZON,
            deice_strategy: DeiceStrategy::ByCtot,
        };

        bencher.bench_local(|| {
            branch_bound.solve(&instance);
        });
    }

    #[divan::bench(args = HEATHROW_INSTANCES)]
    fn deice_integrated(bencher: Bencher, instance: usize) {
        let instance = load_instance(format!("instances/heathrow/toml/{}.toml", instance));

        let branch_bound = BranchBound {
            horizon: HORIZON,
            deice_strategy: DeiceStrategy::Integrated,
        };

        bencher.bench_local(|| {
            branch_bound.solve(&instance);
        });
    }

    pub fn record_all() {
        let branch_bound = BranchBound {
            horizon: HORIZON,
            deice_strategy: DeiceStrategy::ByTobt,
        };
        save_stats(
            HEATHROW_INSTANCES,
            &branch_bound,
            "instances/heathrow/toml/",
            "stats/branch-bound-tobt-heathrow-small.csv",
        );

        let branch_bound = BranchBound {
            horizon: HORIZON,
            deice_strategy: DeiceStrategy::ByCtot,
        };
        save_stats(
            HEATHROW_INSTANCES,
            &branch_bound,
            "instances/heathrow/toml/",
            "stats/branch-bound-ctot-heathrow-small.csv",
        );

        let branch_bound = BranchBound {
            horizon: HORIZON,
            deice_strategy: DeiceStrategy::Integrated,
        };
        save_stats(
            HEATHROW_INSTANCES,
            &branch_bound,
            "instances/heathrow/toml/",
            "stats/branch-bound-integrated-heathrow-small.csv",
        );
    }
}

fn save_stats(
    instances: &[usize],
    branch_bound: &BranchBound,
    input: impl AsRef<Path>,
    output: impl AsRef<Path>,
) {
    let mut csv = Writer::from_path(output).unwrap();

    csv.write_record([
        "Instance",
        "Start",
        "End",
        "De-ice start",
        "De-ice end",
        "Obj. value",
    ])
    .unwrap();

    for id in instances {
        let instance = load_instance(input.as_ref().join(format!("{}.toml", id)));

        let solution = branch_bound.solve(&instance);
        let Some(solution) = solution else {
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
            .map(|sched| sched.deice)
            .min()
            .unwrap();
        let deice_end = solution
            .iter()
            .filter_map(Schedule::as_departure)
            .map(|sched| sched.deice)
            .max()
            .unwrap();

        let cost = branch_bound::solution_cost(&solution, &instance);

        csv.write_record([
            id.to_string(),
            start.format("%H:%M").to_string(),
            end.format("%H:%M").to_string(),
            deice_start.format("%H:%M").to_string(),
            deice_end.format("%H:%M").to_string(),
            cost.to_string(),
        ])
        .unwrap();
    }

    csv.flush().unwrap();
}
