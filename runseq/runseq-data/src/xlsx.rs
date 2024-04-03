use chrono::NaiveDateTime;

use rust_xlsxwriter::{ColNum, Format, RowNum, Worksheet, XlsxError};

use serde::Deserialize;

use runseq_instance::{flight::Flight, Instance};

pub fn to_xlsx(instance: &Instance) -> Result<Worksheet, XlsxError> {
    let mut sheet = Worksheet::new();

    let starting_time = starting_time(instance.flights()).unwrap();

    sheet.write(0, 0, "Number of flights")?;
    sheet.write(1, 0, instance.flights().len() as u64)?;

    sheet.write(3, 0, "Maximum allowed runway hold")?;
    sheet.write(4, 0, instance.max_runway_hold_duration.as_secs())?;

    sheet.deserialize_headers::<RawFlight>(0, 2)?;
    for (idx, flight) in instance.flights().iter().enumerate() {
        let flight = match flight {
            Flight::Arr(arr) => RawFlight {
                kind: FlightKind::Arr,
                base_time: Some(seconds(starting_time, arr.base_time)),
                tobt: None,
                pushback_duration: None,
                deice_taxi_duration: None,
                deice_duration: None,
                deice_hot: None,
                taxi_duration: None,
                lineup_duration: None,
                ctot_target: None,
                ctot_allow_late: None,
                ctot_allow_early: None,
                window_earliest: arr
                    .window
                    .as_ref()
                    .map(|window| seconds(starting_time, window.earliest)),
                window_length: arr.window.as_ref().map(|window| window.duration.as_secs()),
            },
            Flight::Dep(dep) => RawFlight {
                kind: FlightKind::Dep,
                base_time: Some(seconds(starting_time, dep.base_time)),
                tobt: Some(seconds(starting_time, dep.tobt)),
                pushback_duration: Some(dep.pushback_duration.as_secs()),
                deice_taxi_duration: dep
                    .deice
                    .as_ref()
                    .map(|deice| deice.taxi_duration.as_secs()),
                deice_duration: dep.deice.as_ref().map(|deice| deice.duration.as_secs()),
                deice_hot: dep.deice.as_ref().map(|deice| deice.hot.as_secs()),
                taxi_duration: Some(dep.taxi_duration.as_secs()),
                lineup_duration: Some(dep.lineup_duration.as_secs()),
                ctot_target: dep
                    .ctot
                    .as_ref()
                    .map(|ctot| seconds(starting_time, ctot.target)),
                ctot_allow_early: dep.ctot.as_ref().map(|ctot| ctot.allow_early.as_secs()),
                ctot_allow_late: dep.ctot.as_ref().map(|ctot| ctot.allow_late.as_secs()),
                window_earliest: dep
                    .window
                    .as_ref()
                    .map(|window| seconds(starting_time, window.earliest)),
                window_length: dep.window.as_ref().map(|window| window.duration.as_secs()),
            },
        };
        flight.write_to_sheet(idx as u32 + 1, 2, &mut sheet)?;
    }

    // NOTE: Merging is only possible with multiple cells and the Excel library returns an error
    //       when trying to merge a range containing a single cell.
    if instance.flights().len() > 1 {
        sheet.merge_range(
            0,
            17,
            0,
            17 + instance.flights().len() as u16 - 1,
            "Separations",
            &Format::default(),
        )?;
    }

    let pairs = (0..instance.flights().len())
        .flat_map(|i| (0..instance.flights().len()).map(move |j| (i, j)));
    for (row, col) in pairs {
        let sep = instance.separations()[(row, col)].as_secs();
        sheet.write(1 + row as u32, 17 + col as u16, sep)?;
    }

    Ok(sheet)
}

#[derive(Deserialize)]
struct RawFlight {
    #[serde(rename = "Kind")]
    kind: FlightKind,
    #[serde(rename = "Base time")]
    base_time: Option<u64>,
    #[serde(rename = "TOBT")]
    tobt: Option<u64>,
    #[serde(rename = "Pushback duration")]
    pushback_duration: Option<u64>,
    #[serde(rename = "Taxi (before de-icing) duration")]
    deice_taxi_duration: Option<u64>,
    #[serde(rename = "De-icing duration")]
    deice_duration: Option<u64>,
    #[serde(rename = "HOT")]
    deice_hot: Option<u64>,
    #[serde(rename = "Taxi (after de-icing) duration")]
    taxi_duration: Option<u64>,
    #[serde(rename = "Lineup duration")]
    lineup_duration: Option<u64>,
    #[serde(rename = "CTOT")]
    ctot_target: Option<u64>,
    #[serde(rename = "CTOT allowance before")]
    ctot_allow_early: Option<u64>,
    #[serde(rename = "CTOT allowance after")]
    ctot_allow_late: Option<u64>,
    #[serde(rename = "Earliest time")]
    window_earliest: Option<u64>,
    #[serde(rename = "Time window length")]
    window_length: Option<u64>,
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
            .write(row, col + 0, kind)?
            .write(row, col + 1, self.base_time)?
            .write(row, col + 2, self.tobt)?
            .write(row, col + 3, self.pushback_duration)?
            .write(row, col + 4, self.deice_taxi_duration)?
            .write(row, col + 5, self.deice_duration)?
            .write(row, col + 6, self.deice_hot)?
            .write(row, col + 7, self.taxi_duration)?
            .write(row, col + 8, self.lineup_duration)?
            .write(row, col + 9, self.ctot_target)?
            .write(row, col + 10, self.ctot_allow_early)?
            .write(row, col + 11, self.ctot_allow_late)?
            .write(row, col + 12, self.window_earliest)?
            .write(row, col + 13, self.window_length)?;
        Ok(())
    }
}

#[derive(Deserialize)]
enum FlightKind {
    Arr,
    Dep,
}

fn starting_time(flights: &[Flight]) -> Option<NaiveDateTime> {
    flights
        .iter()
        .map(|flight| match flight {
            Flight::Arr(arr) => {
                let mut time = arr.base_time;
                if let Some(window) = &arr.window {
                    time = time.min(window.earliest);
                }
                time
            },
            Flight::Dep(dep) => {
                let mut time = dep.base_time;
                if let Some(window) = &dep.window {
                    time = time.min(window.earliest);
                }
                if let Some(ctot) = &dep.ctot {
                    time = time.min(ctot.earliest());
                }
                time
            },
        })
        .min()
}

fn seconds(from: NaiveDateTime, to: NaiveDateTime) -> u64 {
    (to - from).num_seconds().unsigned_abs()
}
