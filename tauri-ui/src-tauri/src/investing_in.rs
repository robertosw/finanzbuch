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
pub fn send_investing_month_field(field: InvestmentMonthFields, value: String, year: isize, month: isize)
{
    println!("send_investing_month_field: {:?} {:?} {:?} {:?}", field, value, year, month);
}
