use std::{
    fs::{self, File},
    io::Write,
    path::Path,
    time::Duration,
};

use chrono::NaiveTime;

use csv::WriterBuilder;

use irdis_core::instance::{
    aircraft::{Aircraft, Model, Registration, SizeClass},
    op::{ArrivalConstraints, DepartureConstraints, OpConstraints, OpKind},
    Instance,
    InstanceRow,
};

fn main() {
    for idx in 1..=12 {
        let aircraft_path =
            Path::new("instances/furini/original").join(format!("FPT{:0>2}.txt", idx));
        let aircraft = fs::read_to_string(&aircraft_path).unwrap();

        let separations_path = Path::new("instances/furini/original/sep")
            .join(format!("info_matrix_FPT{:0>2}.txt.txt", idx));
        let separations = fs::read_to_string(separations_path).unwrap();

        let instance = instance_from_furini(&aircraft, &separations, 60);

        let instance_path = aircraft_path
            .parent()
            .and_then(|parent| parent.parent())
            .unwrap()
            .join(format!("converted/{}.csv", idx));

        save_instance(instance, instance_path);

        let instance_limited = instance_from_furini(&aircraft, &separations, 10);

        let instance_limited_path = aircraft_path
            .parent()
            .and_then(|parent| parent.parent())
            .unwrap()
            .join(format!("limited/{}.csv", idx));

        save_instance(instance_limited, instance_limited_path);
    }
}

fn save_instance(instance: Instance, path: impl AsRef<Path>) {
    {
        let mut file = File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&path)
            .unwrap();

        file.write_all(b"# registration, model, size class, operation, earliest time, pushback duration (departures only), pre-de-ice taxi duration (departures only), de-ice duration (departures only), post-de-ice taxi duration (departures only), lineup duration (departures only), separations\n").unwrap();

        let mut writer = WriterBuilder::new()
            .has_headers(false)
            .comment(Some(b'#'))
            .from_writer(file);
        for row in instance.into_rows() {
            writer.serialize(row).unwrap();
        }
    }

    let csv = fs::read_to_string(path).unwrap();
    assert!(csv.parse::<Instance>().is_ok());
}

fn instance_from_furini(aircraft: &str, separations: &str, limit: usize) -> Instance {
    let mut aircraft = aircraft.lines();
    let separations = separations.lines();

    let num_aircraft = aircraft.next().unwrap().parse().unwrap();
    let mut rows = Vec::with_capacity(num_aircraft);

    for (aircraft, separations) in aircraft.zip(separations).take(limit) {
        let mut parts = aircraft.split_ascii_whitespace();

        let registration = Registration::new(parts.next().unwrap());
        let model = Model::new(parts.next().unwrap());
        let size_class = match parts.next().unwrap() {
            "S" => SizeClass::Small,
            "M" => SizeClass::Medium,
            "L" => SizeClass::Large,
            _ => panic!("invalid size class"),
        };

        let aircraft = Aircraft {
            registration,
            model,
            size_class,
        };

        let op_kind = match parts.next().unwrap() {
            "D" => OpKind::Departure,
            "A" => OpKind::Arrival,
            _ => unreachable!(),
        };

        let earliest_time = NaiveTime::parse_from_str(parts.next().unwrap(), "%H%M").unwrap();
        let five_minutes = Duration::from_secs(60 * 5);

        let constraints = match op_kind {
            OpKind::Departure => OpConstraints::Departure(DepartureConstraints {
                earliest_time,
                pushback_dur: five_minutes,
                pre_de_ice_dur: five_minutes,
                de_ice_dur: five_minutes,
                post_de_ice_dur: five_minutes,
                lineup_dur: five_minutes,
            }),
            OpKind::Arrival => OpConstraints::Arrival(ArrivalConstraints { earliest_time }),
        };

        let separations = separations
            .split_ascii_whitespace()
            .skip(1)
            .take(limit)
            .map(|sep| Duration::from_secs(sep.parse::<u64>().unwrap() * 60))
            .collect();

        rows.push(InstanceRow {
            aircraft,
            constraints,
            separations,
        });
    }

    Instance::new(rows).unwrap()
}
