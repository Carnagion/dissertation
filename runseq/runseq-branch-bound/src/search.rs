use std::{ops::Range, time::Duration};

use chrono::NaiveDateTime;

use runseq_instance::{
    flight::{Flight, TimeWindow},
    schedule::Schedule,
    Instance,
};

use crate::{
    complete_orders::separation_identical_complete_orders,
    cost::{arrival_cost, departure_cost, estimated_remaining_cost, Cost},
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Node {
    pub sched: Schedule,
    pub depth: usize,
    pub complete_order_idx: usize,
    pub cost: Cost,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct BranchBoundState {
    pub complete_order_sets: Vec<Vec<usize>>,
    pub next_in_complete_order_sets: Vec<usize>,
    pub current_solution: Vec<Node>,
    pub best_solution: Vec<Node>,
}

impl BranchBoundState {
    pub fn new(instance: &Instance) -> Self {
        let complete_order_sets = separation_identical_complete_orders(instance);
        let next_in_complete_order_sets = vec![0; complete_order_sets.len()];

        let current_solution = Vec::with_capacity(instance.flights().len());
        let best_solution = current_solution.clone();

        Self {
            complete_order_sets,
            next_in_complete_order_sets,
            current_solution,
            best_solution,
        }
    }
}

pub fn branch_bound<E, I>(
    instance: &Instance,
    state: &mut BranchBoundState,
    nodes: &mut Vec<Node>,
    expand: &mut E,
    window: Range<usize>,
) where
    E: FnMut(&Flight, usize, &Instance, &BranchBoundState) -> I,
    I: IntoIterator<Item = Schedule>,
{
    let mut current_cost = Cost::default();
    let mut best_cost = Cost::MAX;

    // Initialise the queue with initial states, taken from first aircraft in each complete-order set
    nodes.extend(branches(instance, state, expand));

    while let Some(node) = nodes.pop() {
        // Reset the cost and complete-order sets to match the current depth
        for removed in state.current_solution.drain(node.depth..) {
            state.next_in_complete_order_sets[removed.complete_order_idx] -= 1;
            current_cost -= removed.cost;
        }

        // Ignore the node and its sub-nodes if the lower bound is worse than the best known cost
        if current_cost + node.cost >= best_cost {
            continue;
        }

        let last_sched = node.sched.clone();

        // Update the cost and complete order sets, and add the aircraft to the current solution
        current_cost += node.cost;
        state.next_in_complete_order_sets[node.complete_order_idx] += 1;
        state.current_solution.push(node);

        // If we have sequenced all aircraft according to the rolling horizon size, then update the best solution.
        // We know that this solution will definitely be the best solution since it is a full solution and since the
        // costs are compared above.
        if state.current_solution.len() == window.end {
            best_cost = current_cost;
            state.best_solution = state.current_solution[window.clone()].to_vec();
            continue;
        }

        // Ignore the node and its sub-nodes if the current lower bound plus the estimated cost for the
        // remaining aircraft is worse than the best known cost
        if current_cost + estimated_remaining_cost(instance, state, &last_sched) >= best_cost {
            continue;
        }

        // Expand the node and add its sub-nodes to the queue
        nodes.extend(branches(instance, state, expand));
    }

    // Reset the current solution since we only need the best one
    state.current_solution.drain(window.start..);
}

fn branches<'a, E, I>(
    instance: &'a Instance,
    state: &'a BranchBoundState,
    expand: &'a mut E,
) -> impl Iterator<Item = Node> + 'a
where
    E: FnMut(&Flight, usize, &Instance, &BranchBoundState) -> I,
    I: IntoIterator<Item = Schedule> + 'a,
{
    let latest_release = state
        .current_solution
        .iter()
        .map(|node| {
            let flight_idx = node.sched.flight_index();
            instance.flights()[flight_idx].release_time()
        })
        .max();

    // Ignore aircraft whose latest times are before the latest release time in the current solution, since
    // that would mean that the aircraft cannot be sequenced anyways and should have been sequenced before
    let mut next_flights = state
        .complete_order_sets
        .iter()
        .enumerate()
        .filter_map(|(complete_order_idx, complete_order_set)| {
            let next_in_set_idx = state.next_in_complete_order_sets[complete_order_idx];

            let flight_idx = complete_order_set.get(next_in_set_idx).copied()?;
            let flight = &instance.flights()[flight_idx];

            match latest_release {
                None => Some((flight, flight_idx, complete_order_idx)),
                Some(latest_release)
                    if flight
                        .window()
                        .is_some_and(|window| window.latest() <= latest_release) =>
                {
                    None
                },
                Some(_) => Some((flight, flight_idx, complete_order_idx)),
            }
        })
        .collect::<Vec<_>>();

    // Sort the candidate sub-nodes by the release time of the aircraft, since most of the time the aircraft with the earliest
    // release time will be scheduled first
    next_flights.sort_unstable_by_key(|(flight, ..)| flight.release_time());

    // Like above - ignore aircraft whose earliest times are after the minimum latest time of any aircraft that are
    // also being expanded, since that could lead to situations where an aircraft with a disjoint and later time window
    // is sequenced before one with an earlier time window
    let next_latest = next_flights
        .iter()
        .filter_map(|(flight, ..)| flight.window())
        .map(|window| window.latest())
        .min();
    if let Some(next_latest) = next_latest {
        next_flights.retain(|(flight, ..)| match flight.window() {
            None => true,
            Some(window) => window.earliest <= next_latest,
        });
    }

    next_flights
        .into_iter()
        .rev() // NOTE: Since the last added node is explored first, the best node should be added last.
               //       Here, the first node has the aircraft with the earliest release time among the candidate
               //       aircraft, and is thus potentially the best. Reversing the iterator ensures that it is
               //       the first node to be explored.
        .flat_map(move |(flight, flight_idx, complete_order_idx)| {
            expand(flight, flight_idx, instance, state)
                .into_iter()
                .map(move |sched| {
                    let cost = match (&sched, flight) {
                        (Schedule::Arr(sched), Flight::Arr(arr)) => arrival_cost(sched, arr),
                        (Schedule::Dep(sched), Flight::Dep(dep)) => departure_cost(sched, dep),
                        // PANICS: This case will never be reached, because none of the expansion
                        //         functions will ever schedule a departure when meant to be scheduling
                        //         an arrival and vice-versa.
                        _ => unreachable!(),
                    };

                    Node {
                        sched,
                        depth: state.current_solution.len(),
                        complete_order_idx,
                        cost,
                    }
                })
        })
}

// NOTE: Helper function to iterate between two `NaiveDateTime`s, since they do not impl `Step` which is
//       still an unstable trait.
pub fn iter_minutes(
    from: NaiveDateTime,
    to: NaiveDateTime,
) -> impl DoubleEndedIterator<Item = NaiveDateTime> {
    let diff = (to - from)
        .max(chrono::Duration::zero())
        .num_minutes()
        .unsigned_abs();
    (0..=diff).map(move |minute| from + Duration::from_secs(minute * 60))
}

pub fn within_window(time: NaiveDateTime, window: Option<&TimeWindow>) -> bool {
    match window {
        None => true,
        Some(window) => window.as_range().contains(&time),
    }
}
