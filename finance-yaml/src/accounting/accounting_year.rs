use crate::accounting::accounting_month::AccountingMonth;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct AccountingYear {
    pub year_nr: u16, // This value is needed, so that self knows which year it is (has no access to the HashMap Key)
    pub months: [AccountingMonth; 12],
}
impl AccountingYear {
    pub fn default(year_nr: u16) -> Self {
        return Self {
            year_nr,
            months: AccountingMonth::default_months(),
        };
    }

    /// - If the month (specified by `new_month.month_nr`) contains only default values, these will be overwritten without a note.
    /// - If the month contains values other than defaults, these will also be overwritten without confirmation, but the old values will be printed into the terminal
    pub fn insert_or_overwrite_month(&mut self, new_month: AccountingMonth) {
        let month_nr = new_month.month_nr();
        let month: &mut AccountingMonth = &mut self.months[month_nr as usize - 1];

        if *month != AccountingMonth::default(month.month_nr()) {
            // ("{:0>2?}")
            //       2 - width
            //      > -- where to align actual value, > means {fill}{value}, < means {value}{fill}
            //     0 --- with what to fill
            println!("{:0>2?}.{:4?} will be overwritten!", month.month_nr(), self.year_nr);
            println!("Old content: {:?}", *month);
        }

        *month = new_month;
    }

    pub fn get_difference(&self) -> f64 {
        self.get_sum_income() - self.get_sum_expenses()
    }

    pub fn get_percentage1(&self) -> f64 {
        self.get_sum_expenses() / self.get_sum_income()
    }

    pub fn get_percentage100(&self) -> u16 {
        (self.get_percentage1() * 100.0) as u16
    }

    pub fn get_sum_income(&self) -> f64 {
        let mut sum: f64 = 0.0;
        self.months.iter().for_each(|i| sum += i.income());
        return sum;
    }

    pub fn get_sum_expenses(&self) -> f64 {
        let mut sum: f64 = 0.0;
        self.months.iter().for_each(|i| sum += i.expenses());
        return sum;
    }

    /// will return Err(()) if there is no data to calculate a median of
    pub fn get_median_income(&self) -> Result<f64, ()> {
        let incomes: Vec<f64> = self.months.iter().map(|m| m.income()).collect();
        Self::_get_median(&incomes)
    }

    /// will return Err(()) if there is no data to calculate a median of
    pub fn get_median_expenses(&self) -> Result<f64, ()> {
        let expenses: Vec<f64> = self.months.iter().map(|m| m.expenses()).collect();
        Self::_get_median(&expenses)
    }

    /// will return Err(()) if there is no data to calculate a median of
    pub fn get_median_difference(&self) -> Result<f64, ()> {
        let diffs: Vec<f64> = self.months.iter().map(|m| m.difference()).collect();
        Self::_get_median(&diffs)
    }

    // TODO use percentage_100
    /// will return Err(()) if there is no data to calculate a median of
    pub fn get_median_percentage(&self) -> Result<f64, ()> {
        let percentages: Vec<f64> = self.months.iter().map(|m| m.percentage_1()).collect();
        Self::_get_median(&percentages)
    }

    /// will return Err(()) if there is no data to calculate a median of
    fn _get_median(vec_f64: &Vec<f64>) -> Result<f64, ()> {
        let mut vec: Vec<f64> = vec_f64.iter().filter(|&m| m > &0.0).map(|v| v.to_owned()).collect(); // remove all 0's

        let len = vec.len();
        match len {
            0 => return Err(()),
            1 => return Ok(vec.get(0).unwrap().to_owned()),
            _ => (),
        }

        vec.sort_by(|a, b| a.total_cmp(b));

        // if even, middle is between two elements, return avg of these two
        // if odd, element in the middle is median
        match len % 2 {
            0 => {
                let before_mid = vec.get(len / 2 - 1).unwrap();
                let after_mid = vec.get(len / 2).unwrap();
                return Ok((before_mid + after_mid) / 2.0);
            }
            _ => Ok(vec.get(len / 2).unwrap().to_owned()), // if length is odd, element in middle is median
        }
    }
}
