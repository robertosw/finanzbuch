use finance_yaml::accounting::Accounting;
use finance_yaml::accounting::accounting_month::AccountingMonth;
use finance_yaml::accounting::accounting_year::AccountingYear;
use finance_yaml::investing::Investing;
use std::collections::HashMap;

use finance_yaml::DataFile;

#[test]
fn month_compare() {
    const MONTH: u8 = 1;
    const YEAR: u16 = 2000;

    let mut datafile = DataFile {
        version: 2,
        accounting: Accounting {
            history: HashMap::from([(YEAR, AccountingYear::default(YEAR))]),
            goal: 1.0,
        },
        investing: Investing::default(),
    };

    let year = match datafile.accounting.history.get_mut(&YEAR) {
        Some(v) => v,
        None => panic!("Year that was just created, could not be found in HashMap"),
    };

    let month = &mut year.months[MONTH as usize - 1];

    // I just created this test because I wasn't sure that this comparison is done correctly
    // other languages might have compared the datatype of both sides and would always say its the same
    assert!(*month == AccountingMonth::default(month.month_nr));
    assert_ne!(*month, AccountingMonth::default(month.month_nr + 1));
}

#[test]
fn input_number_filter() {
    let mut s = String::from(" asdasd 339,59 €	").replace(",", ".");
    s.retain(|c| c == '.' || c.is_numeric() || c == ',');
    assert_eq!(s, "339.59");
}
