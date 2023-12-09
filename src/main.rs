use std::str::FromStr;

use dissertation::{bnb, instance::Instance, visual::visualise};

fn main() {
    let mut instance = Instance::from_str(include_str!("../instances/instance.csv")).unwrap();
    instance.randomize_times(&mut rand::thread_rng());

    let sequence = bnb::branch_and_bound(&instance);
    println!("{:#?}", sequence);

    let doc = visualise(&sequence, &instance).unwrap();
    svg::save("visuals/instance.svg", &doc).unwrap();
}

fn convert_all_furini() {
    use std::{fs, path::Path};

    use csv::WriterBuilder;

    for idx in 1..=12 {
        let aircraft_path = Path::new("instances/").join(format!("FPT{:0>2}.txt", idx));
        let aircraft = fs::read_to_string(&aircraft_path).unwrap();

        let separations_path =
            Path::new("instances/sep/").join(format!("info_matrix_FPT{:0>2}.txt.txt", idx));
        let separations = fs::read_to_string(separations_path).unwrap();

        let mut instance = instance_from_furini(&aircraft, &separations);
        instance.randomize_times(&mut rand::thread_rng());

        let instance_path = aircraft_path.with_file_name(format!("instance-{}.csv", idx));

        let mut writer = WriterBuilder::new()
            .has_headers(false)
            .from_path(&instance_path)
            .unwrap();
        for row in instance.0 {
            writer.serialize(row).unwrap();
        }
        writer.flush().unwrap();

        Instance::from_str(&fs::read_to_string(instance_path).unwrap()).unwrap();
    }
}

fn instance_from_furini(aircraft: &str, separations: &str) -> Instance {
    use std::time::Duration;

    use chrono::NaiveTime;

    use dissertation::instance::{
        aircraft::{Aircraft, Model, Registration, SizeClass},
        constraints::DepartureConstraints,
        InstanceRow,
    };

    let mut aircraft = aircraft.lines();
    let separations = separations.lines();

    let num_aircraft = aircraft.next().unwrap().parse().unwrap();
    let mut rows = Vec::with_capacity(num_aircraft);

    for (aircraft, separations) in aircraft.zip(separations).take(10) {
        let mut parts = aircraft.split_ascii_whitespace();

        let reg = Registration::new(parts.next().unwrap().to_owned());
        let model = Model::new(parts.next().unwrap().to_owned());
        let size_class = match parts.next().unwrap() {
            "M" => SizeClass::Medium,
            "L" => SizeClass::Large,
            _ => unreachable!(),
        };

        let aircraft = Aircraft {
            model,
            reg,
            size_class,
        };

        parts.next();

        let earliest_time = NaiveTime::parse_from_str(parts.next().unwrap(), "%H%M").unwrap();
        let zero = Duration::ZERO;

        let constraints = DepartureConstraints {
            earliest_time,
            pushback_dur: zero,
            pre_de_ice_dur: zero,
            de_ice_dur: zero,
            post_de_ice_dur: zero,
            lineup_dur: zero,
        };

        let separations = separations
            .split_ascii_whitespace()
            .skip(1)
            .take(10)
            .map(|sep| Duration::from_secs(sep.parse::<u64>().unwrap() * 60))
            .collect();

        rows.push(InstanceRow {
            aircraft,
            constraints,
            separations,
        });
    }

    Instance(rows)
}
