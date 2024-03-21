use std::{fs, num::NonZeroUsize, path::Path};

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
}

fn load_instance(path: impl AsRef<Path>) -> Instance {
    let toml = fs::read_to_string(path).unwrap();
    let instance = toml::from_str::<Instance>(&toml).unwrap();
    instance
}

// TODO: Save these stats in a file somewhere
fn save_stats(solution: &[Schedule], instance: &Instance) {
    let start = solution.iter().map(|sched| sched.flight_time()).min();
    let end = solution.iter().map(|sched| sched.flight_time()).max();

    let deice_start = solution
        .iter()
        .filter_map(Schedule::as_departure)
        .map(|sched| sched.deice)
        .min();
    let deice_end = solution
        .iter()
        .filter_map(Schedule::as_departure)
        .map(|sched| sched.deice)
        .max();

    let cost = branch_bound::solution_cost(&solution, &instance);
}

mod furini {
    use super::*;

    // NOTE: De-icing by TOBT fails on instances FPT06 and FPT07
    #[divan::bench(args = [1, 2, 3, 4, 5, 8, 9, 10, 11, 12])]
    fn deice_by_tobt(bencher: Bencher, instance: usize) {
        let instance = load_instance(format!("instances/furini/toml/{}.toml", instance));

        let branch_bound = BranchBound {
            horizon: NonZeroUsize::new(12),
            deice_strategy: DeiceStrategy::ByTobt,
        };

        let mut solutions = Vec::with_capacity(12);

        bencher.bench_local(|| {
            let solution = branch_bound.solve(&instance).unwrap();
            solutions.push(solution);
        });

        for solution in solutions {
            save_stats(&solution, &instance);
        }
    }

    // NOTE: De-icing by CTOT fails on instances FPT06 and FPT07
    #[divan::bench(args = [1, 2, 3, 4, 5, 8, 9, 10, 11, 12])]
    fn deice_by_ctot(bencher: Bencher, instance: usize) {
        let instance = load_instance(format!("instances/furini/toml/{}.toml", instance));

        let branch_bound = BranchBound {
            horizon: NonZeroUsize::new(12),
            deice_strategy: DeiceStrategy::ByCtot,
        };

        let mut solutions = Vec::with_capacity(12);

        bencher.bench_local(|| {
            let solution = branch_bound.solve(&instance).unwrap();
            solutions.push(solution);
        });

        for solution in solutions {
            save_stats(&solution, &instance);
        }
    }

    #[divan::bench(args = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12])]
    fn deice_integrated(bencher: Bencher, instance: usize) {
        let instance = load_instance(format!("instances/furini/toml/{}.toml", instance));

        let branch_bound = BranchBound {
            horizon: NonZeroUsize::new(12),
            deice_strategy: DeiceStrategy::Integrated,
        };

        let mut solutions = Vec::with_capacity(12);

        bencher.bench_local(|| {
            let solution = branch_bound.solve(&instance).unwrap();
            solutions.push(solution);
        });

        for solution in solutions {
            save_stats(&solution, &instance);
        }
    }
}

mod heathrow {
    use super::*;

    #[divan::bench(args = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10])]
    fn deice_by_tobt(bencher: Bencher, instance: usize) {
        let instance = load_instance(format!("instances/heathrow/toml/{}.toml", instance));

        let branch_bound = BranchBound {
            horizon: NonZeroUsize::new(10),
            deice_strategy: DeiceStrategy::ByTobt,
        };

        let mut solutions = Vec::with_capacity(10);

        bencher.bench_local(|| {
            let solution = branch_bound.solve(&instance).unwrap();
            solutions.push(solution);
        });

        for solution in solutions {
            save_stats(&solution, &instance);
        }
    }

    #[divan::bench(args = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10])]
    fn deice_by_ctot(bencher: Bencher, instance: usize) {
        let instance = load_instance(format!("instances/heathrow/toml/{}.toml", instance));

        let branch_bound = BranchBound {
            horizon: NonZeroUsize::new(10),
            deice_strategy: DeiceStrategy::ByCtot,
        };

        let mut solutions = Vec::with_capacity(10);

        bencher.bench_local(|| {
            let solution = branch_bound.solve(&instance).unwrap();
            solutions.push(solution);
        });

        for solution in solutions {
            save_stats(&solution, &instance);
        }
    }

    #[divan::bench(args = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10])]
    fn deice_integrated(bencher: Bencher, instance: usize) {
        let instance = load_instance(format!("instances/heathrow/toml/{}.toml", instance));

        // TODO: Increase horizon size after adding more pruning rules
        let branch_bound = BranchBound {
            horizon: NonZeroUsize::new(7),
            deice_strategy: DeiceStrategy::Integrated,
        };

        let mut solutions = Vec::with_capacity(10);

        bencher.bench_local(|| {
            let solution = branch_bound.solve(&instance).unwrap();
            solutions.push(solution);
        });

        for solution in solutions {
            save_stats(&solution, &instance);
        }
    }
}
