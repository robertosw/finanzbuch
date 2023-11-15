use dialoguer::*;
use finance_yaml::{investing::inv_variant::InvestmentVariant, *};
use std::str::FromStr;

pub fn investing_new_depot_entry() {
    println!("Please specify a name for this depot entry.");
    let name: String = Input::new().allow_empty(false).with_prompt("Name").interact_text().unwrap();

    let variants: Vec<&str> = vec!["Stock", "Fund", "Etf", "Bond", "Option", "Commoditiy", "Crypto"];
    let selection: usize = Select::new().with_prompt("Select a type").items(&variants).interact().unwrap();

    let mut datafile = DataFile::read();
    datafile
        .investing
        .add_depot_element(name, DepotElement::default(InvestmentVariant::from_str(variants[selection]).unwrap()));
    datafile.write();

    println!(" --- Creating new depot entry done ---");
}

// TODO
pub fn cli_investing_modify_savings_plan() {
    println!(
        "\n\
        This dialogue option allows you to create a new savings plan or edit an existing one.\n\n\
        - Both the start and end dates are included.\n\
        - The end date of one savings plan can be left blank.\n\
        - A new savings plan must be created each time the monthly payment is changed. \n\
        Example: From the beginning of January 2023 until the end of June 2023 you deposited €10 per month, \n\
        but from the beginning of July you deposited €20. To do this, you need to create a savings plan with a\n\
        start date of 2023-1 and an end date of 2023-6 (10€), and another with a start date of 2023-7 and any end date (20€).\n"
    );

    if is_depot_empty() {
        println!("Your depot is entry. Please create a depot entry first.");
        investing_new_depot_entry();
    }

    let variants: Vec<&str> = vec!["Create", "Modify"];
    let _selection: usize = Select::new()
        .with_prompt("Do you want to create a new savings plan or modify an existing one?")
        .default(0)
        .items(&variants)
        .interact()
        .unwrap();

    if _selection == 0 {
        let _start_year: u16 = Input::new().with_prompt("Start year").interact_text().unwrap();
        let _start_month: u8 = Input::new().with_prompt("Start month").interact_text().unwrap();
        let _end_year: u16 = Input::new().with_prompt("End year").interact_text().unwrap();
        let _end_month: u8 = Input::new().with_prompt("End month").interact_text().unwrap();

        let variants: Vec<&str> = vec!["Monthly", "Annually"];
        let _selection: usize = Select::new().with_prompt("Select your interval").items(&variants).interact().unwrap();

        let _amount: f64 = Input::new().with_prompt("Amount per interval").interact_text().unwrap();

        // TODO do something with this
        todo!();
    } else {
        todo!();
    }

    println!(" --- Modifying savings plan done ---");
}
