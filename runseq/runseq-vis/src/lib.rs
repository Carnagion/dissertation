//! Helpers to visualise sequences of scheduled aircraft landings, take-offs, and de-icing times.

#![deny(rust_2018_idioms)]
#![warn(missing_docs)]

use chrono::NaiveDateTime;

use svg::{
    node::element::{Group, Line, Rectangle, Style},
    Document,
};

use runseq_instance::{
    flight::{Arrival, Departure},
    schedule::{ArrivalSchedule, DepartureSchedule, Schedule},
    Instance,
};

const SCALE_X: f64 = 0.4;

const SCALE_Y: f64 = 20.0;

const SHORT_BAR_HEIGHT: f64 = SCALE_Y * 0.7;

const SHORT_BAR_OFFSET: f64 = (SCALE_Y - SHORT_BAR_HEIGHT) * 0.5;

const SQUARE_SIZE: f64 = SCALE_Y * 0.2;

const SQUARE_OFFSET: f64 = (SCALE_Y - SQUARE_SIZE) * 0.5;

const RECT_HEIGHT: f64 = SCALE_Y * 0.5;

const RECT_OFFSET: f64 = (SCALE_Y - RECT_HEIGHT) * 0.5;

const UNDERLINE_HEIGHT: f64 = SCALE_Y * 0.1;

const FILL_BLACK: &str = "fill: #000000;";

const HOLLOW_BLACK: &str = "stroke: #000000; fill-opacity: 0;";

const FILL_GREY: &str = "fill: #cccccc;";

const FILL_RED: &str = "fill: #c70039;";

const FILL_BLUE: &str = "fill: #1363df";

const FILL_YELLOW: &str = "fill: #ffd23f;";

const FMT: &str = "%F %T";

macro_rules! title {
    ($($arg:tt)*) => {
        ::svg::node::element::Title::new()
            .add(::svg::node::Text::new(::std::format!($($arg)*)))
    }
}

/// A runway sequence visualiser.
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub struct Visualiser {
    _priv: (),
}

impl Visualiser {
    /// Creates a new visualiser with the default settings.
    pub fn new() -> Self {
        Self { _priv: () }
    }

    /// Visualises a runway sequence produced after solving an [`Instance`], returning [`None`] if the sequence is empty.
    ///
    /// # Panics
    ///
    /// This method will panic if the number of aircraft in the sequence does not match the number of aircraft in the instance,
    /// which can happen if the given runway sequence was not produced by solving the given instance.
    pub fn visualise(&self, schedule: &[Schedule], instance: &Instance) -> Option<Document> {
        let start = start_time(schedule, instance)?;
        let end = end_time(schedule, instance)?;

        let width = width(end, start) * SCALE_X;
        let height = schedule.len() as f64 * SCALE_Y;

        let style = Style::new(
            ".hide { fill-opacity: 0; stroke-opacity: 0; }
            .hide:hover { fill-opacity: 1; stroke-opacity: 1; }",
        )
        .set("type", "text/css");

        // NOTE: The extra spacing ensures the document does not cut off certain thick elements.
        let doc = Document::new()
            .set("width", format!("{}px", width + 4.0))
            .set("height", format!("{}px", height + 2.0))
            .add(style);

        let doc = schedule.iter().enumerate().fold(doc, |doc, (row, sched)| {
            let group = match sched {
                Schedule::Arr(sched) => {
                    let arr = instance.flights()[sched.flight_index].as_arrival().unwrap();
                    self.visualise_arrival(sched, arr, row, start)
                },
                Schedule::Dep(sched) => {
                    let dep = instance.flights()[sched.flight_index]
                        .as_departure()
                        .unwrap();
                    self.visualise_departure(sched, dep, row, start)
                },
            };
            doc.add(group)
        });

        Some(doc)
    }

