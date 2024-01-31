use std::time::Duration;

use chrono::NaiveTime;

use svg::{
    node::element::{Group, Line, Rectangle, Style},
    Document,
};

use irdis_core::instance::{
    op::{ArrivalConstraints, DepartureConstraints, OpConstraints},
    schedule::{ArrivalSchedule, DepartureSchedule, Op, OpSchedule},
    Instance,
};

const SCALE: usize = 20;

const FILL_BLACK: &str = "fill: #000000;";

const HOLLOW_BLACK: &str = "stroke: #000000; fill-opacity: 0;";

const FILL_GREY: &str = "fill: #cccccc;";

const FILL_RED: &str = "fill: #c70039;";

const FILL_YELLOW: &str = "fill: #ffd23f;";

const HM: &str = "%H:%M";

macro_rules! title {
    ($($arg:tt)*) => {
        ::svg::node::element::Title::new()
            .add(::svg::node::Text::new(::std::format!($($arg)*)))
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Visualiser {}

impl Visualiser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn visualise(&self, schedule: &[Op], instance: &Instance) -> Option<Document> {
        let starting_time = starting_time(schedule, instance)?;
        let ending_time = schedule.last()?.schedule.op_time();

        let width = width(ending_time, starting_time);
        let height = schedule.len() * SCALE;

        let style = Style::new(
            ".hide { fill-opacity: 0; stroke-opacity: 0; }
            .hide:hover { fill-opacity: 1; stroke-opacity: 1; }",
        )
        .set("type", "text/css");

        let doc = Document::new()
            .set("width", format!("{}px", width + 4))
            .set("height", format!("{}px", height + 2))
            .add(style);

        let doc = schedule.iter().enumerate().fold(doc, |doc, (idx, op)| {
            let constraints = &instance.rows()[op.aircraft_idx].constraints;
            let group = match (&op.schedule, constraints) {
                (OpSchedule::Departure(dep), OpConstraints::Departure(constraints)) => {
                    self.visualise_departure(dep, constraints, idx, starting_time)
                },
                (OpSchedule::Arrival(arr), OpConstraints::Arrival(constraints)) => {
                    self.visualise_arrival(arr, constraints, idx, starting_time)
                },
                _ => unreachable!(),
            };
            doc.add(group)
        });

        Some(doc)
    }

    fn visualise_departure(
        &self,
        dep: &DepartureSchedule,
        constraints: &DepartureConstraints,
        idx: usize,
        starting_time: NaiveTime,
    ) -> Group {
        let take_off = {
            let x = width(dep.take_off_time, starting_time);
            let y = idx * SCALE;

            let title = title!("Departure at {}", dep.take_off_time.format(HM));

            let bar = line(x, y + 3, 14);
            let square = rect(x - 2, y + 8, 4, 4).set("style", FILL_BLACK);
            Group::new().add(bar).add(square).add(title)
        };

        let de_ice = {
            let x = width(dep.de_ice_time, starting_time);
            let y = idx * SCALE;

            let title = title!("De-ice at {}", dep.de_ice_time.format(HM));

            let bar = line(x, y + 3, 14);
            let square = rect(x - 2, y + 8, 4, 4).set("style", FILL_BLACK);
            Group::new().add(bar).add(square).add(title)
        };

        let earliest = {
            let x = width(constraints.earliest_time, starting_time);
            let y = idx * SCALE;

            let title = title!(
                "Earliest possible departure at {}",
                constraints.earliest_time.format(HM),
            );

            let bar = dashed_line(x, y, SCALE);
            let square = rect(x - 2, y + 8, 4, 4).set("style", HOLLOW_BLACK);

            Group::new().add(bar).add(square).add(title)
        };

        let delay = {
            let x = width(constraints.earliest_time, starting_time);
            let y = (idx * SCALE) + 5;

            let width = width(dep.take_off_time, constraints.earliest_time);

            let title = title!("{}-minute delay", width / SCALE);

            rect(x, y, width, 10)
                .set("style", FILL_RED)
                .set("class", "hide")
                .add(title)
        };

        let pushback = {
            let x = width(
                dep.take_off_time
                    - (constraints.lineup_dur
                        + constraints.post_de_ice_dur
                        + constraints.de_ice_dur
                        + constraints.pre_de_ice_dur
                        + constraints.pushback_dur),
                starting_time,
            );
            let y = (idx * SCALE) + 5;

            let width = minutes(constraints.pushback_dur);

            let title = title!("{} minutes to pushback from gates", width);

            let bar = rect(x, y, width * SCALE, 10).set("style", FILL_YELLOW);
            let underline = rect(x, y + 10, width * SCALE, 2).set("style", FILL_BLACK);

            Group::new()
                .add(bar)
                .add(underline)
                .add(title)
                .set("class", "hide")
        };

        let pre_de_ice = {
            let x = width(
                dep.take_off_time
                    - (constraints.lineup_dur
                        + constraints.post_de_ice_dur
                        + constraints.de_ice_dur
                        + constraints.pre_de_ice_dur),
                starting_time,
            );
            let y = (idx * SCALE) + 5;

            let width = minutes(constraints.pre_de_ice_dur);

            let title = title!("{} minutes to taxi to de-icing station", width);

            let bar = rect(x, y, width * SCALE, 10).set("style", FILL_YELLOW);
            let underline = rect(x, y + 10, width * SCALE, 2).set("style", FILL_BLACK);

            Group::new()
                .add(bar)
                .add(underline)
                .add(title)
                .set("class", "hide")
        };

        let de_ice_dur = {
            let x = width(
                dep.take_off_time
                    - (constraints.lineup_dur
                        + constraints.post_de_ice_dur
                        + constraints.de_ice_dur),
                starting_time,
            );
            let y = (idx * SCALE) + 5;

            let width = minutes(constraints.de_ice_dur);

            let title = title!("{} minutes to de-ice", width);

            let bar = rect(x, y, width * SCALE, 10).set("style", FILL_YELLOW);
            let underline = rect(x, y + 10, width * SCALE, 2).set("style", FILL_BLACK);

            Group::new()
                .add(bar)
                .add(underline)
                .add(title)
                .set("class", "hide")
        };

        let post_de_ice = {
            let x = width(
                dep.take_off_time - (constraints.lineup_dur + constraints.post_de_ice_dur),
                starting_time,
            );
            let y = (idx * SCALE) + 5;

            let width = minutes(constraints.post_de_ice_dur);

            let title = title!("{} minutes to taxi to runway", width);

            let bar = rect(x, y, width * SCALE, 10).set("style", FILL_YELLOW);
            let underline = rect(x, y + 10, width * SCALE, 2).set("style", FILL_BLACK);

            Group::new()
                .add(bar)
                .add(underline)
                .add(title)
                .set("class", "hide")
        };

        let lineup = {
            let x = width(dep.take_off_time - constraints.lineup_dur, starting_time);
            let y = (idx * SCALE) + 5;

            let width = minutes(constraints.lineup_dur);

            let title = title!("{} minutes to lineup on runway", width);

            let bar = rect(x, y, width * SCALE, 10).set("style", FILL_YELLOW);
            let underline = rect(x, y + 10, width * SCALE, 2).set("style", FILL_BLACK);

            Group::new()
                .add(bar)
                .add(underline)
                .add(title)
                .set("class", "hide")
        };

        let background = {
            let start = dep.take_off_time
                - (constraints.lineup_dur
                    + constraints.post_de_ice_dur
                    + constraints.de_ice_dur
                    + constraints.pre_de_ice_dur
                    + constraints.pushback_dur);
            let x = width(start, starting_time);
            let y = (idx * SCALE) + 5;
            let width = width(dep.take_off_time, start);
            rect(x, y, width, 10).set("style", FILL_GREY)
        };

        Group::new()
            .add(background)
            .add(pushback)
            .add(pre_de_ice)
            .add(de_ice_dur)
            .add(post_de_ice)
            .add(lineup)
            .add(delay)
            .add(earliest)
            .add(de_ice)
            .add(take_off)
    }

    fn visualise_arrival(
        &self,
        arr: &ArrivalSchedule,
        constraints: &ArrivalConstraints,
        idx: usize,
        starting_time: NaiveTime,
    ) -> Group {
        let landing = {
            let x = width(arr.landing_time, starting_time);
            let y = idx * SCALE;

            let title = title!("Arrival at {}", arr.landing_time.format(HM));

            let bar = line(x, y + 3, 14);
            let square = rect(x - 2, y + 8, 4, 4).set("style", FILL_BLACK);
            Group::new().add(bar).add(square).add(title)
        };

        let earliest = {
            let x = width(constraints.earliest_time, starting_time);
            let y = idx * SCALE;

            let title = title!(
                "Earliest possible arrival at {}",
                constraints.earliest_time.format(HM),
            );

            let bar = dashed_line(x, y, SCALE);
            let square = rect(x - 2, y + 8, 4, 4).set("style", HOLLOW_BLACK);

            Group::new().add(bar).add(square).add(title)
        };

        let delay = {
            let x = width(constraints.earliest_time, starting_time);
            let y = (idx * SCALE) + 5;

            let width = width(arr.landing_time, constraints.earliest_time);

            let title = title!("{}-minute delay", width / SCALE);

            rect(x, y, width, 10)
                .set("style", FILL_RED)
                .set("class", "hide")
                .add(title)
        };

        let background = {
            let x = width(constraints.earliest_time, starting_time);
            let y = (idx * SCALE) + 5;
            let width = width(arr.landing_time, constraints.earliest_time);
            rect(x, y, width, 10).set("style", FILL_GREY)
        };

        Group::new()
            .add(background)
            .add(delay)
            .add(earliest)
            .add(landing)
    }
}

fn starting_time(schedule: &[Op], instance: &Instance) -> Option<NaiveTime> {
    let first_op = schedule.first()?;
    let first_op_constraints = &instance.rows()[first_op.aircraft_idx].constraints;

    let starting_time = match first_op_constraints {
        OpConstraints::Departure(constraints) => {
            first_op.schedule.op_time()
                - (constraints.lineup_dur
                    + constraints.post_de_ice_dur
                    + constraints.de_ice_dur
                    + constraints.pre_de_ice_dur
                    + constraints.pushback_dur)
        },
        OpConstraints::Arrival(constraints) => constraints.earliest_time,
    };

    Some(starting_time)
}

fn width(time: NaiveTime, from: NaiveTime) -> usize {
    (time - from).num_minutes() as usize * SCALE
}

fn line(x: usize, y: usize, height: usize) -> Line {
    Line::new()
        .set("x1", x)
        .set("x2", x)
        .set("y1", y)
        .set("y2", y + height)
        .set("style", "stroke: #000000;")
}

fn dashed_line(x: usize, y: usize, height: usize) -> Line {
    line(x, y, height).set("stroke-dasharray", "2 1")
}

fn rect(x: usize, y: usize, width: usize, height: usize) -> Rectangle {
    Rectangle::new()
        .set("x", x)
        .set("y", y)
        .set("width", width)
        .set("height", height)
}

fn minutes(dur: Duration) -> usize {
    dur.as_secs() as usize / 60
}
