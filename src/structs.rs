use serde::Deserialize;
use serde::Serialize;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;

extern crate dirs;

const FILENAME: &'static str = "finance-data.yaml";

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct YamlFile {
    pub version: u8,
    pub goal: f64,
    pub years: Vec<Year>,
}
impl YamlFile {
    pub fn default() -> Self {
        return YamlFile {
            version: 1,
            goal: 1.0,
            years: vec![],
        };
    }

    /// 1. Checks if the yaml file already exists in the users home directory, creates it with `YamlFile::default` values if not.
    /// 2. If it does exist, reads and parses it into a `YamlFile`
    /// 3. Returns the data, with the `years` sorted ascending
    pub fn read() -> YamlFile {
        let filepath = dirs::home_dir()
            .expect("It was expected that this user has a home directory. This was not the case. This program does not work without a valid home directory.")
            .join(FILENAME);

        // check if file exists, create with template if not
        match filepath.exists() {
            false => match OpenOptions::new().create_new(true).write(true).open(&filepath) {
                Ok(mut file) => match file.write_all(" ".as_bytes()) {
                    Ok(_) => return YamlFile::default(),
                    Err(e) => panic!("error writing to yaml file > {:?}", e),
                },
                Err(e) => panic!("error creating yaml file > {:?}", e),
            },
            true => (),
        };

        let mut file = match OpenOptions::new().create(false).read(true).open(&filepath) {
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

    /// 1. Parses the existing `YamlFile` into a `String`
    /// 2. Writes this `String` into the file on disk
    pub fn write(&self) {
        let filepath = dirs::home_dir()
            .expect("It was expected that this user has a home directory. This was not the case. This program does not work without a valid home directory.")
            .join(FILENAME);
        println!("writing into {:?}", filepath);

        let yaml = match serde_yaml::to_string(self) {
            Ok(v) => v,
            Err(e) => panic!("error at serde_yaml::to_string > {:?}", e),
        };

        let mut file = match OpenOptions::new().create(true).truncate(true).write(true).open(filepath) {
            Ok(file) => file,
            Err(e) => panic!("error at opening yaml file > {:?}", e),
        };

        match file.write_all(yaml.as_bytes()) {
            Ok(_) => (),
            Err(e) => panic!("error at writing yaml file > {:?}", e),
        };
    }

    /// - if the year does not already exist, adds it to `YamlFile.years` with default values
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
                (None, None) => 0,                           // no years yet exist
                _ => panic!("missed a case while checking where to insert year"),
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

    /// - If the month (specified by `new_month.month_nr`) contains only default values, these will be overwritten without a note.
    /// - If the month contains values other than defaults, these will also be overwritten without confirmation, but the old values will be printed into the terminal
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