    fn visualise_arrival(
        &self,
        sched: &ArrivalSchedule,
        arr: &Arrival,
        row: usize,
        start: NaiveDateTime,
    ) -> Group {
        let row = row as f64;

        let landing = {
            let x = width(sched.landing, start) * SCALE_X;
            let y = row * SCALE_Y;

            let title = title!("Landing ({})", sched.landing.format(FMT));

            let bar = line(x, y + SHORT_BAR_OFFSET, SHORT_BAR_HEIGHT);
            let square = square(x - (SQUARE_SIZE * 0.5), y + SQUARE_OFFSET, SQUARE_SIZE)
                .set("style", FILL_BLACK);

            Group::new().add(bar).add(square).add(title)
        };

        let base = {
            let x = width(arr.base_time, start) * SCALE_X;
            let y = row * SCALE_Y;

            let title = title!("Base time ({})", arr.base_time.format(FMT));

            let bar = dashed_line(x, y, SCALE_Y);
            let square = square(x - (SQUARE_SIZE * 0.5), y + SQUARE_OFFSET, SQUARE_SIZE)
                .set("style", HOLLOW_BLACK);

            Group::new().add(bar).add(square).add(title)
        };

        let delay = {
            let x = width(arr.base_time, start) * SCALE_X;
            let y = (row * SCALE_Y) + RECT_OFFSET;

            let width = width(sched.landing, arr.base_time) * SCALE_X;

            let title = title!("Delay ({} seconds)", width / SCALE_X);

            rect(x, y, width, RECT_HEIGHT)
                .set("style", FILL_RED)
                .set("class", "hide")
                .add(title)
        };

        Group::new().add(delay).add(base).add(landing)
    }

