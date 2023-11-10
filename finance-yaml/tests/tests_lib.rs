use finance_yaml::investing::inv_variant::InvestmentVariant;
use finance_yaml::investing::{inv_year::InvestmentYear, Investing, SavingsPlanInterval};
use finance_yaml::{
    accounting::{
        accounting_month::AccountingMonth,
        accounting_year::AccountingYear,
        recurrence::{Recurrence, RecurringInOut},
        Accounting,
    },
    investing::savings_plan_section::SavingsPlanSection,
};
use std::collections::HashMap;
use std::path::PathBuf;

use finance_yaml::DataFile;
use finance_yaml::DepotElement;

#[test]
fn defaults_file_write_read_simple() {
    let datafile = DataFile::default_with_path(PathBuf::from("/tmp/defaults_file_write_read_simple.yaml"));
    datafile.write();
    drop(datafile);

    let datafile = DataFile::read(PathBuf::from("/tmp/defaults_file_write_read_simple.yaml"));

    assert_eq!(datafile.accounting, Accounting::default());
    assert_eq!(datafile.investing, Investing::default());
}

#[test]
fn defaults_file_write_read_all() {
    let path: PathBuf = PathBuf::from("/tmp/defaults_file_write_read_all.yaml");

    // ----- Fill all fields
    let datafile = DataFile {
        version: 2,
        path,
        accounting: Accounting {
            goal: 0.75,
            history: HashMap::from([(
                2023,
                AccountingYear {
                    year_nr: 2023,
                    months: AccountingMonth::default_months(),
                },
            )]),
            recurring_income: vec![RecurringInOut {
                name: String::from("name for recurring income"),
                quantity: 5.0,
                recurrence: Recurrence::Week,
                interval: 1,
                frequency: 5,
            }],
            recurring_expenses: vec![RecurringInOut {
                name: String::from("name for recurring expenses"),
                quantity: 15.0,
                recurrence: Recurrence::Week,
                interval: 3,
                frequency: 1,
            }],
        },
        investing: Investing {
            comparisons: vec![5, 8],
            depot: HashMap::from([(
                String::from("depot entry 1 name"),
                DepotElement {
                    variant: InvestmentVariant::Bond,
                    savings_plan: vec![SavingsPlanSection {
                        start_month: 1,
                        start_year: 2023,
                        end_month: 12,
                        end_year: 2023,
                        amount: 50.0,
                        interval: SavingsPlanInterval::Monthly,
                    }],
                    history: HashMap::from([(
                        2023,
                        InvestmentYear {
                            year_nr: 2023,
                            months: InvestmentYear::default_months(),
                        },
                    )]),
                },
            )]),
        },
    };

    // ----- Write and Read again to confirm parsing works as expected
    let control = datafile.clone();
    datafile.write();
    drop(datafile);

    let localfile = DataFile::read(PathBuf::from("/tmp/defaults_file_write_read_all.yaml"));
    assert_eq!(localfile, control);
}

#[test]
fn month_compare() {
    const MONTH: u8 = 1;
    const YEAR: u16 = 2000;
    let path: PathBuf = PathBuf::from("/tmp/defaults_file_write_read_all.yaml");

    let mut datafile = DataFile {
        version: 2,
        path,
        accounting: Accounting {
            history: HashMap::from([(YEAR, AccountingYear::default(YEAR))]),
            goal: 1.0,
            recurring_income: vec![RecurringInOut {
                name: String::from("name for recurring income"),
                quantity: 5.0,
                recurrence: Recurrence::Week,
                interval: 1,
                frequency: 5,
            }],
            recurring_expenses: vec![RecurringInOut {
                name: String::from("name for recurring income"),
                quantity: 15.0,
                recurrence: Recurrence::Week,
                interval: 3,
                frequency: 1,
            }],
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
    assert!(*month == AccountingMonth::default(month.month_nr()));
    assert_ne!(*month, AccountingMonth::default(month.month_nr() + 1));
}

#[test]
fn input_number_filter() {
    let mut s = String::from(" asdasd 339,59 €	").replace(",", ".");
    s.retain(|c| c == '.' || c.is_numeric() || c == ',');
    assert_eq!(s, "339.59");
}
