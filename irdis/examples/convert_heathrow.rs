use std::fs;

use irdis::data::from_heathrow_with_limit;

fn main() {
    let flights = fs::read_to_string("instances/heathrow/flights.csv").unwrap();
    let runway_configs =
        fs::read_to_string("instances/heathrow/runway-configurations.csv").unwrap();
    let pushback_durs = fs::read_to_string("instances/heathrow/pushback-durations.csv").unwrap();
    let taxi_configs = fs::read_to_string("instances/heathrow/taxi-durations.csv").unwrap();
    let separation_configs =
        fs::read_to_string("instances/heathrow/runway-separations.csv").unwrap();

    let instances = from_heathrow_with_limit(
        &flights,
        &pushback_durs,
        &taxi_configs,
        &runway_configs,
        &separation_configs,
        4013,
    )
    .unwrap();

    println!("{:?}", instances.len());
}
