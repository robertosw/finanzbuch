use dialoguer::*;
use finance_yaml::investing::inv_variant::InvestmentVariant;
use finance_yaml::investing::savings_plan_section::SavingsPlanSection;
use finance_yaml::investing::SavingsPlanInterval;
use finance_yaml::*;
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
