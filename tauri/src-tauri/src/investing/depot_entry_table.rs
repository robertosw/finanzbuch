use finanzbuch_lib::FastDate;
use finanzbuch_lib::SanitizeInput;
use serde::Deserialize;
use serde::Serialize;

use crate::DATAFILE_GLOBAL;

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

        for inv_month in inv_year.months.iter() {
            let month_nr = inv_month.month_nr();
            let price = inv_month.price_per_unit();
            let amount = inv_month.amount();
            let shares_value = SanitizeInput::f64_to_monetary_f64(price * amount);

            let additional_transactions = inv_month.additional_transactions();
            let planned_transactions: f64 = depot_entry.get_planned_transactions(match FastDate::new(year_nr.to_owned(), month_nr, 1) {
                Ok(v) => v,
                Err(_) => return format!(r#"<div class="error">While searching for planned transactions, {month_nr} was out of range</div>"#),
            });
            let combined_transactions: f64 = planned_transactions + additional_transactions;

            let year_str = match month_nr {
                1 => year_nr.to_string(), // only show year number at the first month
                _ => String::new(),
            };

            // assumption: its unlikely that anyone would buy or sell more than 999 999.99 € in one month
            let input_size_additional: u8 = 8;

            // assumption: up to 99 999 per unit + double precicion for cents (.1234 instead of .12)
            let input_size_price: u8 = 9;

            // assumption: up to 999 999.123456
            let input_size_share_count: u8 = 12;

            // 1. The sizes above dont limit what a user can input, its just to shrink the <input>'s to a resonable width
            //    The size="" actually works for monospace fonts
            // 2. The inputs are type=text so that the value parsing can be done in rust
            //    Using type=number wouldnt allow , only .
            this_year_trs.push_str(
                format!(
                    r#"
                    <tr>
                        <td>{year_str}</td>
                        <td>{month_nr}</td>
                        <td><input id="itp-2023-{month_nr}" class="investingTablePrice"      type="text" oninput="setDepotEntryTableCell()" name="{depot_entry_hash}" size="{input_size_price}"       value="{price}">€</input></td>
                        <td><input id="its-2023-{month_nr}" class="investingTableSharecount" type="text" oninput="setDepotEntryTableCell()" name="{depot_entry_hash}" size="{input_size_share_count}" value="{amount}"></input></td>
                        <td>{shares_value}€</td>
                        <td><input id="ita-2023-{month_nr}" class="investingTableAdditional" type="text" oninput="setDepotEntryTableCell()" name="{depot_entry_hash}" size="{input_size_additional}"  value="{additional_transactions}">€</input></td>
                        <td>{planned_transactions}€</td>
                        <td>{combined_transactions}€</td>
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
            <div class="depotEntry" id="button_col">
                <button class="depotEntry" id="depotTableRecalcBtn" onclick="getDepotEntryTableHtml()" name="{depot_entry_hash}">Recalculate table</button>
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
