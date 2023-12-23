use finanzbuch_lib::CurrentDate;

use crate::DATAFILE_GLOBAL;

#[tauri::command]
pub fn depot_overview_get_html() -> String
{
    return format!(
        r#"
        <div id="depotOverviewContainer">
            <div class="depotOverviewChartContainer" id="chartTotalDepotHistory">
                <canvas id="fullDepotChartContext"></canvas>
            </div>
		</div>
        "#
    );
}

#[tauri::command]
/// What this will look like:
/// ['2023-01', '2023-02', '2023-03', '2023-04', '2023-05', '2023-06']
///
/// Returnes and emptry Vec, if there is no data available
pub fn depot_overview_alltime_get_labels() -> Vec<String>
{
    let datafile = DATAFILE_GLOBAL.lock().expect("DATAFILE_GLOBAL Mutex was poisoned");

    let oldest_year: u16 = match datafile.investing.depot.get_oldest_year() {
        Some(y) => y,
        None => return vec![], // All depot entries have no history so there is no data
    };

    // Because every year that is created has all values set to 0, or changed by the user,
    // its fair to assume that data for every month, starting from the oldest, exists

    let current_year = CurrentDate::current_year();

    // Now build a label for each month and year from oldest_year until today
    let mut labels: Vec<String> = Vec::new();
    (oldest_year..current_year + 1).for_each(|year| {
        (1..13_u8).for_each(|month| labels.push(format!("{year}-{month}")));
    });

    return labels;
}

#[tauri::command]
/// The y-datapoints corresponding to the x-labels
/// [6, 8, 3, 5, 2, 3]
///
/// Returnes and emptry Vec, if there is no data available
pub fn depot_overview_alltime_get_data() -> Vec<f64>
{
    let datafile = DATAFILE_GLOBAL.lock().expect("DATAFILE_GLOBAL Mutex was poisoned");

    let oldest_year: u16 = match datafile.investing.depot.get_oldest_year() {
        Some(y) => y,
        None => return vec![], // All depot entries have no history so there is no data
    };
    let current_year = CurrentDate::current_year();

    // fill the vec below with all years and months, starting from oldest_year until now
    let mut values: Vec<f64> = Vec::new();
    for _year in oldest_year..current_year + 1 {
        for _month in 1..13_u8 {
            values.push(0.0);
        }
    }

    assert_eq!(values.len() % 12, 0);

    // Since all entries have the same years, there are no checks needed. Simply add up each month individually
    for de in datafile.investing.depot.entries.values() {
        for year in de.history.values() {
            for month in year.months.iter() {
                let index_year_offset = (year.year_nr - oldest_year) * 12;
                let index: usize = (index_year_offset + month.month_nr() as u16 - 1) as usize; // since months start with 1, subtract 1
                match values.get_mut(index) {
                    Some(v) => *v += month.amount() * month.price_per_unit(),
                    None => panic!(
                        "Tried to access an index, which did not exist. Year: {}  Month: {}  Index: {}  VecLen: {}",
                        year.year_nr,
                        month.month_nr(),
                        index,
                        values.len()
                    ),
                };
            }
        }
    }

    return values;
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
