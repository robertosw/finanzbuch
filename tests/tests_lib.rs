use finance_yaml::accounting::accounting_month::AccountingMonth;
use finance_yaml::accounting::accounting_year::AccountingYear;
use finance_yaml::investing::depot_element::SavingsPlanSection;
use finance_yaml::investing::inv_year::InvestmentYear;
use finance_yaml::investing::{Investing, SavingsPlanInterval};
use finance_yaml::{accounting::Accounting, investing};
use std::collections::HashMap;
use std::path::PathBuf;

use finance_yaml::{DataFile, DepotElement};

#[test]
fn defaults_file_write_read_simple() {
    let datafile = DataFile::default();
    datafile.write(PathBuf::from("/tmp/defaults_file_write_read_simple.yaml"));
    drop(datafile);

    let datafile = DataFile::read(PathBuf::from("/tmp/defaults_file_write_read_simple.yaml"));

    assert_eq!(datafile.accounting, Accounting::default());
    assert_eq!(datafile.investing, Investing::default());
}

#[test]
fn defaults_file_write_read_all() {
    let mut datafile = DataFile::default();

    // ----- Fill all Accounting fields
    datafile.accounting.goal = 0.75;
    datafile.accounting.history.insert(
        2023,
        AccountingYear {
            year_nr: 2023,
            months: AccountingMonth::default_months(),
        },
    );

    // ----- Fill all Investing fields
    let mut history: HashMap<u16, InvestmentYear> = HashMap::new();
    history.insert(2023, InvestmentYear::default(2023));

    datafile.investing.add_depot_element(
        String::from("name 123"),
        DepotElement {
            variant: investing::InvestmentVariant::Bond,
            savings_plan: vec![SavingsPlanSection {
                start_month: 1,
                start_year: 2023,
                end_month: 12,
                end_year: 2023,
                amount: 50.0,
                interval: SavingsPlanInterval::Monthly,
            }],
            history,
        },
    );

    datafile.investing.add_comparison(5);

    // ----- Write and Read again to confirm parsing works as expected
    let control = datafile.clone();
    datafile.write(PathBuf::from("/tmp/defaults_file_write_read_all.yaml"));
    drop(datafile);

    let localfile = DataFile::read(PathBuf::from("/tmp/defaults_file_write_read_all.yaml"));
    assert_eq!(localfile, control);
}

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