    fn visualise_departure(
        &self,
        sched: &DepartureSchedule,
        dep: &Departure,
        row: usize,
        start: NaiveDateTime,
    ) -> Group {
        let row = row as f64;

        let takeoff = {
            let x = width(sched.takeoff, start) * SCALE_X;
            let y = row * SCALE_Y;

            let title = title!("Take-off ({})", sched.takeoff.format(FMT));

            let bar = line(x, y + SHORT_BAR_OFFSET, SHORT_BAR_HEIGHT);
            let square = square(x - (SQUARE_SIZE * 0.5), y + SQUARE_OFFSET, SQUARE_SIZE)
                .set("style", FILL_BLACK);

            Group::new().add(bar).add(square).add(title)
        };

        let deice = match sched.deice {
            None => Group::new(),
            Some(deice) => {
                let x = width(deice, start) * SCALE_X;
                let y = row * SCALE_Y;

                let title = title!("De-ice ({})", deice.format(FMT));

                let bar = line(x, y + SHORT_BAR_OFFSET, SHORT_BAR_HEIGHT);
                let square = square(x - (SQUARE_SIZE * 0.5), y + SQUARE_OFFSET, SQUARE_SIZE)
                    .set("style", FILL_BLACK);

                Group::new().add(bar).add(square).add(title)
            },
        };

        let base = {
            let x = width(dep.base_time, start) * SCALE_X;
            let y = row * SCALE_Y;

            let title = title!("Base time ({})", dep.base_time.format(FMT));

            let bar = dashed_line(x, y, SCALE_Y);
            let square = square(x - (SQUARE_SIZE * 0.5), y + SQUARE_OFFSET, SQUARE_SIZE)
                .set("style", HOLLOW_BLACK);

            Group::new().add(bar).add(square).add(title)
        };

        let delay = {
            let x = width(dep.base_time, start) * SCALE_X;
            let y = (row * SCALE_Y) + RECT_OFFSET;

            let width = width(sched.takeoff, dep.base_time) * SCALE_X;

            let title = title!("Delay ({} seconds)", width / SCALE_X);

            rect(x, y, width, RECT_HEIGHT)
                .set("style", FILL_RED)
                .set("class", "hide")
                .add(title)
        };

        let runway_hold = match sched.deice {
            None => Group::new(),
            Some(deice) => {
                let taxi_end = deice + dep.deice.as_ref().unwrap().duration + dep.taxi_duration;

                let x = width(taxi_end, start) * SCALE_X;
                let y = (row * SCALE_Y) + RECT_OFFSET;

                let width = width(sched.takeoff - dep.lineup_duration, taxi_end) * SCALE_X;

                let title = title!("Runway hold ({} seconds)", width / SCALE_X);

                let bar = rect(x, y, width, RECT_HEIGHT).set("style", FILL_BLUE);
                let underline =
                    rect(x, y + RECT_HEIGHT, width, UNDERLINE_HEIGHT).set("style", FILL_BLACK);

                Group::new()
                    .add(bar)
                    .add(underline)
                    .add(title)
                    .set("class", "hide")
            },
        };

        let pushback = {
            let x = match sched.deice {
                None => width(
                    sched.takeoff - dep.lineup_duration - dep.taxi_duration - dep.pushback_duration,
                    start,
                ),
                Some(deice) => width(
                    deice - dep.deice.as_ref().unwrap().taxi_duration - dep.pushback_duration,
                    start,
                ),
            } * SCALE_X;
            let y = (row * SCALE_Y) + RECT_OFFSET;

            let width = dep.pushback_duration.as_secs_f64() * SCALE_X;

            let title = title!("Pushback ({} seconds)", width / SCALE_X);

            let bar = rect(x, y, width, RECT_HEIGHT).set("style", FILL_YELLOW);
            let underline =
                rect(x, y + RECT_HEIGHT, width, UNDERLINE_HEIGHT).set("style", FILL_BLACK);

            Group::new()
                .add(bar)
                .add(underline)
                .add(title)
                .set("class", "hide")
        };

        let taxi_deice = match sched.deice {
            None => Group::new(),
            Some(deice) => {
                let taxi_deice_dur = dep.deice.as_ref().unwrap().taxi_duration;

                let x = width(deice - taxi_deice_dur, start) * SCALE_X;
                let y = (row * SCALE_Y) + RECT_OFFSET;

                let width = taxi_deice_dur.as_secs_f64() * SCALE_X;

                let title = title!("Taxi ({} seconds)", width / SCALE_X);

                let bar = rect(x, y, width, RECT_HEIGHT).set("style", FILL_YELLOW);
                let underline =
                    rect(x, y + RECT_HEIGHT, width, UNDERLINE_HEIGHT).set("style", FILL_BLACK);

                Group::new()
                    .add(bar)
                    .add(underline)
                    .add(title)
                    .set("class", "hide")
            },
        };

        let apply_deice = match sched.deice {
            None => Group::new(),
            Some(deice) => {
                let x = width(deice, start) * SCALE_X;
                let y = (row * SCALE_Y) + RECT_OFFSET;

                let width = dep.deice.as_ref().unwrap().duration.as_secs_f64() * SCALE_X;

                let title = title!("De-ice ({} seconds)", width / SCALE_X);

                let bar = rect(x, y, width, RECT_HEIGHT).set("style", FILL_YELLOW);
                let underline =
                    rect(x, y + RECT_HEIGHT, width, UNDERLINE_HEIGHT).set("style", FILL_BLACK);

                Group::new()
                    .add(bar)
                    .add(underline)
                    .add(title)
                    .set("class", "hide")
            },
        };

        let taxi_out = {
            let x = match sched.deice {
                None => width(
                    sched.takeoff - dep.lineup_duration - dep.taxi_duration,
                    start,
                ),
                Some(deice) => width(deice + dep.deice.as_ref().unwrap().duration, start),
            } * SCALE_X;
            let y = (row * SCALE_Y) + RECT_OFFSET;

            let width = dep.taxi_duration.as_secs_f64() * SCALE_X;

            let title = title!("Taxi out ({} seconds)", width / SCALE_X);

            let bar = rect(x, y, width, RECT_HEIGHT).set("style", FILL_YELLOW);
            let underline =
                rect(x, y + RECT_HEIGHT, width, UNDERLINE_HEIGHT).set("style", FILL_BLACK);

            Group::new()
                .add(bar)
                .add(underline)
                .add(title)
                .set("class", "hide")
        };

        let lineup = {
            let x = width(sched.takeoff - dep.lineup_duration, start) * SCALE_X;
            let y = (row * SCALE_Y) + RECT_OFFSET;

            let width = dep.lineup_duration.as_secs_f64() * SCALE_X;

            let title = title!("Lineup ({} seconds)", width / SCALE_X);

            let bar = rect(x, y, width, RECT_HEIGHT).set("style", FILL_YELLOW);
            let underline =
                rect(x, y + RECT_HEIGHT, width, UNDERLINE_HEIGHT).set("style", FILL_BLACK);

            Group::new()
                .add(bar)
                .add(underline)
                .add(title)
                .set("class", "hide")
        };

        let background = {
            let pushback = match sched.deice {
                None => {
                    sched.takeoff - dep.lineup_duration - dep.taxi_duration - dep.pushback_duration
                },
                Some(deice) => {
                    deice - dep.deice.as_ref().unwrap().taxi_duration - dep.pushback_duration
                },
            };

            let x = width(pushback, start) * SCALE_X;
            let y = (row * SCALE_Y) + RECT_OFFSET;

            let width = width(sched.takeoff, pushback) * SCALE_X;
            rect(x, y, width, RECT_HEIGHT)
                .set("style", FILL_GREY)
                .set("pointer-events", "none")
        };

        Group::new()
            .add(background)
            .add(pushback)
            .add(taxi_deice)
            .add(apply_deice)
            .add(taxi_out)
            .add(lineup)
            .add(runway_hold)
            .add(delay)
            .add(base)
            .add(deice)
            .add(takeoff)
    }
}

