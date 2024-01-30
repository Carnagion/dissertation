use std::time::Duration;

use chrono::NaiveTime;

use svg::{
    node::{
        element::{Group, Line, Rectangle, Title},
        Text,
    },
    Document,
};

use irdis_core::instance::{
    op::{ArrivalConstraints, DepartureConstraints, OpConstraints},
    schedule::{ArrivalSchedule, DepartureSchedule, Op, OpSchedule},
    Instance,
};

const SCALE_X: usize = 9;

const SCALE_Y: usize = 9;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Visualiser {}

impl Visualiser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn visualise(&self, schedule: &[Op], instance: &Instance) -> Option<Document> {
        let starting_time = starting_time(schedule, instance)?;

        let doc = schedule
            .iter()
            .enumerate()
            .fold(Document::new(), |doc, (idx, op)| {
                let line_y = idx * SCALE_Y;
                let line_height = SCALE_Y - 2;

                let rect_y = (idx * SCALE_Y) + 1;
                let rect_height = SCALE_Y - 4;

                let constraints = &instance.rows()[op.aircraft_idx].constraints;

                let group = match (&op.schedule, constraints) {
                    (OpSchedule::Departure(dep), OpConstraints::Departure(constraints)) => self
                        .visualise_departure(
                            dep,
                            constraints,
                            line_y,
                            line_height,
                            rect_y,
                            rect_height,
                            starting_time,
                        ),
                    (OpSchedule::Arrival(arr), OpConstraints::Arrival(constraints)) => {
                        self.visualise_arrival(arr, constraints, line_y, line_height, starting_time)
                    },
                    _ => unreachable!(),
                };

                doc.add(group)
            });

        let ending_time = schedule.last()?.schedule.op_time();

        let width = (ending_time - starting_time).num_minutes() as usize * SCALE_X;
        let height = schedule.len() * SCALE_Y;

        let doc = doc
            .set("width", format!("{}px", width + 2))
            .set("height", format!("{}px", height + 2));

        Some(doc)
    }

    fn visualise_departure(
        &self,
        dep: &DepartureSchedule,
        constraints: &DepartureConstraints,
        line_y: usize,
        line_height: usize,
        rect_y: usize,
        rect_height: usize,
        starting_time: NaiveTime,
    ) -> Group {
        let take_off_x = (dep.take_off_time - starting_time).num_minutes() as usize * SCALE_X;
        let take_off_title = title(format!("Take-off at {}", dep.take_off_time.format("%H:%M")));
        let take_off = line(take_off_x, line_y, line_height)
            .set("style", "stroke: #000000")
            .add(take_off_title);

        let lineup_width = minutes(constraints.lineup_dur);
        let lineup_title = title(format!("Lineup on runway for {} minutes", lineup_width));
        let lineup_width = lineup_width * SCALE_X;
        let lineup = rect(take_off_x - lineup_width, rect_y, lineup_width, rect_height)
            .set("style", "fill: #e76f51")
            .add(lineup_title);

        let post_de_ice_width = minutes(constraints.post_de_ice_dur);
        let post_de_ice_title = title(format!("Taxi for {} minutes", post_de_ice_width));
        let post_de_ice_width = post_de_ice_width * SCALE_X;
        let post_de_ice = rect(
            take_off_x - lineup_width - post_de_ice_width,
            rect_y,
            post_de_ice_width,
            rect_height,
        )
        .set("style", "fill: #f4a261")
        .add(post_de_ice_title);

        let de_ice_width = minutes(constraints.de_ice_dur);
        let de_ice_title = title(format!("De-ice for {} minutes", de_ice_width));
        let de_ice_width = de_ice_width * SCALE_X;
        let de_ice = rect(
            take_off_x - lineup_width - post_de_ice_width - de_ice_width,
            rect_y,
            de_ice_width,
            rect_height,
        )
        .set("style", "fill: #e9c46a")
        .add(de_ice_title);

        let pre_de_ice_width = minutes(constraints.pre_de_ice_dur);
        let pre_de_ice_title = title(format!("Taxi for {} minutes", pre_de_ice_width));
        let pre_de_ice_width = pre_de_ice_width * SCALE_X;
        let pre_de_ice = rect(
            take_off_x - lineup_width - post_de_ice_width - de_ice_width - pre_de_ice_width,
            rect_y,
            pre_de_ice_width,
            rect_height,
        )
        .set("style", "fill: #2a9d8f")
        .add(pre_de_ice_title);

        let pushback_width = minutes(constraints.pushback_dur);
        let pushback_title = title(format!("Pushback for {} minutes", pushback_width));
        let pushback_width = pushback_width * SCALE_X;
        let pushback = rect(
            take_off_x
                - lineup_width
                - post_de_ice_width
                - de_ice_width
                - pre_de_ice_width
                - pushback_width,
            rect_y,
            pushback_width,
            rect_height,
        )
        .set("style", "fill: #264653")
        .add(pushback_title);

        let earliest_x =
            (constraints.earliest_time - starting_time).num_minutes() as usize * SCALE_X;
        let earliest_title = title(format!(
            "Earliest possible take-off at {}",
            constraints.earliest_time.format("%H:%M"),
        ));
        let earliest = line(earliest_x, line_y, line_height)
            .set("style", "stroke: #000000")
            .set("stroke-dasharray", "1 1")
            .add(earliest_title);

        Group::new()
            .add(pushback)
            .add(pre_de_ice)
            .add(de_ice)
            .add(post_de_ice)
            .add(lineup)
            .add(earliest)
            .add(take_off)
    }

    fn visualise_arrival(
        &self,
        arr: &ArrivalSchedule,
        constraints: &ArrivalConstraints,
        line_y: usize,
        line_height: usize,
        starting_time: NaiveTime,
    ) -> Group {
        let landing_x = (arr.landing_time - starting_time).num_minutes() as usize * SCALE_X;
        let landing_title = title(format!("Land at {}", arr.landing_time.format("%H:%M")));
        let landing = line(landing_x, line_y, line_height)
            .set("style", "stroke: #000000")
            .add(landing_title);

        let earliest_x =
            (constraints.earliest_time - starting_time).num_minutes() as usize * SCALE_X;
        let earliest_title = title(format!(
            "Earliest possible landing at {}",
            constraints.earliest_time.format("%H:%M"),
        ));
        let earliest = line(earliest_x, line_y, line_height)
            .set("style", "stroke: #000000")
            .set("stroke-dasharray", "1 1")
            .add(earliest_title);

        Group::new().add(earliest).add(landing)
    }
}

impl Default for Visualiser {
    fn default() -> Self {
        Self::new()
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

fn line(x: usize, y: usize, height: usize) -> Line {
    Line::new()
        .set("x1", x)
        .set("x2", x)
        .set("y1", y)
        .set("y2", y + height)
}

fn rect(x: usize, y: usize, width: usize, height: usize) -> Rectangle {
    Rectangle::new()
        .set("x", x)
        .set("y", y)
        .set("width", width)
        .set("height", height)
}

fn title<T>(title: T) -> Title
where
    T: Into<String>,
{
    Title::new().add(Text::new(title))
}

fn minutes(dur: Duration) -> usize {
    dur.as_secs() as usize / 60
}
