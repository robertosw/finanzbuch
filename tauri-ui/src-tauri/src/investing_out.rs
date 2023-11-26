#[tauri::command]
pub fn get_investing_table_html() -> String
{
    let mut data_rows: String = String::new();

    for i in 1..13 {
        // only show year number at the first month
        let year_str = match i {
            1 => "2023",
            _ => "",
        };

        data_rows.push_str(
            format!(
                r#"
                <tr>
                    <td>{year_str}</td>
                    <td>{i}</td>
                    <td><input id="atp-2023-{i}" class="investing_table_price" type="number" value="0.00"
                            onblur="on_investing_cell_blur()">€</input></td>
                    <td><input id="ats-2023-{i}" class="investing_table_sharecount" type="number" value="111.000"
                            onblur="on_investing_cell_blur()"></input></td>
                    <td>0.00 €</td>
                    <td><input id="ata-2023-{i}" class="investing_table_additional" type="number" value="-222.11"
                            onblur="on_investing_cell_blur()">€</input></td>
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
                <button class="depot_entry" id="save_btn" onclick="get_depot_entry_html()" >Save changes</button>
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
