use dialoguer::*;
use finance_yaml::accounting::accounting_year::AccountingYear;
use finance_yaml::*;
use rgb::RGB8;
use std::path::PathBuf;
use textplots::*;

/// Lets user import a csv file, choose which column contains monetary values and import these values into a specified year and month
pub fn accounting_csv_import() {
    println!("\nThis dialogue allows you to import values from a csv file and insert them into a selected month.\n");

    // Loop until the given path points to a valid .csv file
    let csv_path: PathBuf = loop {
        let path_str: String = Input::new().with_prompt("Path to csv file").interact_text().unwrap();

        let path = PathBuf::from(&path_str);
        let ext = match &path.extension() {
            Some(ext) => *ext,
            None => {
                println!("{:?} does not have a extension.", &path_str);
                continue;
            }
        };

        let ext_str = match ext.to_str() {
            Some(str) => str,
            None => {
                println!("The extension of {:?} could not be parsed.", &path_str);
                continue;
            }
        };

        match path.is_file() && (ext_str == "csv") {
            true => break path,
            false => {
                println!("{:?} does not point to a .csv file", path);
                continue;
            }
        };
    };

    let csv_lines: Vec<Vec<String>> = get_csv_contents_with_header(&csv_path);
    let csv_headers: &Vec<String> = match csv_lines.get(0) {
        Some(v) => v,
        None => {
            println!("CSV file is empty");
            return;
        }
    };
    let csv_first_line: &Vec<String> = match csv_lines.get(1) {
        Some(v) => v,
        None => {
            println!("CSV has no content");
            return;
        }
    };

    let selection_items: Vec<String> = csv_first_line
        .iter()
        .map(|val| String::from(csv_headers.get(0).unwrap().clone() + "; " + val.as_str()))
        .collect();

    let selected_col = Select::new()
        .with_prompt(
            "\nPlease choose the column which contains the monetary values you want to import.\n\
            Each line below represents one column of the csv file in this format: 'Header; Content of first line'",
        )
        .items(&selection_items)
        .interact()
        .unwrap();

    println!("\nIn which year and month do you want to import this data?");
    let year_nr: u16 = Input::new().with_prompt("Year").interact_text().unwrap();
    let month_nr: u8 = Input::new().with_prompt("Month").interact_text().unwrap();
    // TODO note

    // User input done, save data
    let mut datafile = DataFile::read();

    let mut monetary_csv_values: Vec<f64> = Vec::new();
    for entry in csv_lines {
        let value_f64: f64 = SanitizeInput::monetary_string_to_f64(&entry[selected_col]).unwrap();
        monetary_csv_values.push(value_f64);
    }

    // Sum up income and expenses
    let sum_positives: f64 = monetary_csv_values.iter().filter(|&v| v > &0.0).sum();
    let sum_negatives: f64 = monetary_csv_values.iter().filter(|&v| v < &0.0).sum();

    let acc_year = datafile.accounting.add_or_get_year(year_nr);
    acc_year.months[month_nr as usize - 1].set_income(sum_positives);
    acc_year.months[month_nr as usize - 1].set_expenses(sum_negatives);

    datafile.write();
    println!(" --- Importing csv data done ---");
}

pub fn accounting_manual_input() {
    println!("Adding values into given year and month.");
    let year_nr: u16 = Input::new().with_prompt("Year").interact_text().unwrap();
    let month_nr: u8 = Input::new().with_prompt("Month").interact_text().unwrap();
    let income: f64 = SanitizeInput::monetary_string_to_f64(&Input::new().with_prompt("Income").interact_text().unwrap()).unwrap();
    let expenses: f64 = SanitizeInput::monetary_string_to_f64(&Input::new().with_prompt("Expenses").interact_text().unwrap()).unwrap();
    // TODO note

    println!("Saving.. In: {income} Out: {expenses} to {year_nr} {month_nr}");

    let mut datafile = DataFile::read();

    datafile
        .accounting
        .add_or_get_year(year_nr)
        .insert_or_overwrite_month(AccountingMonth::new(month_nr, income, expenses, String::new()));

    datafile.write();
}

pub fn accounting_table_output() {
    println!("Choose a year to display.");
    let year_nr: u16 = Input::new().with_prompt("Year").interact_text().unwrap();

    // Print out a table all data of the year
    let datafile = DataFile::read();
    let Some(year) = datafile.accounting.history.get(&year_nr) else {
        println!("There is no data for the year {year_nr}.");
        return;
    };

    _print_accounting_table(&year, &datafile);

    // Print out a graph with all 12 months of income and expenses
    let monthly_incomes: Vec<f64> = datafile.accounting.history.get(&2023).unwrap().months.iter().map(|m| m.income()).collect();
    let monthly_expenses: Vec<f64> = datafile.accounting.history.get(&2023).unwrap().months.iter().map(|m| m.expenses()).collect();

    // x = months, y = values
    let incomes_xy: Vec<(f32, f32)> = monthly_incomes.iter().enumerate().map(|(i, v)| (i as f32 + 1.0, v.clone() as f32)).collect();
    let expenses_xy: Vec<(f32, f32)> = monthly_expenses.iter().enumerate().map(|(i, v)| (i as f32 + 1.0, v.clone() as f32)).collect();

    Chart::new(160, 90, 1.0, 12.0)
        .linecolorplot(&Shape::Lines(&expenses_xy), RGB8 { r: 255, g: 0, b: 0 })
        .linecolorplot(&Shape::Lines(&incomes_xy), RGB8 { r: 0, g: 255, b: 0 })
        .x_axis_style(LineStyle::None)
        .y_axis_style(LineStyle::None)
        .x_label_format(LabelFormat::Value)
        .y_label_format(LabelFormat::Value)
        .display();
}

