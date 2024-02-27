use std::{
    fs::{self, File},
    io::Write,
    path::Path,
    time::Duration,
};

use irdis::instance::{
    flight::{Arrival, Departure, Flight},
    time::TimeWindow,
    Instance,
};

use chrono::NaiveTime;

fn main() {
    for id in 1..=12 {
        let flights_path =
            Path::new("instances/furini/original").join(format!("FPT{:0>2}.txt", id));
        let flights = fs::read_to_string(&flights_path).unwrap();

        let separations_path = Path::new("instances/furini/original/sep")
            .join(format!("info_matrix_FPT{:0>2}.txt.txt", id));
        let separations = fs::read_to_string(separations_path).unwrap();

        let instance = instance_from_furini(&flights, &separations, 60);

        let toml = toml::to_string(&instance).unwrap();
        let instance_path = flights_path
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join(format!("converted/{}.toml", id));

        let mut file = File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&instance_path)
            .unwrap();

        file.write_all(toml.as_bytes()).unwrap();
    }
}

fn instance_from_furini(flights: &str, separations: &str, limit: usize) -> Instance {
    let mut lines = flights.lines();
    let _flight_count = lines.next();

    let minute = Duration::from_secs(60);

    let flights = lines
        .filter(|line| !line.is_empty())
        .take(limit)
        .map(|line| {
            let mut parts = line.split_ascii_whitespace();

            let _registration = parts.next();
            let _model = parts.next();
            let _size_class = parts.next();

            let kind = parts.next();

            let target = NaiveTime::parse_from_str(parts.next().unwrap(), "%H%M").unwrap();

            let _unknown = parts.next();

            let flight = match kind {
                Some("A") => Flight::Arr(Arrival {
                    window: TimeWindow {
                        before: minute * 5,
                        target,
                        after: minute * 5,
                    },
                    taxi_in_dur: minute * 5,
                }),
                Some("D") => Flight::Dep(Departure {
                    ctot: TimeWindow {
                        before: minute * 5,
                        target,
                        after: minute * 10,
                    },
                    pushback_dur: minute * 5,
                    taxi_deice_dur: minute * 5,
                    deice_dur: minute * 5,
                    taxi_out_dur: minute * 5,
                    lineup_dur: minute * 5,
                }),
                _ => unreachable!(),
            };

            flight
        })
        .collect::<Vec<_>>();

    let separations = separations
        .lines()
        .filter(|line| !line.is_empty())
        .take(limit)
        .map(|line| {
            line.split_ascii_whitespace()
                .skip(1)
                .take(limit)
                .map(|num| Duration::from_secs(num.parse::<u64>().unwrap() * 60))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Instance::new(
        flights,
        separations.try_into().unwrap(),
        minute * 15,
        minute * 5,
    )
    .unwrap()
}
