use finanzbuch_lib::investing::Investing;
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

// TODO how does the JS event know, which depot entry is shown?

#[tauri::command]
/// Returns `false` if either
/// - the given `value` could not be parsed for this field
/// - no `DepotEntry` with `depot_entry_name` exists
/// - there is no entry for the given `year` in this `DepotEntry`
///
/// The given value was only saved, if true is returned
pub fn set_depot_entry_table_cell(depot_entry_hash: u64, field: InvestmentMonthFields, value: String, year: u16, month: usize) -> bool
{
    println!("set_depot_entry_table_cell: {:?} {:?} {:?} {:?}", field, value, year, month);

    let value_f64: f64 = match value.parse() {
        Ok(v) => v,
        Err(_) => return false,
    };

    let mut datafile = DATAFILE_GLOBAL.lock().expect("DATAFILE_GLOBAL Mutex was poisoned");

    let year = match datafile.investing.depot.get_mut(&depot_entry_hash) {
        Some(v) => match v.history.get_mut(&(year as u16)) {
            Some(v) => v,
            None => return false,
        },
        None => return false,
    };

    return match field {
        InvestmentMonthFields::PricePerUnit => {
            year.months[month - 1].price_per_unit = value_f64;
            true
        }
        InvestmentMonthFields::Amount => {
            year.months[month - 1].amount = value_f64;
            true
        }
        InvestmentMonthFields::AdditionalTransactions => {
            year.months[month - 1].additional_transactions = value_f64;
            true
        }
    };
}

#[tauri::command]
pub fn get_depot_entry_table_html(depot_entry_name: String) -> String
{
    let mut data_rows: String = String::new();
    let depot_entry_hash = Investing::name_to_key(depot_entry_name);

    for i in 1..13 {
        // only show year number at the first month
        let year_str = match i {
            1 => "2023",
            _ => "",
        };

        // the inputs are type=text so that rust can search for a value in there, and not JS
        // JS wouldnt allow , only .
        data_rows.push_str(
            format!(
                r#"
                <tr>
                    <td>{year_str}</td>
                    <td>{i}</td>
                    <td><input id="itp-2023-{i}" class="investing_table_price"      type="text" value="0.00"    oninput="onInvestingCellInput()" name="{depot_entry_hash}">€</input></td>
                    <td><input id="its-2023-{i}" class="investing_table_sharecount" type="text" value="111.000" oninput="onInvestingCellInput()" name="{depot_entry_hash}"></input></td>
                    <td>0.00 €</td>
                    <td><input id="ita-2023-{i}" class="investing_table_additional" type="text" value="-222.11" oninput="onInvestingCellInput()" name="{depot_entry_hash}">€</input></td>
                    <td>100.00 €</td>
                    <td>-122,11 €</td>
                </tr>
                "#,
            )
            .as_str(),
        )
    }

    format!(
        r#"
        <div class="depot_entry" id="{depot_entry_hash}">
            <div class="depot_entry" id="button_col">
                <button class="depot_entry" id="save_btn" onclick="getDepotEntryHtml()" >Save changes</button>
            </div>
            <table>
                <thead>
                    <tr>
                        <th></th>
                        <th></th>
                        <th></th>
                        <th></th>
                        <th></th>
                        <th>Transactions</th>
                        <th></th>
                    </tr>
                    <tr>
                        <th></th>
                        <th>Month</th>
                        <th>Price per share</th>
                        <th>Amount of shares</th>
                        <th>Additional</th>
                        <th>Planned</th>
                        <th>Combined</th>
                    </tr>
                </thead>
                <tbody>
                    {data_rows}
                </tbody>
            </table>
        </div>
        "#
    )
}
