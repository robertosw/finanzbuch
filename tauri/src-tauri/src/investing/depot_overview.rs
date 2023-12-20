use std::time::SystemTime;

use chrono::DateTime;
use chrono::Datelike;
use chrono::Utc;

use crate::DATAFILE_GLOBAL;

fn _get_oldest_year_in_depot() -> Option<u16>
{
    let datafile = DATAFILE_GLOBAL.lock().expect("DATAFILE_GLOBAL Mutex was poisoned");

    // go through all DepotEntries and see what the oldest month and year with values are
    let oldest_year: u16 = datafile
        .investing
        .depot
        .entries
        .values()
        .fold(u16::MAX, |accumulator_oldest_year: u16, de: &finanzbuch_lib::DepotEntry| {
            // Go through all depot entries and note the oldest year in the accumulator
            // The start value is just there to have a value in the variable, if there are no depot entries
            match de.history.first_key_value() {
                Some((year, _)) => accumulator_oldest_year.min(*year),
                None => accumulator_oldest_year,
            }
        });

    if oldest_year == u16::MAX {
        return None;
    }

    return Some(oldest_year);
}

#[tauri::command]
/// What this will look like:
/// ['2023-01', '2023-02', '2023-03', '2023-04', '2023-05', '2023-06']
pub fn depot_overview_alltime_get_labels() -> Vec<String>
{
    let datafile = DATAFILE_GLOBAL.lock().expect("DATAFILE_GLOBAL Mutex was poisoned");

    let oldest_year: u16 = match _get_oldest_year_in_depot() {
        Some(y) => y,
        None => todo!("All depot entries have no history so there is no data, but this warning has to be implemented"), // TODO
    };

    // Because every year that is created has all values set to 0, or changed by the user,
    // its fair to assume that data for every month, starting from the oldest, exists

    // Now build a label for each month and year from oldest_year until today
    let now = SystemTime::now();
    let datetime: DateTime<Utc> = now.into();
    let current_year = datetime.year() as u16;

    let mut labels: Vec<String> = Vec::new();
    (oldest_year..current_year + 1).for_each(|year| {
        (1..13_u8).for_each(|month| labels.push(format!("{year}-{month}")));
    });

    return labels;
}

#[tauri::command]
/// The y-datapoints corresponding to the x-labels
/// [6, 8, 3, 5, 2, 3]
pub fn depot_overview_alltime_get_data() -> Vec<f32>
{
    // TODO
    // Add up all values of all depot entries for each month

    let datafile = DATAFILE_GLOBAL.lock().expect("DATAFILE_GLOBAL Mutex was poisoned");

    let oldest_year: u16 = match _get_oldest_year_in_depot() {
        Some(y) => y,
        None => todo!("All depot entries have no history so there is no data, but this warning has to be implemented"), // TODO
    };

    datafile.investing.depot.ensure_uniform_histories();

    // TODO it would be nice, that its guaranteed, that all depot entries have the same years

    let mut values: Vec<f32> = Vec::new();

    for depot_entry in datafile.investing.depot.entries.values() {
        // TODO
    }

    return vec![6.0, 8.0, 3.0, 5.0, 2.0, 3.0];
}

#[tauri::command]
/// Given a growth rate of 7%, this will get the very first depot value and calculate all
/// y-values for the x-labels up until the latest depot value
pub fn depot_overview_alltime_get_prognosis(growth_rate: f32) -> Vec<f32>
{
    // TODO
    if growth_rate == 0.07 {
        return vec![6.0, 6.42, 6.8694, 7.350258, 7.86477606, 8.415310384];
    } else {
        return vec![6.0, 6.3, 6.615, 6.94575, 7.2933, 7.665];
    }
}
