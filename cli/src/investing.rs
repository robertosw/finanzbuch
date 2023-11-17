use dialoguer::*;
use finance_yaml::investing::inv_variant::InvestmentVariant;
use finance_yaml::investing::savings_plan_section::SavingsPlanSection;
use finance_yaml::investing::SavingsPlanInterval;
use finance_yaml::*;
use std::io::IsTerminal;
use std::process::exit;
use std::str::FromStr;

pub fn new_depot_entry()
{
    println!("Please specify a name for this depot entry.");
    let name: String = Input::new().allow_empty(false).with_prompt("Name").interact_text().unwrap();

    let variants: Vec<&str> = vec!["Stock", "Fund", "Etf", "Bond", "Option", "Commoditiy", "Crypto"];
    let selection: usize = Select::new().with_prompt("Select a type").default(0).items(&variants).interact().unwrap();

    let mut datafile = DataFile::read();
    datafile
        .investing
        .add_depot_element(name, DepotElement::default(InvestmentVariant::from_str(variants[selection]).unwrap()));
    datafile.write();

    println!(" --- Creating new depot entry done ---");
}

pub fn add_savings_plan()
{
    let mut datafile = DataFile::read();

    // TODO let user select interval first, because end month is not needed for annually.

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
    let Some(depot_entry_name) = _let_user_select_depot_entry(&datafile) else {
        println!("Your depot is empty. Please create a depot entry first.");
        return;
    };

    // print existent saving plans before letting user choose
    println!("");
    println!("Existent saving plans for this depot entry:");
    _print_savings_plan(datafile.investing.depot.get(&depot_entry_name).unwrap());
    println!("");

    // Let user specify data for savings plan section
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
            break; // section was added
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

pub fn output_savings_plan()
{
    let datafile = DataFile::read();

    let Some(depot_entry_name) = _let_user_select_depot_entry(&datafile) else {
        println!("Your depot is empty. Please create a depot entry first.");
        return;
    };

    // print existent saving plans before letting user choose
    println!("");
    _print_savings_plan(datafile.investing.depot.get(&depot_entry_name).unwrap());
}

pub fn depot_overview()
{
    let datafile = DataFile::read();

    // ----- Let user select which depot entries to show -----
    // let all_depot_entries: Vec<(String, DepotElement)> = datafile.investing.depot.iter().map(|(k, v)| (k.to_owned(), v.to_owned())).collect();
    let mut all_depot_entry_names: Vec<&String> = datafile.investing.depot.iter().map(|(k, _)| k).collect();
    let all: String = String::from("All");
    all_depot_entry_names.insert(0, &all);

    let user_selected_depot_entries: Vec<(&String, &DepotElement)> = loop {
        let selection: Vec<usize> = MultiSelect::new()
            .with_prompt("Which depot entries do you want to display? (Spacebar to select, Return to submit)")
            .items(&all_depot_entry_names)
            .interact()
            .unwrap();

        // Check if anything has been selected
        let first_vec_el = match selection.get(0) {
            None => {
                println!("Please select something from this list to continue.");
                continue;
            }
            Some(id) => *id,
        };

        // Check if "All" has been selected (this is always "0"), because the selection Vec is ordered
        if first_vec_el == 0 {
            break datafile.investing.depot.iter().collect();
        } else {
            break datafile
                .investing
                .depot
                .iter()
                .enumerate()
                .filter(|(id, (_k, _v))| selection.contains(&(id - 1 as usize)))
                .map(|(_id, (key, val))| (key, val))
                .collect();
        }
    };

    // ----- Let user select if the savings plans should be shown -----
    let show_savings_plans = Confirm::new()
        .with_prompt("Should each savings plan be shown?")
        .default(false)
        .interact()
        .unwrap();

    // ----- Let user select if a timespan or all data should be shown -----
    let show_only_data_in_timeframe = Confirm::new()
        .with_prompt("Do you want to specify a timeframe to limit the data displayed?")
        .default(false)
        .interact()
        .unwrap();

    // ----- Print all the stuff -----

    for entry in user_selected_depot_entries {
        println!("");
        println!(" ----- {} ----- ", entry.0);
        println!("");
        println!("Variant: {}", entry.1.variant);
        if show_savings_plans {
            println!("");
            _print_savings_plan(entry.1);
            println!("");
        }
        println!("History: {:?}", entry.1.history); // TODO
        _print_history(entry.1);
    }
}

/// Prints:
/// - without any leading and trailing empty lines
/// - a table for each new year in history
/// - the current savings plan for each month in a seperate coloumn
fn _print_history(depot_element: &DepotElement)
{
    for (year_nr, content) in depot_element.history.iter() {
        todo!(); // TODO
    }
}

/// returns `None` if the savings plan is empty
fn _let_user_select_depot_entry(datafile: &DataFile) -> Option<String>
{
    let depot_entry_names: Vec<String> = datafile.investing.depot.iter().map(|(name, _)| name.to_owned()).collect();

    match depot_entry_names.len() {
        0 => return None,
        1 => Some(depot_entry_names[0].clone()),
        2.. => {
            let selection: usize = Select::new()
                .with_prompt("Please choose a depot entry")
                .items(&depot_entry_names)
                .default(0)
                .interact()
                .unwrap();
            Some(depot_entry_names[selection].clone())
        }
        _ => unreachable!(),
    }
}

/// Prints:
/// - without any leading and trailing empty lines
/// - each SavingsPlanSection in a new line
fn _print_savings_plan(depot_element: &DepotElement)
{
    for section in depot_element.savings_plan() {
        // Example:     2023-1 >> 2024-12    20€ Monthly

        println!(
            "{}-{} >> {}-{}    {}€ {}",
            section.start_year, section.start_month, section.end_year, section.end_month, section.amount, section.interval
        );
    }
}
