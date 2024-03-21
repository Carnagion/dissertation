use std::time::Duration;

use chrono::NaiveTime;

use rust_xlsxwriter::{ColNum, Format, RowNum, Worksheet, XlsxError};

use serde::Deserialize;

use irdis_instance::{flight::Flight, Instance};

pub fn to_xlsx(instance: &Instance) -> Result<Worksheet, XlsxError> {
    let mut sheet = Worksheet::new();

    let starting_time = starting_time(instance.flights()).unwrap();

    sheet.write(0, 0, "Number of flights")?;
    sheet.write(1, 0, instance.flights().len() as u64)?;

    sheet.write(3, 0, "Maximum allowed holdover")?;
    sheet.write(4, 0, as_minutes(instance.max_holdover_dur))?;

    sheet.write(6, 0, "Maximum allowed slack")?;
    sheet.write(7, 0, as_minutes(instance.max_slack_dur))?;

    sheet.deserialize_headers::<RawFlight>(0, 2)?;
    for (idx, flight) in instance.flights().iter().enumerate() {
        let flight = match flight {
            Flight::Arr(arr) => RawFlight {
                kind: FlightKind::Arr,
                eto: Some(minutes(starting_time, arr.eto)),
                window_earliest: minutes(starting_time, arr.window.earliest),
                window_latest: minutes(starting_time, arr.window.latest),
                tobt: None,
                ctot_target: None,
                ctot_allow_after: None,
                ctot_allow_before: None,
                pushback_dur: None,
                taxi_deice_dur: None,
                deice_dur: None,
                taxi_out_dur: None,
                lineup_dur: None,
            },
            Flight::Dep(dep) => RawFlight {
                kind: FlightKind::Dep,
                tobt: Some(minutes(starting_time, dep.tobt)),
                window_earliest: minutes(starting_time, dep.window.earliest),
                window_latest: minutes(starting_time, dep.window.latest),
                ctot_target: dep
                    .ctot
                    .as_ref()
                    .map(|ctot| minutes(starting_time, ctot.target)),
                ctot_allow_before: dep.ctot.as_ref().map(|ctot| as_minutes(ctot.allow_before)),
                ctot_allow_after: dep.ctot.as_ref().map(|ctot| as_minutes(ctot.allow_after)),
                pushback_dur: Some(as_minutes(dep.pushback_dur)),
                taxi_deice_dur: Some(as_minutes(dep.taxi_deice_dur)),
                deice_dur: Some(as_minutes(dep.deice_dur)),
                taxi_out_dur: Some(as_minutes(dep.taxi_out_dur)),
                lineup_dur: Some(as_minutes(dep.lineup_dur)),
                eto: None,
            },
        };
        flight.write_to_sheet(idx as u32 + 1, 2, &mut sheet)?;
    }

    // NOTE: Merging is only possible with multiple cells and the Excel library returns an error
    //       when trying to merge a range containing a single cell.
    if instance.flights().len() > 1 {
        sheet.merge_range(
            0,
            16,
            0,
            16 + instance.flights().len() as u16 - 1,
            "Separations",
            &Format::default(),
        )?;
    }

    let pairs = (0..instance.flights().len())
        .flat_map(|i| (0..instance.flights().len()).map(move |j| (i, j)));
    for (row, col) in pairs {
        let sep = as_minutes(instance.separations()[(row, col)]);
        sheet.write(1 + row as u32, 16 + col as u16, sep)?;
    }

    Ok(sheet)
}

#[derive(Deserialize)]
struct RawFlight {
    #[serde(rename = "Kind")]
    kind: FlightKind,
    #[serde(rename = "ETO")]
    eto: Option<u64>,
    #[serde(rename = "TOBT")]
    tobt: Option<u64>,
    #[serde(rename = "CTOT")]
    ctot_target: Option<u64>,
    #[serde(rename = "CTOT allowance before")]
    ctot_allow_before: Option<u64>,
    #[serde(rename = "CTOT allowance after")]
    ctot_allow_after: Option<u64>,
    #[serde(rename = "Pushback duration")]
    pushback_dur: Option<u64>,
    #[serde(rename = "Taxi (before de-icing) duration")]
    taxi_deice_dur: Option<u64>,
    #[serde(rename = "De-icing duration")]
    deice_dur: Option<u64>,
    #[serde(rename = "Taxi (after de-icing) duration")]
    taxi_out_dur: Option<u64>,
    #[serde(rename = "Lineup duration")]
    lineup_dur: Option<u64>,
    #[serde(rename = "Earliest time")]
    window_earliest: u64,
    #[serde(rename = "Latest time")]
    window_latest: u64,
}

impl RawFlight {
    fn write_to_sheet(
        self,
        row: RowNum,
        col: ColNum,
        sheet: &mut Worksheet,
    ) -> Result<(), XlsxError> {
        let kind = match self.kind {
            FlightKind::Arr => "arrival",
            FlightKind::Dep => "departure",
        };
        sheet
            .write(row, col, kind)?
            .write(row, col + 1, self.eto)?
            .write(row, col + 2, self.tobt)?
            .write(row, col + 3, self.ctot_target)?
            .write(row, col + 4, self.ctot_allow_before)?
            .write(row, col + 5, self.ctot_allow_after)?
            .write(row, col + 6, self.pushback_dur)?
            .write(row, col + 7, self.taxi_deice_dur)?
            .write(row, col + 8, self.deice_dur)?
            .write(row, col + 9, self.taxi_out_dur)?
            .write(row, col + 10, self.lineup_dur)?
            .write(row, col + 11, self.window_earliest)?
            .write(row, col + 12, self.window_latest)?;
        Ok(())
    }
}

#[derive(Deserialize)]
enum FlightKind {
    Arr,
    Dep,
}

fn starting_time(flights: &[Flight]) -> Option<NaiveTime> {
    flights
        .iter()
        .map(|flight| match flight {
            Flight::Arr(arr) => arr.eto.min(arr.window.earliest),
            Flight::Dep(dep) => {
                let mut earliest = dep.tobt.min(dep.window.earliest);
                if let Some(ctot) = &dep.ctot {
                    earliest = earliest.min(ctot.earliest());
                }
                earliest
            },
        })
        .min()
}

fn minutes(from: NaiveTime, to: NaiveTime) -> u64 {
    (to - from).num_minutes().unsigned_abs()
}

fn as_minutes(dur: Duration) -> u64 {
    dur.as_secs() / 60
}
