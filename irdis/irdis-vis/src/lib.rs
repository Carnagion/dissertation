use std::{array, time::Duration};

use chrono::NaiveTime;

use svg::{
    node::element::{Group, Line, Rectangle, Style},
    Document,
};

use irdis_instance::{
    flight::{Arrival, Departure},
    schedule::{ArrivalSchedule, DepartureSchedule, Schedule},
    Instance,
};

const SCALE_X: u64 = 20;

const SCALE_Y: u64 = 20;

const FILL_BLACK: &str = "fill: #000000;";

const HOLLOW_BLACK: &str = "stroke: #000000; fill-opacity: 0;";

const FILL_GREY: &str = "fill: #cccccc;";

const FILL_RED: &str = "fill: #c70039;";

const FILL_BLUE: &str = "fill: #1363df";

const FILL_YELLOW: &str = "fill: #ffd23f;";

const HM: &str = "%H:%M";

macro_rules! title {
    ($($arg:tt)*) => {
        ::svg::node::element::Title::new()
            .add(::svg::node::Text::new(::std::format!($($arg)*)))
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub struct Visualiser {}

impl Visualiser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn visualise(&self, schedule: &[Schedule], instance: &Instance) -> Option<Document> {
        let starting_time = starting_time(schedule, instance)?;
        let ending_time = ending_time(schedule, instance)?;

        let width = width(ending_time, starting_time) * SCALE_X;
        let height = schedule.len() as u64 * SCALE_Y;

        let style = Style::new(
            ".hide { fill-opacity: 0; stroke-opacity: 0; }
            .hide:hover { fill-opacity: 1; stroke-opacity: 1; }",
        )
        .set("type", "text/css");

        // NOTE: The extra spacing is to ensure the document does not cut off certain thick elements
        let doc = Document::new()
            .set("width", format!("{}px", width + 4))
            .set("height", format!("{}px", height + 2))
            .add(style);

        let doc = schedule.iter().enumerate().fold(doc, |doc, (row, sched)| {
            let group = match sched {
                Schedule::Arr(sched) => {
                    let arr = instance.flights()[sched.flight_idx].as_arrival().unwrap();
                    self.visualise_arr(sched, arr, row as u64, starting_time)
                },
                Schedule::Dep(sched) => {
                    let dep = instance.flights()[sched.flight_idx].as_departure().unwrap();
                    self.visualise_dep(sched, dep, row as u64, starting_time)
                },
            };
            doc.add(group)
        });

        Some(doc)
    }

    fn visualise_arr(
        &self,
        sched: &ArrivalSchedule,
        arr: &Arrival,
        row: u64,
        starting_time: NaiveTime,
    ) -> Group {
        let landing = {
            let x = width(sched.landing, starting_time) * SCALE_X;
            let y = row * SCALE_Y;

            let title = title!("Arrival at {}", sched.landing.format(HM));

            let bar = line(x, y + 3, 14);
            let square = rect(x - 2, y + 8, 4, 4).set("style", FILL_BLACK);
            Group::new().add(bar).add(square).add(title)
        };

        let base = {
            let x = width(arr.base_time(), starting_time) * SCALE_X;
            let y = row * SCALE_Y;

            let title = title!("Release time at {}", arr.release_time().format(HM),);

            let bar = dashed_line(x, y, SCALE_Y);
            let square = rect(x - 2, y + 8, 4, 4).set("style", HOLLOW_BLACK);

            Group::new().add(bar).add(square).add(title)
        };

        let window = {
            let x = width(arr.window.earliest, starting_time) * SCALE_X;
            let y = (row * SCALE_Y) + 5;

            let width = width(arr.window.latest, arr.window.earliest) * SCALE_X;

            let title = title!("{}-minute arrival window", width / SCALE_X);

            dashed_rect(x, y, width, 10)
                .set("style", HOLLOW_BLACK)
                .set("pointer-events", "none")
                .add(title)
        };

        let delay = {
            let x = width(arr.base_time(), starting_time) * SCALE_X;
            let y = (row * SCALE_Y) + 5;

            let width = width(sched.landing, arr.release_time()) * SCALE_X;

            let title = title!("{}-minute delay", width / SCALE_X);

            rect(x, y, width, 10)
                .set("style", FILL_RED)
                .set("class", "hide")
                .add(title)
        };

        Group::new().add(delay).add(window).add(base).add(landing)
    }

    fn visualise_dep(
        &self,
        sched: &DepartureSchedule,
        dep: &Departure,
        row: u64,
        starting_time: NaiveTime,
    ) -> Group {
        let takeoff = {
            let x = width(sched.takeoff, starting_time) * SCALE_X;
            let y = row * SCALE_Y;

            let title = title!("Departure at {}", sched.takeoff.format(HM));

            let bar = line(x, y + 3, 14);
            let square = rect(x - 2, y + 8, 4, 4).set("style", FILL_BLACK);
            Group::new().add(bar).add(square).add(title)
        };

        let deice = {
            let x = width(sched.deice, starting_time) * SCALE_X;
            let y = row * SCALE_Y;

            let title = title!("De-ice at {}", sched.deice.format(HM));

            let bar = line(x, y + 3, 14);
            let square = rect(x - 2, y + 8, 4, 4).set("style", FILL_BLACK);
            Group::new().add(bar).add(square).add(title)
        };

        let base = {
            let x = width(dep.base_time(), starting_time) * SCALE_X;
            let y = row * SCALE_Y;

            let title = title!("Release time at {}", dep.release_time().format(HM),);

            let bar = dashed_line(x, y, SCALE_Y);
            let square = rect(x - 2, y + 8, 4, 4).set("style", HOLLOW_BLACK);

            Group::new().add(bar).add(square).add(title)
        };

        // TODO: Also display CTOT window
        let window = {
            let x = width(dep.window.earliest, starting_time) * SCALE_X;
            let y = (row * SCALE_Y) + 5;

            let width = width(dep.window.latest, dep.window.earliest) * SCALE_X;

            let title = title!("{}-minute departure window", width / SCALE_X);

            dashed_rect(x, y, width, 10)
                .set("style", HOLLOW_BLACK)
                .set("pointer-events", "none")
                .add(title)
        };

        let delay = {
            let x = width(dep.base_time(), starting_time) * SCALE_X;
            let y = (row * SCALE_Y) + 5;

            let width = width(sched.takeoff, dep.release_time()) * SCALE_X;

            let title = title!("{}-minute delay", width / SCALE_X);

            rect(x, y, width, 10)
                .set("style", FILL_RED)
                .set("class", "hide")
                .add(title)
        };

        let durs = [dep.pushback_dur, dep.taxi_deice_dur];
        let titles = ["pushback from gates", "taxi to de-icing station"];
        let [pushback, taxi_deice] = array::from_fn(|idx| {
            let x = width(
                sched.deice - durs[idx..].iter().sum::<Duration>(),
                starting_time,
            ) * SCALE_X;
            let y = (row * SCALE_Y) + 5;

            let width = minutes(durs[idx]);

            let title = title!("{} minutes to {}", width, titles[idx]);

            let bar = rect(x, y, width * SCALE_X, 10).set("style", FILL_YELLOW);
            let underline = rect(x, y + 10, width * SCALE_X, 2).set("style", FILL_BLACK);

            Group::new()
                .add(bar)
                .add(underline)
                .add(title)
                .set("class", "hide")
        });

        let durs = [dep.deice_dur, dep.taxi_out_dur, dep.lineup_dur];
        let titles = ["de-ice", "taxi to runway", "lineup on runway"];
        let [deice_dur, taxi_out, lineup] = array::from_fn(|idx| {
            let x = width(
                sched.deice + durs[..idx].iter().sum::<Duration>(),
                starting_time,
            ) * SCALE_X;
            let y = (row * SCALE_Y) + 5;

            let width = minutes(durs[idx]);

            let title = title!("{} minutes to {}", width, titles[idx]);

            let bar = rect(x, y, width * SCALE_X, 10).set("style", FILL_YELLOW);
            let underline = rect(x, y + 10, width * SCALE_X, 2).set("style", FILL_BLACK);

            Group::new()
                .add(bar)
                .add(underline)
                .add(title)
                .set("class", "hide")
        });

        let slack = {
            let from = sched.deice + dep.deice_dur + dep.taxi_out_dur + dep.lineup_dur;

            let x = width(from, starting_time) * SCALE_X;
            let y = (row * SCALE_Y) + 5;

            let width = width(sched.takeoff, from) * SCALE_X;

            let title = title!("{}-minute wait", width / SCALE_X);

            let bar = rect(x, y, width, 10).set("style", FILL_BLUE);
            let underline = rect(x, y + 10, width, 2).set("style", FILL_BLACK);

            Group::new()
                .add(bar)
                .add(underline)
                .add(title)
                .set("class", "hide")
        };

        let background = {
            let start = sched.deice - dep.taxi_deice_dur - dep.pushback_dur;

            let x = width(start, starting_time) * SCALE_X;
            let y = (row * SCALE_Y) + 5;
            let width = width(sched.takeoff, start) * SCALE_X;
            rect(x, y, width, 10)
                .set("style", FILL_GREY)
                .set("pointer-events", "none")
        };

        Group::new()
            .add(background)
            .add(pushback)
            .add(taxi_deice)
            .add(deice_dur)
            .add(taxi_out)
            .add(lineup)
            .add(slack)
            .add(delay)
            .add(window)
            .add(base)
            .add(deice)
            .add(takeoff)
    }
}

fn starting_time(schedule: &[Schedule], instance: &Instance) -> Option<NaiveTime> {
    schedule
        .iter()
        .map(|sched| match sched {
            Schedule::Arr(sched) => {
                let arr = instance.flights()[sched.flight_idx].as_arrival().unwrap();
                arr.eto.min(arr.window.earliest).min(sched.landing)
            },
            Schedule::Dep(sched) => {
                let dep = instance.flights()[sched.flight_idx].as_departure().unwrap();
                let mut earliest = dep
                    .tobt
                    .min(dep.window.earliest)
                    .min(sched.deice - dep.taxi_deice_dur - dep.pushback_dur);
                if let Some(ctot) = &dep.ctot {
                    earliest = earliest.min(ctot.earliest());
                }
                earliest
            },
        })
        .min()
}

fn ending_time(schedule: &[Schedule], instance: &Instance) -> Option<NaiveTime> {
    schedule
        .iter()
        .map(|sched| match sched {
            Schedule::Arr(sched) => {
                let arr = instance.flights()[sched.flight_idx].as_arrival().unwrap();
                arr.window.latest.max(sched.landing)
            },
            Schedule::Dep(sched) => {
                let dep = instance.flights()[sched.flight_idx].as_departure().unwrap();
                let mut latest = dep.window.latest.max(sched.takeoff);
                if let Some(ctot) = &dep.ctot {
                    latest = latest.min(ctot.latest());
                }
                latest
            },
        })
        .max()
}

fn width(to: NaiveTime, from: NaiveTime) -> u64 {
    (to - from).num_minutes().unsigned_abs()
}

fn line(x: u64, y: u64, height: u64) -> Line {
    Line::new()
        .set("x1", x)
        .set("x2", x)
        .set("y1", y)
        .set("y2", y + height)
        .set("style", "stroke: #000000;")
}

fn dashed_line(x: u64, y: u64, height: u64) -> Line {
    line(x, y, height).set("stroke-dasharray", "2 1")
}

fn rect(x: u64, y: u64, width: u64, height: u64) -> Rectangle {
    Rectangle::new()
        .set("x", x)
        .set("y", y)
        .set("width", width)
        .set("height", height)
}

fn dashed_rect(x: u64, y: u64, width: u64, height: u64) -> Rectangle {
    rect(x, y, width, height).set("stroke-dasharray", "2 1")
}

fn minutes(dur: Duration) -> u64 {
    dur.as_secs() / 60
}
