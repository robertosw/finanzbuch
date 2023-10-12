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
    year_nr: u16,
    income: f64,
    expenses: f64,
    months: [YamlMonth; 12],
}
impl YamlYear {
    fn default(year_nr: u16) -> Self {
        return YamlYear {
            year_nr,
            income: 0.0,
            expenses: 0.0,
            months: YamlMonth::default_months(),
        };
    }

    fn insert_or_override(&mut self, new_month: YamlMonth) {
        let month_nr = new_month.month_nr;
        let ymlmonth: &mut YamlMonth = &mut self.months[month_nr as usize - 1];

        if *ymlmonth != YamlMonth::default(ymlmonth.month_nr) {
            // ("{:0>2?}")
            //       2 - width
            //      > -- where to align actual value, > means {fill}{value}, < means {value}{fill}
            //     0 --- with what to fill
            println!("{:0>2?}.{:4?} will be overwritten!", ymlmonth.month_nr, self.year_nr);
            println!("Old content: {:?}", *ymlmonth);

            // reset this month to default = subtract from year sum
            self.income -= ymlmonth.income;
            self.expenses -= ymlmonth.expenses;
            *ymlmonth = YamlMonth::default(ymlmonth.month_nr);
        }

        // write given values into month and add to year sum
        ymlmonth.income = new_month.income;
        ymlmonth.expenses = new_month.expenses;
        ymlmonth.difference = new_month.difference;
        ymlmonth.percentage = new_month.percentage;
        self.income += new_month.income;
        self.expenses += new_month.expenses;
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
struct YamlMonth {
    month_nr: u8,
    income: f64,
    expenses: f64,
    difference: f64,
    percentage: f64,
}
impl YamlMonth {
    fn default(month: u8) -> Self {
        return YamlMonth {
            month_nr: month,
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
    let (input_income, input_expenses, input_month_nr, input_year_nr): (f64, f64, u8, u16) = generate_random_input();
    println!("in {}, out {}, month {}, year {}", input_income, input_expenses, input_month_nr, input_year_nr);

    let calc_difference: f64 = input_income - input_expenses;
    let calc_percentage: f64 = input_expenses / input_income;
    println!("diff {}, perc {}", calc_difference, calc_percentage);

    // read file and sort ascending
    let mut ymlfile = read();
    ymlfile.years.sort_by(|a: &YamlYear, b: &YamlYear| a.year_nr.cmp(&b.year_nr));

    // check if the targeted year already exists
    match ymlfile.years.iter().position(|e: &YamlYear| e.year_nr == input_year_nr) {
        Some(index) => match ymlfile.years.get_mut(index) {
            // the given year exists in ymlfile.years
            Some(ymlyear) => ymlyear.insert_or_override(YamlMonth {
                month_nr: input_month_nr,
                income: input_income,
                expenses: input_expenses,
                difference: calc_difference,
                percentage: calc_percentage,
            }),
            None => panic!("This case should never happen"),
        },
        None => {
            // the given year does not exist in ymlfile.years
            ymlfile.years.insert(0, YamlYear::default(input_year_nr));
            match ymlfile.years.get_mut(0) {
                Some(ymlyear) => ymlyear.insert_or_override(YamlMonth {
                    month_nr: input_month_nr,
                    income: input_income,
                    expenses: input_expenses,
                    difference: calc_difference,
                    percentage: calc_percentage,
                }),
                None => panic!("Inserting a new YamlYear did not work"),
            }
            ymlfile.years.sort_by(|a, b| a.year_nr.cmp(&b.year_nr));
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
                year_nr: YEAR,
                income: 0.0,
                expenses: 0.0,
                months: YamlMonth::default_months(),
            }],
        };

        match ymlfile.years.iter().position(|y| y.year_nr == YEAR) {
            Some(index) => match ymlfile.years.get_mut(index) {
                Some(ymlyear) => {
                    let month = &mut ymlyear.months[MONTH as usize - 1];

                    // I just created this test because I wasn't sure that this comparison is done correctly
                    // other languages might have compared the datatype of both sides and would always say its the same
                    assert!(*month == YamlMonth::default(month.month_nr));
                    assert_ne!(*month, YamlMonth::default(month.month_nr + 1));
                }
                None => (),
            },
            None => (),
        }
    }
}
