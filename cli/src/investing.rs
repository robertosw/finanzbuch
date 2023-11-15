use dialoguer::*;
use finance_yaml::{
    investing::{inv_variant::InvestmentVariant, savings_plan_section::SavingsPlanSection, SavingsPlanInterval},
    *,
};
use std::str::FromStr;

pub fn new_depot_entry() {
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

pub fn add_savings_plan() {
    let mut datafile = DataFile::read();

    println!(
        "\n\
        This dialogue option allows you to create a new savings plan or edit an existing one.\n\n\
        - Both the start and end dates are inclusive.\n\
        - A new savings plan must be created each time the monthly payment is changed. \n\
        Example: From the beginning of January 2023 until the end of June 2023 you deposited €10 per month, \n\
        but from the beginning of July you deposited €20. To do this, you need to create a savings plan with a\n\
        start date of 2023-1 and an end date of 2023-6 (10€), and another with a start date of 2023-7 and any end date (20€).\n"
    );

    // Select which depot entry to edit
    let depot_entry_names: Vec<String> = datafile.investing.depot.iter().map(|(name, _)| name.to_owned()).collect();

    let depot_entry_name: String = match depot_entry_names.len() {
        0 => {
            println!("Your depot is entry. Please create a depot entry first.");
            return;
        }
        1 => depot_entry_names[0].clone(),
        2.. => {
            let selection: usize = Select::new()
                .with_prompt("In which depot entry do you want to add this savings plan?")
                .items(&depot_entry_names)
                .interact()
                .unwrap();
            depot_entry_names[selection].clone()
        }
        _ => unreachable!(),
    };
    loop {
        let start_year: u16 = Input::new().with_prompt("Start year").interact_text().unwrap();
        let start_month: u8 = Input::new().with_prompt("Start month").interact_text().unwrap();
        let end_year: u16 = Input::new().with_prompt("End year").interact_text().unwrap();
        let end_month: u8 = Input::new().with_prompt("End month").interact_text().unwrap();
        let interval: SavingsPlanInterval = match Select::new()
            .with_prompt("Select your interval")
            .items(&vec!["Monthly", "Annually"])
            .default(0)
            .interact()
            .unwrap()
        {
            0 => SavingsPlanInterval::Monthly,
            1 => SavingsPlanInterval::Annually,
            _ => unreachable!(), // has to fit len of vec in items
        };
        let amount: f64 = Input::new().with_prompt("Amount per interval").interact_text().unwrap();

        println!("");

        let Some(depot_element) = datafile.investing.depot.get_mut(&depot_entry_name) else {
        println!("Could not get this depot element '{depot_entry_name}' mutably.");
        return;
    };

        let new_section = SavingsPlanSection {
            start_month,
            start_year,
            end_month,
            end_year,
            amount,
            interval,
        };

        let result = depot_element.add_savings_plan_section(&new_section);

        let Err(err_option) = result else {
            break;  // section was added
        };

        match err_option {
            Some(existent_section) => {
                println!(
                    "You tried to add this section: {:?}.\n\
                    This overlaps with this existent section: {:?}.",
                    new_section, existent_section
                ); // TODO fancy output
                println!("\nEither change the existent section from the main menu or adjust your start or end date");
                continue;
            }
            None => {
                println!(
                    "The given start and end dates do not comply with the rules.\n\
                    Both dates are inclusive. Please check that the start is before the end date."
                );
                continue;
            }
        };
    }

    datafile.write();

    println!(" --- Adding savings plan done ---");
}
