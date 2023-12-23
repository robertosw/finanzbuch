use std::str::FromStr;

use finanzbuch_lib::fast_date::FastDate;
use finanzbuch_lib::investing::inv_variant::InvestmentVariant;
use finanzbuch_lib::investing::inv_year::InvestmentYear;
use finanzbuch_lib::CurrentDate;
use finanzbuch_lib::DepotEntry;
use finanzbuch_lib::SanitizeInput;
use serde::Deserialize;
use serde::Serialize;

// keep this one imported for better linting support
#[allow(unused_imports)]
use finanzbuch_lib::datafile;
use crate::DATAFILE_GLOBAL;

static YEAR_TD_ID_PREFIX: &str = "depotTableScrollTarget";

// TODO if there is no data for this year yet, still show table up until this month with empty values

#[derive(Debug, Serialize, Deserialize)]
pub enum InvestmentMonthFields
{
    Amount,
    PricePerUnit,
    AdditionalTransactions,
}

#[tauri::command]
/// Returns `false` if either
/// - any of the the given fields could not be parsed
/// - no `DepotEntry` with `depot_entry_hash` exists
/// - there is no entry for the given `year` in this `DepotEntry`
///
/// The given value was only saved, if true is returned
pub fn set_depot_entry_table_cell(depot_entry_hash: String, field: InvestmentMonthFields, value: String, year: u16, month: usize) -> bool
{
    // println!( "set_depot_entry_table_cell: {:?} {:?} {:?} {:?} {:?}", depot_entry_hash, field, value, year, month );

    // JS does not support 64 bit Ints without using BigInt and BigInt cannot be serialized.
    let Ok(depot_entry_hash) = depot_entry_hash.parse() else {
        return false;
    };

    let Ok(value_f64) = SanitizeInput::string_to_f64(&value, false) else {
        return false;
    };
    let mut datafile = DATAFILE_GLOBAL.lock().expect("DATAFILE_GLOBAL Mutex was poisoned");
    let year = match datafile.investing.depot.entries.get_mut(&depot_entry_hash) {
        Some(v) => match v.history.get_mut(&(year as u16)) {
            Some(v) => v,
            None => return false,
        },
        None => return false,
    };

    match field {
        InvestmentMonthFields::PricePerUnit => year.months[month - 1].set_price_per_unit(value_f64),
        InvestmentMonthFields::Amount => year.months[month - 1].set_amount(value_f64),
        InvestmentMonthFields::AdditionalTransactions => year.months[month - 1].set_additional_transactions(value_f64),
    }

    datafile.write();
    return true;
}

