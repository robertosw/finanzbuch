use crate::DATAFILE_GLOBAL;

#[tauri::command]
/// What this will look like:
/// ['2023-01', '2023-02', '2023-03', '2023-04', '2023-05', '2023-06']
pub fn depot_overview_alltime_get_labels() -> Vec<&'static str>
{
    return vec![
        "2023-01", "2023-02", "2023-03", "2023-04", "2023-05", "2023-06", "2023-07", "2023-08", "2023-09", "2023-10", "2023-11", "2023-12",
    ];
}

#[tauri::command]
/// The y-datapoints corresponding to the x-labels
/// [6, 8, 3, 5, 2, 3]
pub fn depot_overview_alltime_get_data() -> Vec<u32> { return vec![6, 8, 3, 5, 2, 3]; }

#[tauri::command]
/// Given a growth rate of 7%, this will get the very first depot value and calculate all
/// y-values for the x-labels up until the latest depot value
pub fn depot_overview_alltime_get_prognosis(growth_rate: f32) -> Vec<f32>
{
    if growth_rate == 0.07 {
        return vec![6.0, 6.42, 6.8694, 7.350258, 7.86477606, 8.415310384];
    } else {
        return vec![6.0, 6.3, 6.615, 6.94575, 7.2933, 7.665];
    }
}