fn start_time(schedule: &[Schedule], instance: &Instance) -> Option<NaiveDateTime> {
    schedule
        .iter()
        .map(|sched| match sched {
            Schedule::Arr(sched) => {
                let arr = instance.flights()[sched.flight_index].as_arrival().unwrap();

                let mut time = arr.base_time;
                if let Some(window) = &arr.window {
                    time = time.min(window.earliest);
                }
                time
            },
            Schedule::Dep(sched) => {
                let dep = instance.flights()[sched.flight_index]
                    .as_departure()
                    .unwrap();

                let mut time = dep.base_time;
                if let Some(window) = &dep.window {
                    time = time.min(window.earliest);
                }
                if let Some(ctot) = &dep.ctot {
                    time = time.min(ctot.earliest());
                }
                time = match sched.deice {
                    None => time.min(
                        sched.takeoff
                            - dep.lineup_duration
                            - dep.taxi_duration
                            - dep.pushback_duration,
                    ),
                    Some(deice) => {
                        let taxi_deice = dep.deice.as_ref().unwrap().taxi_duration;
                        time.min(deice - taxi_deice - dep.pushback_duration)
                    },
                };
                time
            },
        })
        .min()
}

fn end_time(schedule: &[Schedule], instance: &Instance) -> Option<NaiveDateTime> {
    schedule
        .iter()
        .map(|sched| match sched {
            Schedule::Arr(sched) => {
                let arr = instance.flights()[sched.flight_index].as_arrival().unwrap();

                let mut time = sched.landing;
                if let Some(window) = &arr.window {
                    time = time.max(window.latest());
                }
                time
            },
            Schedule::Dep(sched) => {
                let dep = instance.flights()[sched.flight_index]
                    .as_departure()
                    .unwrap();

                let mut time = sched.takeoff;
                if let Some(window) = &dep.window {
                    time = time.max(window.latest());
                }
                if let Some(ctot) = &dep.ctot {
                    time = time.max(ctot.latest());
                }
                time
            },
        })
        .max()
}

fn width(to: NaiveDateTime, from: NaiveDateTime) -> f64 {
    (to - from).num_seconds().unsigned_abs() as f64
}

fn line(x: f64, y: f64, height: f64) -> Line {
    Line::new()
        .set("x1", x)
        .set("x2", x)
        .set("y1", y)
        .set("y2", y + height)
        .set("style", "stroke: #000000;")
}

fn dashed_line(x: f64, y: f64, height: f64) -> Line {
    line(x, y, height).set("stroke-dasharray", "2 1")
}

fn rect(x: f64, y: f64, width: f64, height: f64) -> Rectangle {
    Rectangle::new()
        .set("x", x)
        .set("y", y)
        .set("width", width)
        .set("height", height)
}

// fn dashed_rect(x: f64, y: f64, width: f64, height: f64) -> Rectangle {
//     rect(x, y, width, height).set("stroke-dasharray", "2 1")
// }

fn square(x: f64, y: f64, size: f64) -> Rectangle {
    rect(x, y, size, size)
}
