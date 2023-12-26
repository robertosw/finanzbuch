use finanzbuch_lib::CurrentDate;
use serde::Deserialize;
use serde::Serialize;

// keep this one imported for better linting support
use crate::DATAFILE_GLOBAL;
#[allow(unused_imports)]
use finanzbuch_lib::datafile;

/// Use like this:
/// ```rs
/// let comparison_bar_html = _fill_comparison_selection_container(...);
/// ```
/// ```html
/// <div class="depotOverview" id="comparisonSelectionContainer">
///     {comparison_bar_html}
/// </div>
/// ```
fn _fill_comparison_selection_container(comparison_groups_html: String) -> String
{
    return format!(
        r#"
        <div class="textContainer">
            <div>Vergleichen mit:</div>
        </div>
        {comparison_groups_html}
        <button id="addComparison" onclick="depotOverviewAddComparison()">+</button>
        <button id="removeComparison" onclick="depotOverviewRemoveComparison()">-</button>
        "#
    );
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ComparisonButtonAction
{
    Add,
    Remove,
}

#[tauri::command]
/// Adds or removes a comparison at the end and returns the html to replace the entire row of comparisons
pub fn depot_overview_get_html_new_comparison(action: ComparisonButtonAction) -> String
{
    let mut datafile = DATAFILE_GLOBAL.lock().expect("DATAFILE_GLOBAL Mutex was poisoned");

    match action {
        ComparisonButtonAction::Add => datafile.investing.comparisons.push(7),
        ComparisonButtonAction::Remove => {
            let _ = datafile.investing.comparisons.pop();
        }
    }

    let mut comparison_groups_html: String = String::new();
    for (i, comp) in datafile.investing.comparisons.iter().enumerate() {
        comparison_groups_html.push_str(
            format!(
                r#"
                <div class="comparisonInputGroup">
                    <input type="number" name="comparison{i}" id="comparison{i}" step="1" min="1" max="99" value="{comp}">
                    <div class="textContainer"><div>%</div></div>
                </div>
                "#
            )
            .as_str(),
        );
    }
    datafile.write();

    return _fill_comparison_selection_container(comparison_groups_html);
}

#[tauri::command]
/// Get the html for the entire "Overview" page
pub fn depot_overview_get_html() -> String
{
    let datafile = DATAFILE_GLOBAL.lock().expect("DATAFILE_GLOBAL Mutex was poisoned");

    let mut comparison_groups_html: String = String::new();
    for (i, comp) in datafile.investing.comparisons.iter().enumerate() {
        comparison_groups_html.push_str(
            format!(
                r#"
                <div class="comparisonInputGroup">
                    <input type="number" name="comparison{i}" id="comparison{i}" step="1" min="1" max="99" value="{comp}">
                    <div class="textContainer"><div>%</div></div>
                </div>
                "#
            )
            .as_str(),
        );
    }

    let comparison_bar_html = _fill_comparison_selection_container(comparison_groups_html);

    return format!(
        r#"
        <div id="depotOverviewContainer">
            <div class="depotOverview" id="comparisonSelectionContainer">
                {comparison_bar_html}
            </div>
            <div id="depotOverviewAllChartsContainer">
                <div class="depotOverviewChartContainer">
                    <canvas class="chartjs" id="fullDepotChartContext"></canvas>
                </div>
                <div class="depotOverviewChartContainer">
                    <canvas class="chartjs" id="fullDepotChartContext"></canvas>
                </div>
                <div class="depotOverviewChartContainer">
                    <canvas class="chartjs" id="fullDepotChartContext"></canvas>
                </div>
            </div>
		</div>
        "#
    );
}

#[tauri::command]
/// What this will look like:
/// `['2023-01', '2023-02', '2023-03', '2023-04', '2023-05', '2023-06']`
///
/// Returnes an empty Vec, if there is no data available
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
/// `[6, 8, 3, 5, 2, 3]`
///
/// Returnes an empty Vec, if there is no data available
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
/// Starting with the oldest value in the depot, this will calculate each monthly value, assuming that the portfolio has grown at the given percentage rate.
///
/// Return value is a Vec that contains one vec for each comparison / growth rate.
/// Each comparison-vec has as many elements as depot_overview_alltime_get_labels returns.
pub fn depot_overview_alltime_get_prognosis() -> Vec<Vec<f64>>
{
    let datafile = DATAFILE_GLOBAL.lock().expect("DATAFILE_GLOBAL Mutex was poisoned");
    let oldest_year: u16 = match datafile.investing.depot.get_oldest_year() {
        Some(y) => y,
        None => return vec![], // All depot entries have no history so there is no data
    };
    let current_year = CurrentDate::current_year();

    // 1. get the oldest value of all entries and sum them up
    let mut start_value: f64 = 0.0;
    for entry in datafile.investing.depot.entries.values() {
        match entry.history.first_key_value() {
            Some((_, val)) => start_value += val.months[0].amount() * val.months[0].price_per_unit(),
            None => {}
        };
    }

    let total_months_in_depot = (current_year + 1 - oldest_year) * 12;
    let mut all_comparisons_values: Vec<Vec<f64>> = Vec::new();

    for comp in datafile.investing.comparisons.iter() {
        let rate = 1.0 + (*comp as f64 / 100.0); // 1.08 for 8%
        let mut values: Vec<f64> = Vec::new();
        let mut prev: f64 = start_value;

        for _ in 0..total_months_in_depot {
            values.push(prev * rate);
            prev = prev * rate;
        }

        all_comparisons_values.push(values);
    }

    return all_comparisons_values;
}
