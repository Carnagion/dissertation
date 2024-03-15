use std::{iter, num::NonZeroUsize, ops::Range, time::Duration};

use chrono::NaiveTime;

use either::{Left, Right};

use irdis_instance::{
    flight::{Arrival, Departure, Flight},
    schedule::{ArrivalSchedule, DepartureSchedule, Schedule},
    Instance,
};

use crate::{
    complete_orders::separation_identical_sets,
    cost::{arrival_cost, departure_cost, Cost},
};

// TODO: Generate de-ice times by sorting departures by release time, then going through that list
//       and allocating a de-ice time to each departure as the max of previous de-ice finish and its
//       release time. Then perform a branch-and-bound over the aircraft with de-ice times being chosen
//       from this pre-generated de-ice queue. Then perform post-processing on the solution if necessary
//       to optimize for stand holding.

struct DeiceSchedule {
    flight_idx: usize,
    deice: NaiveTime,
}

fn generate_deice(instance: &Instance) -> Option<Vec<DeiceSchedule>> {
    let mut departures = instance
        .flights()
        .iter()
        .enumerate()
        .filter_map(|(idx, flight)| flight.as_departure().zip(Some(idx)))
        .collect::<Vec<_>>();
    departures.sort_unstable_by_key(|(dep, _)| dep.release_time());

    let &(first_dep, first_dep_idx) = departures.first()?;
    let first_deice = first_dep.release_time()
        - instance.max_slack_dur
        - first_dep.lineup_dur
        - first_dep.taxi_out_dur
        - first_dep.deice_dur;
    let first_sched = DeiceSchedule {
        flight_idx: first_dep_idx,
        deice: first_deice,
    };

    let deice_queue = departures
        .into_iter()
        .scan(first_sched, |last_deice, (dep, dep_idx)| {
            let last_dep = instance.flights()[last_deice.flight_idx]
                .as_departure()
                .unwrap();
            let deice = (last_deice.deice + last_dep.deice_dur).max(
                dep.release_time()
                    - instance.max_slack_dur
                    - dep.lineup_dur
                    - dep.taxi_out_dur
                    - dep.deice_dur,
            );

            last_deice.deice = deice;
            last_deice.flight_idx = dep_idx;

            Some(DeiceSchedule {
                flight_idx: dep_idx,
                deice,
            })
        })
        .collect();

    Some(deice_queue)
}
