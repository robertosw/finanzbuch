// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String { format!("Hello, {}! You've been greeted from Rust!", name) }

#[tauri::command]
fn get_accounting_table_html() -> String
{
    let mut data_rows: String = String::new();

    for i in 1..13 {
        data_rows.push_str(format!(
            r#"
            <tr>
                <td>2023</td>
                <td>{}</td>
                <td><input id="atp-2023-{}" class="accounting_table_price" type="number" value="0.00" onblur="on_accounting_cell_blur()">€</input></td>
                <td><input id="ats-2023-{}" class="accounting_table_sharecount" type="number" value="111.000" onblur="on_accounting_cell_blur()"></input></td>
                <td>0.00 €</td>
                <td><input id="ata-2023-{}" class="accounting_table_additional" type="number" value="-222.11" onblur="on_accounting_cell_blur()">€</input></td>
                <td>100.00 €</td>
                <td>-122,11 €</td>
            </tr>
            "#, i,i,i,i
        ).as_str())
    }

    format!(
        r#"
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
        "#
    )
}

fn main()
{
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_accounting_table_html])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
