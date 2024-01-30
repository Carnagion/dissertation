use std::fs;

use divan::Bencher;

use irdis::instance::Instance;

fn main() {
    divan::main();
}

#[divan::bench(consts = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12])]
fn from_csv<const INSTANCE: usize>(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let path = format!("instances/furini/converted/{}.csv", INSTANCE);
            fs::read_to_string(path).unwrap()
        })
        .bench_refs(|csv| csv.parse::<Instance>());
}
