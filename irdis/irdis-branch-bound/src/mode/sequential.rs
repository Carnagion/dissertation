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

fn generate_deice(instance: &Instance) -> Vec<NaiveTime> {
    let mut departures = instance
        .flights()
        .iter()
        .filter_map(Flight::as_departure)
        .cloned()
        .collect::<Vec<_>>();
    departures.sort_unstable_by_key(|dep| dep.release_time());

    // TODO: Generate de-ice times by sorting departures by release time, then going through that list
    //       and allocating a de-ice time to each departure as the max of previous de-ice finish and its
    //       release time. Then perform a branch-and-bound over the aircraft with de-ice times being chosen
    //       from this pre-generated de-ice queue. Then perform post-processing on the solution if necessary
    //       to optimize for stand holding.
    todo!()
}
