pub mod config;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Year {
    pub year_nr: u16,
    pub income_sum: f64,
    pub expenses_sum: f64,
    pub months: [Month; 12],
}
impl Year {
    pub fn default(year_nr: u16) -> Self {
        return Self {
            year_nr,
            income_sum: 0.0,
            expenses_sum: 0.0,
            months: Month::default_months(),
        };
    }

    /// - If the month (specified by `new_month.month_nr`) contains only default values, these will be overwritten without a note.
    /// - If the month contains values other than defaults, these will also be overwritten without confirmation, but the old values will be printed into the terminal
    pub fn insert_or_overwrite_month(&mut self, new_month: Month) {
        let month_nr = new_month.month_nr;
        let month: &mut Month = &mut self.months[month_nr as usize - 1];

        if *month != Month::default(month.month_nr) {
            // ("{:0>2?}")
            //       2 - width
            //      > -- where to align actual value, > means {fill}{value}, < means {value}{fill}
            //     0 --- with what to fill
            println!("{:0>2?}.{:4?} will be overwritten!", month.month_nr, self.year_nr);
            println!("Old content: {:?}", *month);

            // reset this month to default = subtract from year sum
            self.income_sum -= month.income;
            self.expenses_sum -= month.expenses;
            *month = Month::default(month.month_nr);
        }

        // write given values into month and add to year sum
        month.income = new_month.income;
        month.expenses = new_month.expenses;
        month.difference = new_month.difference;
        month.percentage = new_month.percentage;
        self.income_sum += new_month.income;
        self.expenses_sum += new_month.expenses;
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
        return Self {
            month_nr: month,
            income: 0.0,
            expenses: 0.0,
            difference: 0.0,
            percentage: 0.0,
        };
    }

    pub fn default_months() -> [Self; 12] {
        return [
            Self::default(1),
            Self::default(2),
            Self::default(3),
            Self::default(4),
            Self::default(5),
            Self::default(6),
            Self::default(7),
            Self::default(8),
            Self::default(9),
            Self::default(10),
            Self::default(11),
            Self::default(12),
        ];
    }
}
