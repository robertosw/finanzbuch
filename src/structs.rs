use serde::Deserialize;
use serde::Serialize;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::process::exit;

const FILE: &'static str = "/root/project/sample.yaml";

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct YamlFile {
    pub version: u8,
    pub goal: f32,
    pub years: Vec<Year>,
}
impl YamlFile {
    /// returns the contents of the yaml file as a `YamlFile`, sorted by years ascending
    pub fn read() -> YamlFile {
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

        let mut ymlfile: YamlFile = match serde_yaml::from_str(&content) {
            Ok(v) => v,
            Err(e) => {
                println!("error reading in file contents > {:?}", e);
                exit(1);
            }
        };

        ymlfile.years.sort_by(|a: &Year, b: &Year| a.year_nr.cmp(&b.year_nr));
        return ymlfile;
    }

    pub fn write(&self) {
        let yaml = match serde_yaml::to_string(self) {
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

    pub fn add_default_year(&mut self, year_nr: u16) {
        // TODO check if exists
        self.years.insert(0, Year::default(year_nr));
        self.years.sort_by(|a, b| a.year_nr.cmp(&b.year_nr));
    }

    pub fn add_year_with_month(&mut self, year_nr: u16, new_month: Month) {
        // TODO check if exists
        self.years.insert(0, Year::default(year_nr));

        match self.years.get_mut(0) {
            Some(ymlyear) => ymlyear.insert_or_override_month(new_month),
            None => panic!("Inserting a new Year did not work"),
        }

        self.years.sort_by(|a, b| a.year_nr.cmp(&b.year_nr));
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Year {
    pub year_nr: u16,
    pub income: f64,
    pub expenses: f64,
    pub months: [Month; 12],
}
impl Year {
    pub fn default(year_nr: u16) -> Self {
        return Year {
            year_nr,
            income: 0.0,
            expenses: 0.0,
            months: Month::default_months(),
        };
    }

    pub fn insert_or_override_month(&mut self, new_month: Month) {
        let month_nr = new_month.month_nr;
        let ymlmonth: &mut Month = &mut self.months[month_nr as usize - 1];

        if *ymlmonth != Month::default(ymlmonth.month_nr) {
            // ("{:0>2?}")
            //       2 - width
            //      > -- where to align actual value, > means {fill}{value}, < means {value}{fill}
            //     0 --- with what to fill
            println!("{:0>2?}.{:4?} will be overwritten!", ymlmonth.month_nr, self.year_nr);
            println!("Old content: {:?}", *ymlmonth);

            // reset this month to default = subtract from year sum
            self.income -= ymlmonth.income;
            self.expenses -= ymlmonth.expenses;
            *ymlmonth = Month::default(ymlmonth.month_nr);
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
pub struct Month {
    pub month_nr: u8,
    pub income: f64,
    pub expenses: f64,
    pub difference: f64,
    pub percentage: f64,
}
impl Month {
    pub fn default(month: u8) -> Self {
        return Month {
            month_nr: month,
            income: 0.0,
            expenses: 0.0,
            difference: 0.0,
            percentage: 0.0,
        };
    }

    pub fn default_months() -> [Self; 12] {
        return [
            Month::default(1),
            Month::default(2),
            Month::default(3),
            Month::default(4),
            Month::default(5),
            Month::default(6),
            Month::default(7),
            Month::default(8),
            Month::default(9),
            Month::default(10),
            Month::default(11),
            Month::default(12),
        ];
    }
}
