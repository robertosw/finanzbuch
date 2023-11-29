use finanzbuch_lib::accounting::accounting_month::AccountingMonth;
use finanzbuch_lib::accounting::accounting_year::AccountingYear;
use finanzbuch_lib::accounting::recurrence::Recurrence;
use finanzbuch_lib::accounting::recurrence::RecurringInOut;
use finanzbuch_lib::accounting::Accounting;
use finanzbuch_lib::datafile::FILE_VERSION;
use finanzbuch_lib::investing::inv_months::InvestmentMonth;
use finanzbuch_lib::investing::inv_variant::InvestmentVariant;
use finanzbuch_lib::investing::inv_year::InvestmentYear;
use finanzbuch_lib::investing::savings_plan_section::SavingsPlanSection;
use finanzbuch_lib::investing::Investing;
use finanzbuch_lib::investing::SavingsPlanInterval;
use finanzbuch_lib::FastDate;
use std::collections::HashMap;
use std::path::PathBuf;

use finanzbuch_lib::DataFile;
use finanzbuch_lib::DepotEntry;

use tinyrand::Rand;
use tinyrand::Seeded;
use tinyrand::StdRand;
use tinyrand_std::ClockSeed;

fn randomly_filled_investment_months() -> [InvestmentMonth; 12]
{
    let seed = ClockSeed::default().next_u64();
    let mut rand = StdRand::seed(seed);

    return std::array::from_fn(|i| {
        return InvestmentMonth {
            month_nr: i as u8 + 1,
            amount: rand.next_u16() as f64 / 111.11,
            price_per_unit: rand.next_u16() as f64 / 11.11,
            additional_transactions: rand.next_u16() as f64 / 1111.11,
        };
    });
}

fn randomly_filled_accounting_months() -> [AccountingMonth; 12]
{
    let seed = ClockSeed::default().next_u64();
    let mut rand = StdRand::seed(seed);

    return std::array::from_fn(|i| {
        return AccountingMonth::new(i as u8 + 1, rand.next_u16() as f64 / 11.11, rand.next_u16() as f64 / 11.11, String::new());
    });
}

#[test]
fn hash_test()
{
    // the hashing algorithm has to be deterministic (same result across multiple program restarts)
    // but this cannot be tested automatically
    
    // create entry by name, create hash of name, get entry by hash and by name, should all be the same
    const NAME: &str = "Depot Test name 123 &#+.-";
    let depot_entry = DepotEntry::default(NAME, InvestmentVariant::Etf);
    let hash = Investing::name_to_key(NAME);

    let mut datafile: DataFile = DataFile::default();
    datafile.investing.add_depot_entry(NAME, depot_entry.clone());

    assert!(datafile.investing.depot.contains_key(&hash));

    let entry_from_name: Option<&DepotEntry> = datafile.investing.get_depot_entry(NAME);
    assert_ne!(entry_from_name, None);
    assert_eq!(NAME, entry_from_name.unwrap().name());
    assert_eq!(entry_from_name.unwrap(), &depot_entry);
    
    let entry_from_hash: Option<&DepotEntry> = datafile.investing.depot.get(&hash);
    assert_ne!(entry_from_hash, None);
    assert_eq!(NAME, entry_from_hash.unwrap().name());
    assert_eq!(entry_from_hash.unwrap(), &depot_entry);
}


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
                    months: randomly_filled_accounting_months(),
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
                Investing::name_to_key("depot entry 1 name"),
                DepotEntry::new(
                    InvestmentVariant::Bond,
                    String::from("depot entry 1 name"),
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
                            months: randomly_filled_investment_months(),
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
        version: FILE_VERSION,
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
