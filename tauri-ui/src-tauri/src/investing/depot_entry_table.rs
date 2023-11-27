use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub enum InvestmentMonthFields
{
    Amount,
    PricePerUnit,
    AdditionalTransactions,
}

#[tauri::command]
pub fn set_depot_entry_table_cell(field: InvestmentMonthFields, value: String, year: isize, month: isize)
{
    println!("set_depot_entry_table_cell: {:?} {:?} {:?} {:?}", field, value, year, month);
}

#[tauri::command]
pub fn get_depot_entry_table_html() -> String
{
    let mut data_rows: String = String::new();

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
                    <td><input id="itp-2023-{i}" class="investing_table_price" type="text" value="0.00" oninput="onInvestingCellInput()">€</input></td>
                    <td><input id="its-2023-{i}" class="investing_table_sharecount" type="text" value="111.000" oninput="onInvestingCellInput()"></input></td>
                    <td>0.00 €</td>
                    <td><input id="ita-2023-{i}" class="investing_table_additional" type="text" value="-222.11" oninput="onInvestingCellInput()">€</input></td>
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
        <div class="depot_entry" id="de_container">
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
