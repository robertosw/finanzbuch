use finanzbuch_lib::fast_date::FastDate;
use finanzbuch_lib::CurrentDate;
use finanzbuch_lib::DataFile;
use serde::Deserialize;
use serde::Serialize;

// keep this one imported for better linting support
use crate::DATAFILE_GLOBAL;
#[allow(unused_imports)]
use finanzbuch_lib::datafile;

// To avoid a multi-lock of the datafile, only allow tauri commands to lock it and all private functions that a command calls expect the datafile to be passed

#[derive(Debug, Serialize, Deserialize)]
pub enum ComparisonAction
{
    Add,
    Remove,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChartJsDataset
{
    pub label: String,
    pub data: Vec<f64>,
}

#[tauri::command]
/// Adds or removes a comparison at the end and returns the html to replace the entire row of comparisons
pub fn depot_overview_do_comparison_action(action: ComparisonAction)
{
    let mut datafile = DATAFILE_GLOBAL.lock().expect("DATAFILE_GLOBAL Mutex was poisoned");
    match action {
        ComparisonAction::Add => datafile.investing.comparisons.push(7),
        ComparisonAction::Remove => {
            let _ = datafile.investing.comparisons.pop();
        }
    }
    datafile.write();
}

#[tauri::command]
/// Get the html for the entire "Overview" page
pub fn depot_overview_get_html() -> String
{
    let datafile = DATAFILE_GLOBAL.lock().expect("DATAFILE_GLOBAL Mutex was poisoned");
    let comparison_bar_html = _build_comparison_bar_html(&datafile);

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

    let Some((oldest_date, _, month_count)) = datafile.investing.depot.get_oldest_year_and_total_month_count() else {
        return vec![]; // All depot entries have no history so there is no data
    };

    // Build a label for each month and year from oldest_year until today
    let mut labels: Vec<String> = vec![String::new(); month_count];

    for (i, el) in labels.iter_mut().enumerate() {
        let year = oldest_date.year() as usize + (i / 12);
        let month = (i % 12) + 1;
        *el = format!("{year}-{month}");
    }

    return labels;
}

#[tauri::command]
/// Constructs an Array of Objects that should be used in the ChartJs `data.datasets` property.
pub fn depot_overview_alltime_get_datasets() -> Vec<ChartJsDataset>
{
    let datafile = DATAFILE_GLOBAL.lock().expect("DATAFILE_GLOBAL Mutex was poisoned");
    let mut datasets: Vec<ChartJsDataset> = Vec::new();

    let mut depot_value_data = Vec::new();
    let mut transactions_data = Vec::new();
    _alltime_graph_data_poll(&datafile, &mut depot_value_data, &mut transactions_data);

    // 1. Depot value over time
    datasets.push(ChartJsDataset {
        label: "Depot value".to_string(),
        data: depot_value_data,
    });
    // 2. All planned and additional transactions
    datasets.push(ChartJsDataset {
        label: "Transactions".to_string(),
        data: transactions_data,
    });

    // 3. Calculated prognosis for each comparison
    for growth_rate in datafile.investing.comparisons.iter() {
        datasets.push(ChartJsDataset {
            label: format!("Prognosis {}%", *growth_rate),
            data: _alltime_graph_get_prognosis(&datafile, *growth_rate),
        });
    }

    return datasets;
}

#[tauri::command]
pub fn depot_overview_change_comparison(comparison_id: String, new_value: String)
{
    let mut datafile = DATAFILE_GLOBAL.lock().expect("DATAFILE_GLOBAL Mutex was poisoned");

    let Ok(new_value) = new_value.parse::<u8>() else {
        return;
    };
    let Ok(comparison_id) = comparison_id.parse::<usize>() else {
        return;
    };

    match datafile.investing.comparisons.get_mut(comparison_id) {
        Some(comp_val) => *comp_val = new_value,
        None => {}
    };
    datafile.write();
}

// ------------------------- Private functions ------------------------- //

fn _alltime_graph_get_prognosis(datafile: &DataFile, growth_rate: u8) -> Vec<f64>
{
    // TODO calc in savings plans
    // value of month 1 = 0 + saving plan transactions + additional transactions
    // value of month 2 = (value of month 1) * growth + saving plan transactions + additional transactions
    // this never uses the actual values of the depot, only the transactions

    // since growth_rate is for one year, the monthly growth_rate has to be calculated:
    // For growth_rate=7 : 1,07^(1÷12) = 1,005654145
    //                                   1,005654145^12 = 1,07
    let rate_yearly = 1.0 + (growth_rate as f64 / 100.0); // 1.08 for 8%
    let rate_monthly = rate_yearly.powf(1.0 / 12.0);

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

    let mut values: Vec<f64> = Vec::new();
    let mut prev: f64 = start_value;

    for _ in 0..total_months_in_depot {
        values.push(prev * rate_monthly);
        prev = prev * rate_monthly;
    }

    return values;
}

/// - First Vec contains data for the total value of the depot in each month
/// - Second Vec contains data for the total transactions in each month
///
/// Does not change the Vec's, if there is no data available in the depot
fn _alltime_graph_data_poll(datafile: &DataFile, history_data_vec: &mut Vec<f64>, transactions_data_vec: &mut Vec<f64>)
{
    let Some((oldest_date, end_date, month_count)) = datafile.investing.depot.get_oldest_year_and_total_month_count() else {
        return; // All depot entries have no history so there is no data
    };

    *history_data_vec = vec![0.0; month_count];
    let history_data = history_data_vec.as_mut_slice(); // size of data is fixed, its only allowed to override values in place

    *transactions_data_vec = vec![0.0; month_count];
    let transactions_data = transactions_data_vec.as_mut_slice(); // size of data is fixed, its only allowed to override values in place

    // Since all entries have the same years, there are no checks needed. Simply add up each month individually
    for entry in datafile.investing.depot.entries.values() {
        'year: for year in entry.history.values() {
            for month in year.months.iter() {
                // only go up until the current date
                let this_date = FastDate::new_risky(year.year_nr, month.month_nr(), 1);
                if this_date > end_date {
                    break 'year;
                }

                let index_year_offset = (year.year_nr - oldest_date.year()) * 12;
                let i: usize = (index_year_offset + month.month_nr() as u16 - 1) as usize; // since months start with 1, subtract 1

                // actual history //
                history_data[i] = history_data[i] + month.amount() * month.price_per_unit();

                // transactions //
                transactions_data[i] = transactions_data[i]
                    + month.additional_transactions()
                    + entry.get_planned_transactions(FastDate::new_risky(year.year_nr, month.month_nr(), 1));
            }
        }
    }
}

/// Use like this:
/// ```rs
/// let comparison_bar_html = _fill_comparison_selection_container(...);
/// ```
/// ```html
/// <div class="depotOverview" id="comparisonSelectionContainer">
///     {comparison_bar_html}
/// </div>
/// ```
fn _build_comparison_bar_html(datafile: &DataFile) -> String
{
    let mut comparison_groups_html: String = String::new();
    for (i, comp) in datafile.investing.comparisons.iter().enumerate() {
        comparison_groups_html.push_str(
            format!(
                r#"
                <div class="comparisonInputGroup">
                    <input type="number" name="comparison{i}" id="comparison{i}" value="{comp}"
                           step="1" min="1" max="99" data-id="{i}"
                           oninput="depotOverviewOnInputComparison()"
                           onblur="depotOverviewInitialize()">
                    <div class="textContainer"><div>%</div></div>
                </div>
                "#
            )
            .as_str(),
        );
    }

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