fn _print_accounting_table(year: &AccountingYear, datafile: &DataFile) {
    // target:
    //    Month  |   Income   |  Expenses  | Difference | Percentage | Goal met?
    //    ------- | ---------- | ---------- | ---------- | ---------- | ---------
    //    2023 01 |       0.00 |       0.00 |       0.00 |        0 % | -
    //    2023 02 |       0.00 |       0.00 |       0.00 |        0 % | -
    //    2023 03 |       0.00 |       0.00 |       0.00 |        0 % | -
    //    2023 04 |       0.00 |       0.00 |       0.00 |        0 % | -
    //    2023 05 |     378.76 |    3445.18 |   -3066.43 |      910 % | false
    //    2023 06 |       0.00 |       0.00 |       0.00 |        0 % | -
    //    2023 07 |       0.00 |       0.00 |       0.00 |        0 % | -
    //    2023 08 |       0.00 |       0.00 |       0.00 |        0 % | -
    //    2023 09 |   12345.00 |  123456.00 | -111111.00 |     1000 % | false
    //    2023 10 |   12345.00 |    1234.00 |   11111.00 |       10 % | true
    //    2023 11 |       0.00 |       0.00 |       0.00 |        0 % | -
    //    2023 12 |    1111.11 |    2222.22 |   -1111.11 |      200 % | false
    //    ------- | ---------- | ---------- | ---------- | ---------- | ---------
    //       2023 |   26179.87 |  130357.40 |          - |          % | -

    // table for months
    println!("");
    println!("The goal is to spend less than {} % of monthly income", datafile.accounting.goal * 100.0);
    println!("");
    println!(
        " {:^7} | {:^10} | {:^10} | {:^10} | {:^10} | {}",
        "Month", "Income", "Expenses", "Difference", "Percentage", "Goal met?"
    );
    println!(" {:-^7} | {:-^10} | {:-^10} | {:-^10} | {:-^10} | {:-^9}", "", "", "", "", "", ""); // divider
    for month in &year.months {
        let goal_met: &str = match (month.percentage_1() * 100.0) as u64 {
            0 => "-", // dont show true/false if there is no value
            _ => match month.percentage_1() <= datafile.accounting.goal {
                true => "true",
                false => "false",
            },
        };

        println!(
            " {:4} {:>2} | {:>10.2} | {:>10.2} | {:>10.2} | {:>8.0} % | {}",
            year.year_nr,
            month.month_nr(),
            month.income(),
            month.expenses(),
            month.difference(),
            month.percentage_100(),
            goal_met
        );
    }
    println!("");

    // table for different statics for year
    println!(
        " {:>7} | {:^10} | {:^10} | {:^10} | {:^10} | {}",
        year.year_nr, "Income", "Expenses", "Difference", "Percentage", "Goal met?"
    );
    println!(" {:-^7} | {:-^10} | {:-^10} | {:-^10} | {:-^10} | {:-^9}", "", "", "", "", "", ""); // divider

    // Sum
    let year_diff: f64 = year.get_sum_income() - year.get_sum_expenses();
    let year_perc: f64 = (year.get_sum_expenses() / year.get_sum_income()) * 100.0;

    let months_with_goal_hit = year
        .months
        .iter()
        .filter(|&m| (m.percentage_1() <= datafile.accounting.goal) && m.percentage_1() != 0.0)
        .count() as f32;
    let months_with_data = year.months.iter().filter(|&m| *m != AccountingMonth::default(m.month_nr())).count() as f32;
    let goals_over_months = format!("{} / {}", months_with_goal_hit, months_with_data);

    println!(
        " {:>7} | {:>10.2} | {:>10.2} | {:>10.2} | {:>8.0} % | {:^9}",
        "Sum",
        year.get_sum_income(),
        year.get_sum_expenses(),
        year_diff,
        year_perc,
        goals_over_months,
    );

    // Median
    let goals_in_year_perc = format!("{:3.0} %", (months_with_goal_hit / months_with_data) * 100.0);

    let Ok(median_income) = year.get_median_income() else {
        println!("There is no data in this year.");
        return;
    };
    let Ok(median_expenses) = year.get_median_expenses() else {
        println!("There is no data in this year.");
        return;
    };
    let Ok(median_difference) = year.get_median_difference() else {
        println!("There is no data in this year.");
        return;
    };
    let Ok(median_percentage) = year.get_median_percentage_100() else {
        println!("There is no data in this year.");
        return;
    };

    println!(
        " {:>7} | {:>10.2} | {:>10.2} | {:>10.2} | {:>8.0} % | {:^9}",
        "Median", median_income, median_expenses, median_difference, median_percentage, goals_in_year_perc
    );
    println!("");
}