#[tauri::command]
/// Builds the entire table for one depot entry.
/// Currently, all existant years are in this one return
pub fn get_depot_entry_table_html(depot_entry_hash: String) -> String
{
    // JS does not natively support 64 bit Ints. This would need BigInt, but BigInt cannot be serialized by serde
    let Ok(depot_entry_hash) = depot_entry_hash.parse() else {
        return format!(r#"<div class="error">This hash {depot_entry_hash} could not be parsed</div>"#);
    };

    let mut datafile = DATAFILE_GLOBAL.lock().expect("DATAFILE_GLOBAL Mutex was poisoned");
    let depot_entry = match datafile.investing.depot.entries.get_mut(&depot_entry_hash) {
        None => return format!(r#"<div class="error">There is no depot entry with this hash: {depot_entry_hash}</div>"#),
        // if this ^ pops up after changing the hashing algorithm, the new one is not deterministic
        Some(de) => de,
    };

    // ensure that history has at least the current year, and that the latest year is the current year
    match depot_entry.history.last_key_value() {
        Some((year, _)) => {
            // check that the latest year is the current year, if not create it
            let current_year = CurrentDate::current_year();
            if *year != current_year {
                depot_entry.history.insert(current_year, InvestmentYear::default(current_year));
            }
        }
        None => {
            // add current year, because history is empty
            let current_year = CurrentDate::current_year();
            depot_entry.history.insert(current_year, InvestmentYear::default(current_year));
        }
    };

    let mut history_iterator = depot_entry.history.iter().peekable();

    let mut all_years_trs: String = String::new();
    let mut all_years_buttons: String = String::new();
    let one_before_min_year = history_iterator.peek().unwrap().0 - 1;

    while let Some((year_nr, inv_year)) = history_iterator.next() {
        // Prepare to format all values in one column in such a way that all . are below each other
        let mut price_precision: usize = 2; // since its a monetary value, always enfore .00 precision
        let mut amount_precision: usize = 0;

        // Find out max precision necessary
        for inv_month in inv_year.months.iter() {
            price_precision = std::cmp::max(_count_precision(inv_month.price_per_unit()), price_precision);
            amount_precision = std::cmp::max(_count_precision(inv_month.amount()), amount_precision);
        }

        // Generate html for the months <tr>'s
        let mut trs_of_this_year: String =
            match _build_all_month_rows(year_nr, &price_precision, &amount_precision, &depot_entry, &depot_entry_hash, inv_year) {
                Ok(trs_of_this_year) => trs_of_this_year,
                Err(error_msg_html) => return error_msg_html,
            };

        if history_iterator.peek() != None {
            // This is not the last year in the iterator, so add a spacer to visually seperate the years
            trs_of_this_year.push_str(
                format!(
                    r#"<tr><td></td><td></td><td></td><td></td><td></td><td></td><td></td><td></td></tr>
                    <tr><td></td><td></td><td></td><td></td><td></td><td></td><td></td><td></td></tr>"#,
                )
                .as_str(),
            );
        }

        all_years_trs.push_str(&trs_of_this_year.as_str());

        all_years_buttons.push_str(
            format!(
                r#"
                <button class="depotEntryYearBtn" id="depotEntryYearBtn{year_nr}"
                onclick="depotEntryTableScrollToRow('{YEAR_TD_ID_PREFIX}{year_nr}')">{year_nr}</button>
                "#
            )
            .as_str(),
        );
    }

    format!(
        r#"
        <div class="depotEntry" id="{depot_entry_hash}">
            <div id="depotEntryButtonContainer">
                <button id="depotTableDeleteBtn" ondblclick="depotEntryTableDeleteEntry()" data-hash="{depot_entry_hash}">Delete Entry</button>
                <button id="depotTableRecalcBtn" onclick="depotEntryTableGetHtml()" data-hash="{depot_entry_hash}">Recalculate table</button>
                <button id="depotTableAddBtn" onclick="depotEntryTableAddYear()" data-hash="{depot_entry_hash}">Add {one_before_min_year}</button>
                <div id="depotEntryYearBtnContainer">
                    {all_years_buttons}
                </div>
            </div>
            <div id="depotEntryTableContainer">
                <table>
                    <thead>
                        <tr>
                            <th></th>
                            <th></th>
                            <th></th>
                            <th></th>
                            <th></th>
                            <th colspan=3>Transactions</th>
                        </tr>
                        <tr>
                            <th colspan=2>Month</th>
                            <th>Price per share</th>
                            <th>Amount of shares</th>
                            <th>Shares value</th>
                            <th>Additional</th>
                            <th>Planned</th>
                            <th>Combined</th>
                        </tr>
                    </thead>
                    <tbody>{all_years_trs}</tbody>
                </table>
            </div>
        </div>
        "#
    )
}

#[tauri::command]
pub fn add_depot_entrys_previous_year(depot_entry_hash: String) -> bool
{
    let Ok(depot_entry_hash) = depot_entry_hash.parse::<u64>() else {
        return false;
    };

    let mut datafile = DATAFILE_GLOBAL.lock().expect("DATAFILE_GLOBAL Mutex was poisoned");
    let this_depot_entry = match datafile.investing.depot.entries.get_mut(&depot_entry_hash) {
        Some(de) => de,
        None => return false,
    };

    let oldest_year = match this_depot_entry.history.first_key_value() {
        Some((k, _)) => k,
        None => return false,
    };

    match this_depot_entry.history.insert(oldest_year - 1, InvestmentYear::default(oldest_year - 1)) {
        Some(_) => return false, // this key already had a value
        None => (),
    };

    datafile.write();
    return true;
}

#[tauri::command]
pub fn add_depot_entry(name: String, variant: String) -> bool
{
    if name.is_empty() {
        return false;
    }

    let variant = match InvestmentVariant::from_str(variant.as_str()) {
        Ok(v) => v,
        Err(e) => {
            println!("Error converting String into InvestmentVariant: {e}");
            return false;
        }
    };

    let mut datafile = DATAFILE_GLOBAL.lock().expect("DATAFILE_GLOBAL Mutex was poisoned");
    datafile
        .investing
        .depot
        .add_entry(name.as_str(), DepotEntry::default_with_current_year(name.as_str(), variant));

    datafile.write();
    return true;
}

#[tauri::command]
pub fn delete_depot_entry(depot_entry_hash: String) -> bool
{
    let Ok(depot_entry_hash) = depot_entry_hash.parse::<u64>() else {
        return false;
    };

    let mut datafile = DATAFILE_GLOBAL.lock().expect("DATAFILE_GLOBAL Mutex was poisoned");

    let Some(_) = datafile.investing.depot.entries.remove(&depot_entry_hash) else {
        return false;
    };

    datafile.write();
    return true;
}

// -------------------- private -------------------- //

fn _count_precision(num: f64) -> usize
{
    let num_str = format!("{}", num);
    let num_str_split = num_str.split('.').collect::<Vec<&str>>();
    match num_str_split.get(1) {
        None => return 0,
        Some(len) => return len.chars().filter(|&c| c != '0').count(),
    };
}

/// - If sucessful, will return 12 `<tr>`'s for that year as a HTML String
/// - In the case of an error, a warning as a HTML String will be returned
fn _build_all_month_rows(
    year_nr: &u16,
    price_precision: &usize,
    amount_precision: &usize,
    depot_entry: &DepotEntry,
    depot_entry_hash: &u64,
    inv_year: &InvestmentYear,
) -> Result<String, String>
{
    let mut trs_of_this_year: String = String::new();

    for inv_month in inv_year.months.iter() {
        let month_nr = inv_month.month_nr();
        let (year_str, year_td_id) = match month_nr {
            // only show year number at the first month
            1 => (year_nr.to_string(), format!("id='{YEAR_TD_ID_PREFIX}{year_nr}'")),
            _ => (String::new(), String::new()),
        };

        // Group 1
        let price = inv_month.price_per_unit();
        let amount = inv_month.amount();
        let price_fmt = format!("{:.*}", price_precision, price);
        let amount_fmt = format!("{:.*}", amount_precision, amount);
        let share_volume_fmt = format!("{:.2}", SanitizeInput::f64_to_monetary_f64(price * amount));

        // Group 2
        let planned_trs: f64 = depot_entry.get_planned_transactions(match FastDate::new(year_nr.to_owned(), month_nr, 1) {
            Ok(v) => v,
            Err(_) => {
                return Err(format!(
                    r#"<div class="error">While searching for planned transactions, {month_nr} was out of range</div>"#
                ))
            }
        });
        let combined_trs: f64 = planned_trs + inv_month.additional_transactions();

        // These only need precision 2, because they are monetary values
        let additional_trs_fmt = format!("{:.2}", inv_month.additional_transactions());
        let planned_trs_fmt = format!("{:.2}", planned_trs);
        let combined_trs_fmt = format!("{:.2}", combined_trs);

        // - <span> automatically adjusts it size to the content, which is way easier to use than fiddling with <input>'s
        //   but its innerHTML cannot be empty, or tabbing from one to the next will look weird
        //   but that is guaranteed since this function will always write some number
        // - The id of the year's <td> is later used to have a target to scroll to
        trs_of_this_year.push_str(
            format!(
                r#"
                <tr>
                    <td {year_td_id}>{year_str}</td>
                    <td>{month_nr}</td>
                    <td><span 
                        contenteditable="true" oninput="depotEntryTableSetCell()" id="itp-{year_nr}-{month_nr}-{depot_entry_hash}"
                        class="investingTablePrice">{price_fmt}</span> €</td>
                    <td><span 
                        contenteditable="true" oninput="depotEntryTableSetCell()" id="its-{year_nr}-{month_nr}-{depot_entry_hash}"
                        class="investingTableSharecount">{amount_fmt}</span></td>
                    <td>{share_volume_fmt} €</td>
                    <td><span 
                        contenteditable="true" oninput="depotEntryTableSetCell()" id="ita-{year_nr}-{month_nr}-{depot_entry_hash}"
                        class="investingTableAdditional">{additional_trs_fmt}</span> €</td>
                    <td>{planned_trs_fmt} €</td>
                    <td>{combined_trs_fmt} €</td>
                </tr>
                "#,
            )
            .as_str(),
        );
    }

    return Ok(trs_of_this_year);
}
