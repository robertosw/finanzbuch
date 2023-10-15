use csv::ReaderBuilder;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use crate::structs::config::Config;

pub fn input_month_from_csv(path: &Path, year_nr: u16, month_nr: u8) {
    let mut config = Config::read();

    // open file for reading
    let mut file: File = match File::options().read(true).truncate(false).open(path) {
        Ok(file) => file,
        Err(_) => panic!("Could not open {:?}", path),
    };

    // read file content
    let content: String = {
        let mut temp: String = String::new();
        match file.read_to_string(&mut temp) {
            Ok(v) => v,
            Err(_) => panic!("Error reading file {:?}", path),
        };
        temp
    };

    // read file content into CSV Reader
    let mut reader = ReaderBuilder::new().delimiter(b';').from_reader(content.as_bytes());

    // let user choose column with values
    let header = match reader.headers() {
        Ok(val) => val,
        Err(e) => panic!("Could not get CSV Header: {e}"),
    };
    let chosen_column_id: usize = let_user_choose_column_index(header);

    // Get values from that column
    let mut column_values: Vec<f64> = Vec::new();
    for record in reader.records() {
        let record = match record {
            Ok(string_rec) => string_rec,
            Err(_) => panic!("Could not transform csv record"),
        };
        let value_string: &String = &record[chosen_column_id].replace(",", ".");

        // parse into float
        let value_f64 = match value_string.parse::<f64>() {
            Ok(i) => i.to_owned(),
            Err(..) => panic!("Value in the specified row is not a number: {}", &record[chosen_column_id]),
        };
        column_values.push(value_f64);
    }

    // Calculate
    let mut income: f64 = 0.0;
    let mut expenses: f64 = 0.0;

    for value in column_values {
        if value > 0.0 {
            income += value;
        } else if value < 0.0 {
            expenses += value;
        }
    }

    config.add_or_get_year(year_nr).months[month_nr as usize - 1].income = income;
    config.add_or_get_year(year_nr).months[month_nr as usize - 1].expenses = expenses;
    config.add_or_get_year(year_nr).months[month_nr as usize - 1].difference = expenses + income;
    config.add_or_get_year(year_nr).months[month_nr as usize - 1].percentage = (expenses / income).abs();

    config.write();
}

fn let_user_choose_column_index(header: &csv::StringRecord) -> usize {
    // Print all header fields with numbers for user to choose from
    for (index, element) in header.iter().enumerate() {
        println!("{:2}: {}", index, element);
    }

    println!("");
    println!("Please choose a row containing numbers");
    loop {
        let mut input_text = String::new();
        io::stdin().read_line(&mut input_text).expect("failed to read from stdin");

        let trimmed = input_text.trim();
        let input_int: usize = match trimmed.parse::<usize>() {
            Ok(i) => i as usize,
            Err(_) => {
                println!("this was not an integer: {}", trimmed);
                continue;
            }
        };

        // Validate User input
        match header.get(input_int) {
            Some(_) => return input_int,
            None => {
                println!("{} does not represent a header field.", input_int);
                continue;
            }
        };
    }
}
