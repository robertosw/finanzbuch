use dialoguer::*;
use finance_yaml::investing::inv_variant::InvestmentVariant;
use finance_yaml::investing::inv_year::InvestmentYear;
use finance_yaml::investing::savings_plan_section::SavingsPlanSection;
use finance_yaml::investing::SavingsPlanInterval;
use finance_yaml::*;
use std::str::FromStr;

struct YearAndMonth
{
    year: u16,
    month: u8,
}

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

pub fn individual_depot_entry_output()
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
                .filter(|(id, (_k, _v))| selection.contains(&(id)))
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

    let mut start: Option<YearAndMonth> = None;
    let mut end: Option<YearAndMonth> = None;

    if show_only_data_in_timeframe {
        start = Some(YearAndMonth {
            year: Input::new().with_prompt("Start year").interact_text().unwrap(),
            month: Input::new().with_prompt("Start month").interact_text().unwrap(),
        });
        end = Some(YearAndMonth {
            year: Input::new().with_prompt("End year").interact_text().unwrap(),
            month: Input::new().with_prompt("End month").interact_text().unwrap(),
        });
    }

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

        _print_history(entry.1, &start, &end);
    }
}

/// Prints:
/// - without any leading and trailing empty lines
/// - a table containing every data point from `start` to `end`
/// - the current savings plan for each month added as a seperate coloumn
fn _print_history(depot_element: &DepotElement, start: &Option<YearAndMonth>, end: &Option<YearAndMonth>)
{
    //           |            |              |          Transactions        
    //    Month  |   amount   |  Unit Price  | Planned | Additional | Total 
    //   ------- | ---------- | ------------ | ------- | ---------- | ----- 
    //   2023 01 |       0.00 |         0.00 |    0.00 |       0.00 |  0.00 

    // ^ means oriented to the middle of the space available
    // > means oriented to the right
    println!(" {:33} | {:^36}", "", "Transactions");
    println!(
        " {:^7} | {:^10} | {:^10} | {:^10} | {:^10} | {:^10}",
        "Month", "Amount", "Unit Price", "Planned", "Additional", "Total"
    );
    println!(" {:-^7} | {:-^10} | {:-^10} | {:-^10} | {:-^10} | {:-^10}", "", "", "", "", "", ""); // divider

    for (year_nr, content) in depot_element.history.iter().collect::<Vec<(&u16, &InvestmentYear)>>() {
        for month in content.months.iter() {
            let planned_transactions: f64 = depot_element.get_planned_transactions(*year_nr, month.month_nr);
            println!(
                " {:4} {:>2} | {:>10.2} | {:>10.2} | {:>10.2} | {:>10.2} | {:>10.2}",
                year_nr,
                month.month_nr,
                month.amount,
                month.price_per_unit,
                planned_transactions,
                month.additional_transactions,
                (planned_transactions + month.additional_transactions),
            );
        }

        // // table for months
        // println!("");
        // println!("The goal is to spend less than {} % of monthly income", datafile.accounting.goal * 100.0);
        // println!("");
        // println!(
        //     " {:^7} | {:^10} | {:^10} | {:^10} | {:^10} | {}",
        //     "Month", "Income", "Expenses", "Difference", "Percentage", "Goal met?"
        // );
        // println!(" {:-^7} | {:-^10} | {:-^10} | {:-^10} | {:-^10} | {:-^9}", "", "", "", "", "", ""); // divider
        // for month in &year.months {
        //     let goal_met: &str = match (month.percentage_1() * 100.0) as u64 {
        //         0 => "-", // dont show true/false if there is no value
        //         _ => match month.percentage_1() <= datafile.accounting.goal {
        //             true => "true",
        //             false => "false",
        //         },
        //     };

        //     println!(
        //         " {:4} {:>2} | {:>10.2} | {:>10.2} | {:>10.2} | {:>8.0} % | {}",
        //         year.year_nr,
        //         month.month_nr(),
        //         month.income(),
        //         month.expenses(),
        //         month.difference(),
        //         month.percentage_100(),
        //         goal_met
        //     );
        // }
        // println!("");

        // // table for different statics for year
        // println!(
        //     " {:>7} | {:^10} | {:^10} | {:^10} | {:^10} | {}",
        //     year.year_nr, "Income", "Expenses", "Difference", "Percentage", "Goal met?"
        // );
        // println!(" {:-^7} | {:-^10} | {:-^10} | {:-^10} | {:-^10} | {:-^9}", "", "", "", "", "", ""); // divider

        // // Sum
        // let year_diff: f64 = year.get_sum_income() - year.get_sum_expenses();
        // let year_perc: f64 = (year.get_sum_expenses() / year.get_sum_income()) * 100.0;

        // let months_with_goal_hit = year
        //     .months
        //     .iter()
        //     .filter(|&m| (m.percentage_1() <= datafile.accounting.goal) && m.percentage_1() != 0.0)
        //     .count() as f32;
        // let months_with_data = year.months.iter().filter(|&m| *m != AccountingMonth::default(m.month_nr())).count() as f32;
        // let goals_over_months = format!("{} / {}", months_with_goal_hit, months_with_data);

        // println!(
        //     " {:>7} | {:>10.2} | {:>10.2} | {:>10.2} | {:>8.0} % | {:^9}",
        //     "Sum",
        //     year.get_sum_income(),
        //     year.get_sum_expenses(),
        //     year_diff,
        //     year_perc,
        //     goals_over_months,
        // );

        // // Median
        // let goals_in_year_perc = format!("{:3.0} %", (months_with_goal_hit / months_with_data) * 100.0);

        // let Ok(median_income) = year.get_median_income() else {
        //     println!("There is no data in this year.");
        //     return;
        // };
        // let Ok(median_expenses) = year.get_median_expenses() else {
        //     println!("There is no data in this year.");
        //     return;
        // };
        // let Ok(median_difference) = year.get_median_difference() else {
        //     println!("There is no data in this year.");
        //     return;
        // };
        // let Ok(median_percentage) = year.get_median_percentage_100() else {
        //     println!("There is no data in this year.");
        //     return;
        // };

        // println!(
        //     " {:>7} | {:>10.2} | {:>10.2} | {:>10.2} | {:>8.0} % | {:^9}",
        //     "Median", median_income, median_expenses, median_difference, median_percentage, goals_in_year_perc
        // );
        // println!("");
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
