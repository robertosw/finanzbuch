use serde::Deserialize;
use serde::Serialize;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;

const FILE: &'static str = "/root/project/sample.yaml";

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct YamlFile {
    pub version: u8,
    pub goal: f64,
    pub years: Vec<Year>,
}
impl YamlFile {
    /// returns the contents of the yaml file as a `YamlFile`, sorted by years ascending
    pub fn read() -> YamlFile {
        let mut file = match OpenOptions::new().create(false).read(true).open(FILE) {
            Ok(file) => file,
            Err(e) => panic!("error at opening yaml file > {:?}", e),
        };

        let mut content: String = String::new();
        match file.read_to_string(&mut content) {
            Ok(size) => size,
            Err(e) => panic!("error reading in file contents > {:?}", e),
        };

        let mut ymlfile: YamlFile = match serde_yaml::from_str(&content) {
            Ok(v) => v,
            Err(e) => panic!("error reading in file contents > {:?}", e),
        };

        ymlfile.years.sort_by(|a: &Year, b: &Year| a.year_nr.cmp(&b.year_nr));
        return ymlfile;
    }

    pub fn write(&self) {
        let yaml = match serde_yaml::to_string(self) {
            Ok(v) => v,
            Err(e) => panic!("error at serde_yaml::to_string > {:?}", e),
        };

        let mut file = match OpenOptions::new().create(true).truncate(true).write(true).open(FILE) {
            Ok(file) => file,
            Err(e) => panic!("error at opening yaml file > {:?}", e),
        };

        match file.write_all(yaml.as_bytes()) {
            Ok(_) => (),
            Err(e) => panic!("error at writing yaml file > {:?}", e),
        };
    }

    /// - if the year does not already exist, adds it to YamlFile.years with default values
    /// - changes nothing if the year exists
    /// - returns the year as a mutable reference (`&mut Year`)`
    ///   - this allows function chaining: `YamlFile.add_or_get_year().function_on_year()`
    pub fn add_or_get_year(&mut self, year_nr: u16) -> &mut Year {
        let mut lt_index: Option<usize> = None;
        let mut gt_index: Option<usize> = None; // year_nr is greater then  the year at this index
        let mut eq_index: Option<usize> = None; // only used if the year exists

        // see details in test

        for (index, year) in self.years.iter_mut().enumerate() {
            if year.year_nr == year_nr {
                eq_index = Some(index);
                break;
            } else if year_nr > year.year_nr {
                gt_index = Some(index);
            } else if (year_nr < year.year_nr) && (lt_index == None) {
                // checking for None is only needed for "less-then" because the years are ordered ascendingly
                lt_index = Some(index);
            }
        }

        match eq_index {
            // the year does exist, this is outside the for loop to satisfy the borrow checker
            Some(index) => match self.years.get_mut(index) {
                Some(y) => return y,
                None => panic!("thats not supposed to be possible"),
            },
            None => (),
        }

        // the year does not yet exist
        let insert_index: usize = {
            match (lt_index, gt_index) {
                (Some(0), None) => 0,                        // all years are greater than year_nr
                (Some(0..), Some(0..)) => lt_index.unwrap(), // year has to be somewhere in the middle
                (None, Some(0..)) => gt_index.unwrap() + 1,  // all years are smaller than year_nr
                _ => panic!("missed a case"),
            }
        };

        self.years.insert(insert_index, Year::default(year_nr));
        match self.years.get_mut(insert_index) {
            Some(y) => return y,
            None => panic!("thats not supposed to be possible"),
        };
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
    fn add_or_get_year() {
        // if 0
        // 1 2 4 5
        // lt_index = 0
        // gt_index = None
        //
        // if 3
        // 1 2 4 5
        // lt_index = 2
        // gt_index = 1
        //
        // if 6
        // 1 2 4 5
        // lt_index = None
        // gt_index = 3

        let yamlfile = YamlFile {
            version: 1,
            goal: 0.0,
            years: vec![Year::default(2025), Year::default(2031)],
        };

        let yamlfile_added_front_manual = YamlFile {
            version: 1,
            goal: 0.0,
            years: vec![Year::default(2018), Year::default(2025), Year::default(2031)],
        };
        let yamlfile_added_middle_manual = YamlFile {
            version: 1,
            goal: 0.0,
            years: vec![Year::default(2025), Year::default(2028), Year::default(2031)],
        };
        let yamlfile_added_end_manual = YamlFile {
            version: 1,
            goal: 0.0,
            years: vec![Year::default(2025), Year::default(2031), Year::default(2032)],
        };

        let mut yamlfile_added_front_fn = yamlfile.clone();
        yamlfile_added_front_fn.add_or_get_year(2018);

        let mut yamlfile_added_middle_fn = yamlfile.clone();
        yamlfile_added_middle_fn.add_or_get_year(2028);

        let mut yamlfile_added_end_fn = yamlfile.clone();
        yamlfile_added_end_fn.add_or_get_year(2032);

        assert_eq!(yamlfile_added_front_fn.years, yamlfile_added_front_manual.years);
        assert_ne!(yamlfile_added_front_fn.years, yamlfile.years);

        assert_eq!(yamlfile_added_middle_fn.years, yamlfile_added_middle_manual.years);
        assert_ne!(yamlfile_added_middle_fn.years, yamlfile.years);

        assert_eq!(yamlfile_added_end_fn.years, yamlfile_added_end_manual.years);
        assert_ne!(yamlfile_added_end_fn.years, yamlfile.years);
    }
}
