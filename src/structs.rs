use serde::Deserialize;
use serde::Serialize;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::process::exit;
use std::ptr::eq;

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

    // pub fn add_default_year(&mut self, year_nr: u16) {
    //     match self.years.iter().any(|y| y.year_nr == year_nr) {
    //         true => {
    //             self.years.insert(0, Year::default(year_nr));
    //             self.years.sort_by(|a, b| a.year_nr.cmp(&b.year_nr));
    //         }
    //         false => {
    //             println!("Error while trying to add")
    //         }
    //     }
    // }

    /// - returns with `Err` if this year already exists.
    /// - if not, adds this year with the given month
    // pub fn try_add_year_with_month(&mut self, year_nr: u16, new_month: Month) -> Result<(), ()> {
    //     // does this year already exist
    //     match self.years.iter().any(|e: &Year| e.year_nr == year_nr) {
    //         true => return Err(()),
    //         false => (),
    //     }

    //     self.years.insert(0, Year::default(year_nr));

    //     match self.years.get_mut(0) {
    //         Some(ymlyear) => ymlyear.insert_or_overwrite_month(new_month),
    //         None => panic!("Could not get mutable reference to Year in Vec"),
    //     }

    //     self.years.sort_by(|a, b| a.year_nr.cmp(&b.year_nr));
    //     return Ok(());
    // }

    /// - If the year already exists, inserts or overwrites the month
    /// - If the year does not exist, adds it (with the month) to the Vec
    pub fn add_or_insert_year_with_month(&mut self, year_nr: u16, new_month: Month) {
        // does this year already exist in self.years
        let index = match self.years.iter().position(|e: &Year| e.year_nr == year_nr) {
            Some(index) => index,
            None => {
                self.years.insert(0, Year::default(year_nr));
                0
            }
        };

        match self.years.get_mut(index) {
            Some(ymlyear) => ymlyear.insert_or_overwrite_month(new_month),
            None => panic!("Could not get mutable reference to Year in Vec"),
        }

        self.years.sort_by(|a, b| a.year_nr.cmp(&b.year_nr));
    }

    /// - if the year does not already exist, adds it to YamlFile.years
    /// - changes nothing if the year exists
    /// - returns the year as &mut Year
    pub fn add_year_soft(&mut self, year_nr: u16) -> &mut Year {
        let mut lt_index: isize = -1; // -1 means, not yet set
        let mut gt_index: isize = -1; // year_nr is greater then  the year at this index
        let mut eq_index: Option<usize> = None;

        // if 0
        // 1 2 4 5
        // lt_index = 0
        // gt_index = -1
        //
        // if 3
        // 1 2 4 5
        // lt_index = 2
        // gt_index = 1
        //
        // if 6
        // 1 2 4 5
        // lt_index = -1
        // gt_index = 3

        for (index, year) in self.years.iter_mut().enumerate() {
            if year.year_nr == year_nr {
                eq_index = Some(index);
                break;
            } else if year_nr > year.year_nr {
                gt_index = index as isize;
            } else if (year_nr < year.year_nr) && (lt_index == -1) {
                // checking for -1 is only needed for "less-then" because the years are ordered ascendingly
                lt_index = index as isize;
            }
        }
        // getting here means, the year does not yet exist

        match eq_index {
            Some(index) => match self.years.get_mut(index) {
                Some(y) => return y,
                None => panic!("thats not supposed to be possible"),
            },
            None => (),
        }

        let insert_index: usize = {
            if lt_index == 0 && gt_index == -1 {
                // all years are greater than year_nr
                0
            } else if lt_index > -1 && gt_index > -1 {
                // add somewhere in the middle
                lt_index as usize
            } else if (lt_index == -1) && (gt_index > -1) {
                // all years are smaller than year_nr
                gt_index as usize + 1
            } else {
                panic!("logic error");
            }
        };

        self.years.insert(insert_index, Year::default(year_nr));
        match self.years.get_mut(insert_index) {
            Some(y) => return y,
            None => panic!("thats not supposed to be possible"),
        };

        // // getting here means the year is not included, so add it and return with reference

        // // does this year already exist in self.years
        // let index = match self.years.iter().position(|e: &Year| e.year_nr == year_nr) {
        //     Some(index) => index,
        //     None => 0,
        // };

        // match self.years.get_mut(index) {
        //     Some(ymlyear) => ymlyear.insert_or_overwrite_month(new_month),
        //     None => panic!("Could not get mutable reference to Year in Vec"),
        // }

        // self.years.sort_by(|a, b| a.year_nr.cmp(&b.year_nr));
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

    pub fn insert_or_overwrite_month(&mut self, new_month: Month) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// run with cargo test -- --nocapture
    fn year_finder() {
        // test setup
        let year_nr = 2032;
        let mut years = vec![
            Year::default(2012), // 0
            Year::default(2017), // 1
            Year::default(2020), // 2
            Year::default(2021), // 3
            Year::default(2024), // 4
            Year::default(2025), // 5
            Year::default(2031), // 6
        ];

        // copied from YamlFile.add_year()

        let mut lt_index: isize = -1; // -1 means, not yet set
        let mut gt_index: isize = -1; // year_nr is greater then the year at this index

        // if 0
        // 1 2 4 5
        // lt_index = 0
        // gt_index = -1
        //
        // if 3
        // 1 2 4 5
        // lt_index = 2
        // gt_index = 1
        //
        // if 6
        // 1 2 4 5
        // lt_index = -1
        // gt_index = 3

        for (index, year) in years.iter_mut().enumerate() {
            // check if the current year is the given year number, if so, return with reference
            if year.year_nr == year_nr {
                println!("exact match! index {index}");
                return;
            } else if year_nr > year.year_nr {
                gt_index = index as isize;
            } else if (year_nr < year.year_nr) && (lt_index == -1) {
                // checking for -1 is only needed for "less-then" because the years are ordered ascendingly
                lt_index = index as isize;
            }
        }
        println!("lt_index {lt_index}");
        println!("gt_index {gt_index}");
    }

    fn add_year() {}
}
