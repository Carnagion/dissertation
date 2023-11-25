use std::time::Duration;

use svg::{
    node::element::{Group, Line, Rectangle},
    Document,
};

use crate::{departure::Departure, instance::Instance};

pub fn visualise(sequence: &[Departure], instance: &Instance) -> Option<Document> {
    // TODO: Review scaling and impose a minimum width and height
    const SCALE_X: u64 = 9;

    // Earliest time that is represented on the image
    let earliest_dep = sequence.first()?;
    let earliest_constraints = &instance.rows()[earliest_dep.aircraft_idx].constraints;
    let earliest_time = earliest_dep.de_ice_time
        - (earliest_constraints.pre_de_ice_dur + earliest_constraints.pushback_dur);

    // Latest time that is represented on the image
    let latest_dep = sequence.last()?;
    let latest_time = latest_dep.take_off_time + Duration::from_secs(60);

    // Dimensions of the image
    let svg_width = (latest_time - earliest_time).num_minutes() as u64 * SCALE_X;
    let svg_height = sequence.len() * 9;

    let doc = Document::new()
        .set("width", svg_width)
        .set("height", svg_height);

    let doc = sequence.iter().enumerate().fold(doc, |doc, (idx, dep)| {
        // Get the constraints for the current aircraft
        let constraints = &instance.rows()[dep.aircraft_idx].constraints;

        // Calculate the common y and height for all lines
        let line_y = idx * 9;
        let line_height = line_y + 7;

        // Line that marks the beginning of de-icing
        let de_ice_x = (dep.de_ice_time - earliest_time).num_minutes() as u64 * SCALE_X;
        let de_ice_marker = Line::new()
            .set("x1", de_ice_x)
            .set("x2", de_ice_x)
            .set("y1", line_y)
            .set("y2", line_height)
            .set("style", "stroke: rgb(0, 0, 0)");

        // Line that marks take-off time
        let take_off_x = (dep.take_off_time - earliest_time).num_minutes() as u64 * SCALE_X;
        let take_off_marker = Line::new()
            .set("x1", take_off_x)
            .set("x2", take_off_x)
            .set("y1", line_y)
            .set("y2", line_height)
            .set("style", "stroke: rgb(0, 0, 0)");

        // Line that marks the earliest possible take-off time for the aircraft
        let earliest_take_off_x =
            (constraints.earliest_time - earliest_time).num_minutes() as u64 * SCALE_X;
        let earliest_take_off_marker = Line::new()
            .set("x1", earliest_take_off_x)
            .set("x2", earliest_take_off_x)
            .set("y1", line_y)
            .set("y2", line_height)
            .set("style", "stroke: rgb(0, 0, 0)");

        // Calculate the common y and height for all rects
        let rect_y = (idx * 9) + 1;
        let rect_height = 5;

        // Rect that represents time spent de-icing
        let de_ice_width = (constraints.de_ice_dur.as_secs() / 60) * SCALE_X;
        let de_ice_rect = Rectangle::new()
            .set("x", de_ice_x)
            .set("y", rect_y)
            .set("width", de_ice_width)
            .set("height", rect_height)
            .set("style", "fill: #E9C46A");

        // Rect that represents time spent from gates to de-icing station
        let pre_de_ice_width = (constraints.pre_de_ice_dur.as_secs() / 60) * SCALE_X;
        let pre_de_ice_rect = Rectangle::new()
            .set("x", de_ice_x - pre_de_ice_width)
            .set("y", rect_y)
            .set("width", pre_de_ice_width)
            .set("height", rect_height)
            .set("style", "fill: #2A9D8F");

        // Rect that represents time spent in pushback
        let pushback_width = (constraints.pushback_dur.as_secs() / 60) * SCALE_X;
        let pushback_rect = Rectangle::new()
            .set("x", de_ice_x - pre_de_ice_width - pushback_width)
            .set("y", rect_y)
            .set("width", pushback_width)
            .set("height", rect_height)
            .set("style", "fill: #264653");

        // Rect that represents time spent from de-icing station to runway
        let post_de_ice_width = (constraints.post_de_ice_dur.as_secs() / 60) * SCALE_X;
        let post_de_ice_rect = Rectangle::new()
            .set("x", de_ice_x + de_ice_width)
            .set("y", rect_y)
            .set("width", post_de_ice_width)
            .set("height", rect_height)
            .set("style", "fill: #F4A261");

        // Rect that represents time spent lining up before take-off
        let lineup_width = (constraints.lineup_dur.as_secs() / 60) * SCALE_X;
        let lineup_rect = Rectangle::new()
            .set("x", de_ice_x + de_ice_width + post_de_ice_width)
            .set("y", rect_y)
            .set("width", lineup_width)
            .set("height", rect_height)
            .set("style", "fill: #E76F51");

        // Collect all the elements in a group
        let group = Group::new()
            .add(pushback_rect)
            .add(pre_de_ice_rect)
            .add(de_ice_rect)
            .add(post_de_ice_rect)
            .add(lineup_rect)
            .add(earliest_take_off_marker)
            .add(de_ice_marker)
            .add(take_off_marker);

        doc.add(group)
    });

    Some(doc)
}
