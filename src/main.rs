use serde::Deserialize;
use serde::Serialize;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::process::exit;
use tinyrand::Rand;
use tinyrand::RandRange;
use tinyrand::Seeded;
use tinyrand::StdRand;
use tinyrand_std::ClockSeed;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
struct YamlFile {
    version: String,
    goal: f32,
    years: Vec<YamlYear>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
struct YamlYear {
    year: u16,
    income: f64,
    expenses: f64,
    months: [YamlMonth; 12],
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
struct YamlMonth {
    month: u8,
    income: f64,
    expenses: f64,
    difference: f64,
    percentage: f64,
}
impl YamlMonth {
    fn default(month: u8) -> Self {
        return YamlMonth {
            month,
            income: 0.0,
            expenses: 0.0,
            difference: 0.0,
            percentage: 0.0,
        };
    }

    fn default_months() -> [Self; 12] {
        return [
            YamlMonth::default(1),
            YamlMonth::default(2),
            YamlMonth::default(3),
            YamlMonth::default(4),
            YamlMonth::default(5),
            YamlMonth::default(6),
            YamlMonth::default(7),
            YamlMonth::default(8),
            YamlMonth::default(9),
            YamlMonth::default(10),
            YamlMonth::default(11),
            YamlMonth::default(12),
        ];
    }
}

const FILE: &'static str = "/root/project/sample.yaml";

fn main() {
    let (input_income, input_expenses, input_month, input_year): (f64, f64, u8, u16) = generate_random_input();
    println!("in {}, out {}, month {}, year {}", input_income, input_expenses, input_month, input_year);

    let calc_difference: f64 = input_income - input_expenses;
    let calc_percentage: f64 = input_expenses / input_income;
    println!("diff {}, perc {}", calc_difference, calc_percentage);

    // read file and sort ascending
    let mut ymlfile = read();
    ymlfile.years.sort_by(|a: &YamlYear, b: &YamlYear| a.year.cmp(&b.year));

    // check if the targeted year already exists
    match ymlfile.years.iter().position(|e: &YamlYear| e.year == input_year) {
        Some(index) => match ymlfile.years.get_mut(index) {
            // the given year exists in ymlfile.years
            Some(ymlyear) => {
                let ymlmonth: &mut YamlMonth = &mut ymlyear.months[input_month as usize - 1];

                // is this month not "empty" = not default values
                if *ymlmonth != YamlMonth::default(ymlmonth.month) {
                    // ("{:0>2?}")
                    //       2 - width
                    //      > -- where to align actual value, > means {fill}{value}, < means {value}{fill}
                    //     0 --- with what to fill
                    println!("{:0>2?}.{:4?} will be overwritten!", ymlmonth.month, ymlyear.year);
                    println!("Old content: {:?}", *ymlmonth);

                    // reset this month to default = subtract from year sum
                    ymlyear.income -= ymlmonth.income;
                    ymlyear.expenses -= ymlmonth.expenses;
                    *ymlmonth = YamlMonth::default(ymlmonth.month);
                }

                // write given values into month and add to year sum
                ymlmonth.income = input_income;
                ymlmonth.expenses = input_expenses;
                ymlmonth.difference = calc_difference;
                ymlmonth.percentage = calc_percentage;
                ymlyear.income += input_income;
                ymlyear.expenses += input_expenses;
            }
            None => panic!("This case should never happen"),
        },
        None => {
            // the given year does not exist in ymlfile.years

            ymlfile.years.insert(
                0,
                YamlYear {
                    year: input_year,
                    income: 0.0,
                    expenses: 0.0,
                    months: YamlMonth::default_months(),
                },
            );

            match ymlfile.years.get_mut(0) {
                Some(ymlyear) => {
                    let ymlmonth: &mut YamlMonth = &mut ymlyear.months[input_month as usize - 1];

                    // write given values into month and add to year sum
                    ymlmonth.income = input_income;
                    ymlmonth.expenses = input_expenses;
                    ymlmonth.difference = calc_difference;
                    ymlmonth.percentage = calc_percentage;
                    ymlyear.income += input_income;
                    ymlyear.expenses += input_expenses;
                }
                None => panic!("Inserting a new YamlYear did not work"),
            }

            ymlfile.years.sort_by(|a, b| a.year.cmp(&b.year));
        }
    }

    // beim einfügen in ein Jahr und Monat überprüfen ob in dem Monat schon Werte waren
    // Wenn nicht, zur Jahres summe einfach die Monatswerte aufaddieren
    // Wenn Monat überschrieben, dann erst Differenz zu vorherigen Werten berechnen, überschreiben und im Jahr aufaddieren
    write(ymlfile);
}

/// return values
/// - income, expenses, month, year
fn generate_random_input() -> (f64, f64, u8, u16) {
    let seed = ClockSeed::default().next_u64();
    let mut rand = StdRand::seed(seed);
    let rand_month: u8 = rand.next_range(1 as usize..13 as usize) as u8;
    let rand_year: u16 = rand.next_range(2000 as usize..2024 as usize) as u16;
    let rand_income: f64 = rand.next_u16() as f64 / 11.11;
    let rand_expenses: f64 = rand.next_u16() as f64 / 11.11;
    return (rand_income, rand_expenses, rand_month, rand_year);
}

fn read() -> YamlFile {
    let mut file = match OpenOptions::new().create(false).read(true).open(FILE) {
        Ok(file) => file,
        Err(e) => {
            println!("error at opening yaml file > {:?}", e);
            exit(1);
        }
    };

    let mut content: String = String::new();
    match file.read_to_string(&mut content) {
        Ok(size) => size,
        Err(e) => {
            println!("error reading in file contents > {:?}", e);
            exit(1);
        }
    };

    let ymlfile: YamlFile = match serde_yaml::from_str(&content) {
        Ok(v) => v,
        Err(e) => {
            println!("error reading in file contents > {:?}", e);
            exit(1);
        }
    };

    return ymlfile;
}

fn write(ymlfile: YamlFile) {
    let yaml = match serde_yaml::to_string(&ymlfile) {
        Ok(v) => v,
        Err(e) => {
            println!("error at serde_yaml::to_string > {:?}", e);
            exit(1);
        }
    };

    match OpenOptions::new().create(true).truncate(true).write(true).open(FILE) {
        Ok(mut file) => {
            match file.write_all(yaml.as_bytes()) {
                Ok(_) => {}
                Err(e) => {
                    println!("error at writing yaml file > {:?}", e);
                    exit(1);
                }
            };
        }
        Err(e) => {
            println!("error at opening yaml file > {:?}", e);
            exit(1);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn month_compare() {
        const MONTH: u8 = 1;
        const YEAR: u16 = 2000;

        let mut ymlfile = YamlFile {
            version: String::from(""),
            goal: 0.0,
            years: vec![YamlYear {
                year: YEAR,
                income: 0.0,
                expenses: 0.0,
                months: YamlMonth::default_months(),
            }],
        };

        match ymlfile.years.iter().position(|y| y.year == YEAR) {
            Some(index) => match ymlfile.years.get_mut(index) {
                Some(ymlyear) => {
                    let month = &mut ymlyear.months[MONTH as usize - 1];

                    // I just created this test because I wasn't sure that this comparison is done correctly
                    // other languages might have compared the datatype of both sides and would always say its the same
                    assert!(*month == YamlMonth::default(month.month));
                    assert_ne!(*month, YamlMonth::default(month.month + 1));
                }
                None => (),
            },
            None => (),
        }
    }
}
