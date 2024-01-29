use std::{fs, num::NonZeroUsize};

use irdis_branch_bound::BranchBound;

use irdis_core::instance::Instance;

fn main() {
    let csv = fs::read_to_string("instances/furini/converted/1.csv").unwrap();
    let instance = csv.parse::<Instance>().unwrap();

    let branch_bound = BranchBound::with_rolling_horizon(NonZeroUsize::new(12).unwrap());
    let schedule = instance.solve_with(&branch_bound);

    for op in schedule.0 {
        println!("{:?}", op);
    }
}
