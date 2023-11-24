use finanzbuch_lib::accounting::accounting_month::AccountingMonth;
use finanzbuch_lib::accounting::accounting_year::AccountingYear;
use finanzbuch_lib::accounting::recurrence::Recurrence;
use finanzbuch_lib::accounting::recurrence::RecurringInOut;
use finanzbuch_lib::accounting::Accounting;
use finanzbuch_lib::investing::inv_variant::InvestmentVariant;
use finanzbuch_lib::investing::inv_year::InvestmentYear;
use finanzbuch_lib::investing::savings_plan_section::SavingsPlanSection;
use finanzbuch_lib::investing::Investing;
use finanzbuch_lib::investing::SavingsPlanInterval;
use finanzbuch_lib::FastDate;
use std::collections::HashMap;
use std::path::PathBuf;

use finanzbuch_lib::DataFile;
use finanzbuch_lib::DepotElement;

#[test]
fn defaults_file_write_read_simple()
{
    let datafile = DataFile::default();
    datafile.write_to_custom_path(PathBuf::from("/tmp/defaults_file_write_read_simple.yaml"));
    drop(datafile);

    let datafile = DataFile::read_from_custom_path(PathBuf::from("/tmp/defaults_file_write_read_simple.yaml"));

    assert_eq!(datafile.accounting, Accounting::default());
    assert_eq!(datafile.investing, Investing::default());
}

#[test]
fn defaults_file_write_read_all()
{
    // ----- Fill all fields
    let datafile = DataFile {
        accounting: Accounting {
            goal: 0.75,
            history: HashMap::from([(
                2023,
                AccountingYear {
                    year_nr: 2023,
                    months: AccountingMonth::randomly_filled_months(),
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
                DepotElement::new(
                    InvestmentVariant::Bond,
                    vec![SavingsPlanSection {
                        start: FastDate::new_risky(2023, 1, 1),
                        end: FastDate::new_risky(2023, 12, 1),
                        amount: 50.0,
                        interval: SavingsPlanInterval::Monthly,
                    }],
                    HashMap::from([(
                        2023,
                        InvestmentYear {
                            year_nr: 2023,
                            months: InvestmentYear::randomly_filled_months(),
                        },
                    )]),
                ),
            )]),
        },
        ..Default::default()
    };

    // ----- Write and Read again to confirm parsing works as expected
    let control = datafile.clone();
    datafile.write_to_custom_path(PathBuf::from("/tmp/defaults_file_write_read_all.yaml"));
    drop(datafile);

    let localfile = DataFile::read_from_custom_path(PathBuf::from("/tmp/defaults_file_write_read_all.yaml"));
    assert_eq!(localfile, control);
}

#[test]
fn month_compare()
{
    const MONTH: u8 = 1;
    const YEAR: u16 = 2000;

    let mut datafile = DataFile {
        version: 2,
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
fn input_number_filter()
{
    let mut s = String::from(" asdasd 339,59 €	").replace(",", ".");
    s.retain(|c| c == '.' || c.is_numeric() || c == ',');
    assert_eq!(s, "339.59");
}

#[cfg(test)]
mod fast_date_tests
{
    use finanzbuch_lib::FastDate;

    /// - 9.7.2023 is 190th day of the year:
    /// - 31 + 28 + 31 + 30 + 31 + 30 + 9
    /// - ceil(190 / 7) = 28
    #[test]
    fn new() { assert_eq!(FastDate::new(2023, 7, 9).unwrap().date(), (2023, 7, 9, 28)) }

    #[test]
    fn new_invalid_month() { assert!(FastDate::new(2023, 13, 23).is_err()) }

    #[test]
    fn new_invalid_day() { assert!(FastDate::new(2023, 11, 32).is_err()) }

    #[test]
    fn max_values() { assert_eq!(FastDate::new(2023, 12, 31).unwrap().date(), (2023, 12, 31, 53)) }

    #[test]
    fn min_values() { assert_eq!(FastDate::new(2023, 1, 1).unwrap().date(), (2023, 1, 1, 1)) }

    #[test]
    fn set_year()
    {
        let mut date = FastDate::new(2023, 11, 23).unwrap();
        date.set_year(2024);
        assert_eq!(date.year(), 2024);
    }

    #[test]
    fn set_month()
    {
        let mut date = FastDate::new(2023, 11, 23).unwrap();
        assert!(date.set_month(13).is_err());
        assert!(date.set_month(0).is_err());
        assert!(date.set_month(12).is_ok());
    }

    #[test]
    fn set_day()
    {
        let mut date = FastDate::new(2023, 11, 23).unwrap();
        assert!(date.set_day(32).is_err());
        assert!(date.set_day(0).is_err());
        assert!(date.set_day(31).is_ok());
    }

    #[test]
    fn comparison_smaller_larger()
    {
        let past = FastDate::new(2000, 1, 1).unwrap();
        let future = FastDate::new(2000, 1, 2).unwrap();
        assert!(past < future);
        assert!(future > past);
    }

    #[test]
    fn comparison_eq()
    {
        let now = FastDate::new(2000, 11, 23).unwrap();
        assert!(now == now);
    }

    #[test]
    fn comparison_smaller_eq()
    {
        let past = FastDate::new(2000, 1, 1).unwrap();
        let future = FastDate::new(2000, 1, 2).unwrap();
        assert!(past < future);
        assert!(past <= past);
    }

    #[test]
    fn bit_mask_negate()
    {
        assert_eq!(0b0000_0000 as u8, !0b1111_1111 as u8);
        assert_ne!(0b1111_1111, !0b0000_0000);
        assert_ne!(0b1111 as u8, !0b0000 as u8);
    }
}
