use finanzbuch_lib::FastDate;
use finanzbuch_lib::SanitizeInput;
use serde::Deserialize;
use serde::Serialize;

use crate::DATAFILE_GLOBAL;

// TODO possibility to add data to years in the past (older years are above the current one)
// TODO after opening this page this year is always seen first / scrolled to
// TODO if there is no data for this year yet, still create table with empty values so user can input values

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
    let year = match datafile.investing.depot.get_mut(&depot_entry_hash) {
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
/// Currently, All existant years are in this one return
pub fn get_depot_entry_table_html(depot_entry_hash: String) -> String
{
    // JS does not support 64 bit Ints without using BigInt and BigInt cannot be serialized.
    let Ok(depot_entry_hash) = depot_entry_hash.parse() else {
        return format!(r#"<div class="error">This hash {depot_entry_hash} could not be parsed</div>"#);
    };

    let depot_entry: finanzbuch_lib::DepotEntry = {
        let datafile = DATAFILE_GLOBAL.lock().expect("DATAFILE_GLOBAL Mutex was poisoned");
        match datafile.investing.depot.get(&depot_entry_hash) {
            None => return format!(r#"<div class="error">There is no depot entry with this hash: {depot_entry_hash}</div>"#),
            // if this ^ pops up after changing the hashing algorithm, the new one is not deterministic
            Some(de) => de.to_owned(),
        }
    };

    let mut all_years_trs: String = String::new();
    for (year_nr, inv_year) in depot_entry.history.iter() {
        let mut this_year_trs: String = String::new();

        // Prepare to format all values in one column in such a way that all . are below each other
        let mut price_precision: usize = 0;
        let mut amount_precision: usize = 0;

        // Get max precision necessary
        for inv_month in inv_year.months.iter() {
            price_precision = std::cmp::max(_count_precision(inv_month.price_per_unit()), price_precision);
            amount_precision = std::cmp::max(_count_precision(inv_month.amount()), amount_precision);
        }

        for inv_month in inv_year.months.iter() {
            let month_nr = inv_month.month_nr();
            let year_str = match month_nr {
                1 => year_nr.to_string(), // only show year number at the first month
                _ => String::new(),
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
                Err(_) => return format!(r#"<div class="error">While searching for planned transactions, {month_nr} was out of range</div>"#),
            });
            let combined_trs: f64 = planned_trs + inv_month.additional_transactions();

            // These only need precision 2, because they are monetary values
            let additional_trs_fmt = format!("{:.2}", inv_month.additional_transactions());
            let planned_trs_fmt = format!("{:.2}", planned_trs);
            let combined_trs_fmt = format!("{:.2}", combined_trs);

            // - <span> automatically adjusts it size to the content, which is way easier to use than fiddling with <input>'s
            //   but its innerHTML cannot be empty, or tabbing from one to the next will look weird
            //   but that is guaranteed since this function will always write some number
            this_year_trs.push_str(
                format!(
                    r#"
                    <tr>
                        <td>{year_str}</td>
                        <td>{month_nr}</td>
                        <td><span 
                            contenteditable="true" oninput="setDepotEntryTableCell()" id="itp-2023-{month_nr}-{depot_entry_hash}"
                            class="investingTablePrice">{price_fmt}</span> €</td>
                        <td><span 
                            contenteditable="true" oninput="setDepotEntryTableCell()" id="its-2023-{month_nr}-{depot_entry_hash}"
                            class="investingTableSharecount">{amount_fmt}</span></td>
                        <td>{share_volume_fmt} €</td>
                        <td><span 
                            contenteditable="true" oninput="setDepotEntryTableCell()" id="ita-2023-{month_nr}-{depot_entry_hash}"
                            class="investingTableAdditional">{additional_trs_fmt}</span> €</td>
                        <td>{planned_trs_fmt} €</td>
                        <td>{combined_trs_fmt} €</td>
                    </tr>
                    "#,
                )
                .as_str(),
            )
        }
        all_years_trs.push_str(&this_year_trs.as_str());
    }

    // TODO List with button for each year to scroll to that year

    format!(
        r#"
        <div class="depotEntry" id="{depot_entry_hash}">
            <div id="button_col">
                <button id="depotTableRecalcBtn" onclick="getDepotEntryTableHtml()" name="{depot_entry_hash}">Recalculate table</button>
                <button id="depotTableAddBtn" onclick="addDepotTable()" name="{depot_entry_hash}">Add another year</button>
            </div>
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
        "#
    )
}

fn _count_precision(num: f64) -> usize
{
    let num_str = format!("{}", num);
    let num_str_split = num_str.split('.').collect::<Vec<&str>>();
    match num_str_split.get(1) {
        None => return 0,
        Some(len) => return len.chars().filter(|&c| c != '0').count(),
    };
}
