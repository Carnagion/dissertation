use std::{num::NonZeroUsize, ops::Range, time::Duration};

use chrono::NaiveTime;

use irdis_instance::{flight::Flight, schedule::Schedule, Instance, Solve};

mod cost;
use cost::{arrival_cost, departure_cost, solution_cost, Cost};

mod complete_orders;

mod decomposed;

mod integrated;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct BranchBound {
    pub deice_mode: DeiceMode,
    pub horizon: Option<NonZeroUsize>,
}

impl BranchBound {
    pub fn new() -> Self {
        Self {
            deice_mode: DeiceMode::default(),
            horizon: None,
        }
    }
}

impl Default for BranchBound {
    fn default() -> Self {
        Self::new()
    }
}

impl Solve for BranchBound {
    fn solve(&self, instance: &Instance) -> Option<Vec<Schedule>> {
        let solution = match self.deice_mode {
            DeiceMode::Decomposed => {
                let deice_queue = decomposed::deice_queue(instance)?;
                branch_and_bound(
                    instance,
                    self.horizon,
                    |flight, flight_idx, instance, state| {
                        decomposed::expand(flight, flight_idx, instance, state, &deice_queue)
                    },
                )
            },
            DeiceMode::Integrated => branch_and_bound(instance, self.horizon, integrated::expand),
        }?;

        // TODO: Remove once testing is done
        println!("cost = {}", solution_cost(&solution, instance));

        Some(solution)
    }
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash)]
pub enum DeiceMode {
    Decomposed,
    #[default]
    Integrated,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct PartialNode {
    sched: Schedule,
    depth: usize,
    complete_order_idx: usize,
    cost: u64,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct BranchBoundState {
    complete_order_sets: Vec<Vec<usize>>,
    next_in_complete_order_sets: Vec<usize>,
    current_solution: Vec<PartialNode>,
    best_solution: Vec<PartialNode>,
}

fn branch_and_bound<E, I>(
    instance: &Instance,
    horizon: Option<NonZeroUsize>,
    mut expand: E,
) -> Option<Vec<Schedule>>
where
    E: FnMut(&Flight, usize, &Instance, &BranchBoundState) -> I,
    I: IntoIterator<Item = Schedule>,
{
    let complete_order_sets = crate::complete_orders::separation_identical_sets(instance);
    let next_in_complete_order_sets = vec![0; complete_order_sets.len()];

    let flight_count = instance.flights().len();

    let current_solution = Vec::with_capacity(flight_count);
    let best_solution = current_solution.clone();

    let end = horizon
        .map(usize::from)
        .unwrap_or(flight_count)
        .min(flight_count);

    let mut nodes = Vec::with_capacity(flight_count);

    let mut state = BranchBoundState {
        complete_order_sets,
        next_in_complete_order_sets,
        current_solution,
        best_solution,
    };

    branch_and_bound_with(instance, &mut state, &mut nodes, &mut expand, 0..end);

    let windows = (1..)
        .zip(end + 1..=flight_count)
        .map(|(start, end)| start..end);
    for window in windows {
        let fixed = state.best_solution.drain(..).next()?;
        let fixed_idx = fixed.sched.flight_index();

        state.current_solution.push(fixed);

        state.next_in_complete_order_sets.fill(0);
        for set in &mut state.complete_order_sets {
            set.retain(|&flight_idx| flight_idx != fixed_idx);
        }

        branch_and_bound_with(instance, &mut state, &mut nodes, &mut expand, window);
    }

    let solution = state
        .current_solution
        .into_iter()
        .chain(state.best_solution)
        .map(|node| node.sched)
        .collect();
    Some(solution)
}

fn branch_and_bound_with<E, I>(
    instance: &Instance,
    state: &mut BranchBoundState,
    nodes: &mut Vec<PartialNode>,
    expand: &mut E,
    window: Range<usize>,
) where
    E: FnMut(&Flight, usize, &Instance, &BranchBoundState) -> I,
    I: IntoIterator<Item = Schedule>,
{
    let mut cost = Cost::default();

    nodes.extend(branches(instance, state, expand, window.start));

    while let Some(node) = nodes.pop() {
        let depth = node.depth;

        for removed in state.current_solution.drain(depth..) {
            state.next_in_complete_order_sets[removed.complete_order_idx] -= 1;
            cost.current -= removed.cost;
        }

        if cost.current + node.cost >= cost.lowest {
            continue;
        }

        cost.current += node.cost;
        state.next_in_complete_order_sets[node.complete_order_idx] += 1;
        state.current_solution.push(node);

        if state.current_solution.len() == window.end {
            cost.lowest = cost.current;
            state.best_solution = state.current_solution[window.clone()].to_vec();
            continue;
        }

        nodes.extend(branches(instance, state, expand, depth + 1));
    }

    state.current_solution.drain(window.start..);
}

fn branches<'a, E, I>(
    instance: &'a Instance,
    state: &'a BranchBoundState,
    expand: &'a mut E,
    depth: usize,
) -> impl Iterator<Item = PartialNode> + 'a
where
    E: FnMut(&Flight, usize, &Instance, &BranchBoundState) -> I,
    I: IntoIterator<Item = Schedule> + 'a,
{
    let prev_sched_earliest = state.current_solution.last().map(|node| {
        let flight_idx = node.sched.flight_index();
        let flight = &instance.flights()[flight_idx];
        flight.time_window().earliest
    });

    // NOTE: Prunes nodes according to complete orders induced by disjoint time windows.
    //
    //       For example, let the last scheduled flight in the current solution be `i`. If
    //       a candidate `j` flight to be scheduled next has a time window that is disjoint
    //       with `i`'s  and the latest time of `j` is before the earliest time of `i`, then
    //       scheduling `j` after `i` would lead to an infeasible solution. Thus, any solution
    //       containing `j` after `i` can be pruned.
    let mut next_flights = state
        .complete_order_sets
        .iter()
        .enumerate()
        .filter_map(|(set_idx, complete_order_set)| {
            let next_in_set_idx = state.next_in_complete_order_sets[set_idx];

            let flight_idx = complete_order_set.get(next_in_set_idx).copied()?;
            let flight = &instance.flights()[flight_idx];

            match prev_sched_earliest {
                None => Some((flight, flight_idx, set_idx)),
                Some(prev_sched_earliest) if flight.time_window().latest <= prev_sched_earliest => {
                    None
                },
                Some(_) => Some((flight, flight_idx, set_idx)),
            }
        })
        .collect::<Vec<_>>();
    next_flights.sort_unstable_by_key(|(flight, ..)| flight.release_time());

    // NOTE: Prunes nodes according to complete orders induced by disjoint time windows.
    //
    //       For example, let the earliest flight among the set of candidate flights to be scheduled
    //       next be `i`. If there is any candidate flight `j` such that `j`'s time window is disjoint
    //       with `i`'s and the earliest time of `j` is after the latest time of `i`, then any feasible
    //       solution will always schedule `j` after `i`. Thus, `j` can be pruned from any candidate set
    //       that contains `i`.
    let next_latest = next_flights
        .iter()
        .map(|(flight, ..)| flight.time_window().latest)
        .min();
    if let Some(next_latest) = next_latest {
        next_flights.retain(|(flight, ..)| flight.time_window().earliest < next_latest);
    }

    next_flights
        .into_iter()
        .rev() // NOTE: Since the last added node is explored first, the best node should be added last.
               //       Here, the first node has the earliest time in the take-off or landing window, and is thus
               //       potentially the best and should be explored first. Reversing the iterator ensures this.
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

                    PartialNode {
                        sched,
                        depth,
                        complete_order_idx,
                        cost,
                    }
                })
        })
}

fn iter_minutes(from: NaiveTime, to: NaiveTime) -> impl DoubleEndedIterator<Item = NaiveTime> {
    let diff = (to - from)
        .max(chrono::Duration::zero())
        .num_minutes()
        .unsigned_abs();
    (0..=diff).map(move |minute| from + Duration::from_secs(minute * 60))
}
