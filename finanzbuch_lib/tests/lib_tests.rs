use finanzbuch_lib::accounting::accounting_month::AccountingMonth;
use finanzbuch_lib::accounting::accounting_year::AccountingYear;
use finanzbuch_lib::accounting::recurrence::Recurrence;
use finanzbuch_lib::accounting::recurrence::RecurringInOut;
use finanzbuch_lib::accounting::Accounting;
use finanzbuch_lib::datafile::FILE_VERSION;
use finanzbuch_lib::investing::depot::Depot;
use finanzbuch_lib::investing::inv_variant::InvestmentVariant;
use finanzbuch_lib::investing::Investing;
use std::collections::BTreeMap;

use finanzbuch_lib::DataFile;
use finanzbuch_lib::DepotEntry;

#[cfg(test)]
mod read_write_datafile
{
    use finanzbuch_lib::accounting::accounting_year::AccountingYear;
    use finanzbuch_lib::accounting::recurrence::Recurrence;
    use finanzbuch_lib::accounting::recurrence::RecurringInOut;
    use finanzbuch_lib::fast_date::FastDate;
    use finanzbuch_lib::investing::depot::Depot;
    use finanzbuch_lib::investing::inv_months::InvestmentMonth;
    use finanzbuch_lib::investing::inv_variant::InvestmentVariant;
    use finanzbuch_lib::investing::inv_year::InvestmentYear;
    use finanzbuch_lib::investing::savings_plan_section::SavingsPlanSection;
    use finanzbuch_lib::investing::Investing;
    use finanzbuch_lib::investing::SavingsPlanInterval;
    use finanzbuch_lib::Accounting;
    use finanzbuch_lib::AccountingMonth;
    use finanzbuch_lib::DataFile;
    use finanzbuch_lib::DepotEntry;
    use std::collections::BTreeMap;
    use std::collections::HashMap;
    use std::path::PathBuf;
    use tinyrand::Wyrand;

    use tinyrand::Rand;
    use tinyrand::Seeded;
    use tinyrand::StdRand;
    use tinyrand_std::ClockSeed;

    fn _next_price(rand: &mut Wyrand, price: &mut f64) -> f64
    {
        print!("price start {price}\t");
        let nr: i16 = rand.next_lim_u16(u8::MAX as u16 * 2) as i16; // 0 .. 512
        let change: i16 = nr - (u8::MAX as i16); // -256 .. 256
        *price = *price * 1.05 + change as f64; // simulate stock changes
        *price = price.max(0.0); // price cannot be below 0
        *price = (*price * 100.0).round() / 100.0;
        print!("nr {nr}\tchange {change}\tprice new {price}\n");
        return *price;
    }

    fn _randomly_filled_investment_months() -> [InvestmentMonth; 12]
    {
        let seed = ClockSeed::default().next_u64();
        let mut rand = StdRand::seed(seed);
        let mut start_value = rand.next_u16() as f64 / 10.0;

        return std::array::from_fn(|i| {
            return InvestmentMonth::new(
                i as u8 + 1,
                123.45,
                _next_price(&mut rand, &mut start_value),
                rand.next_u16() as f64 / 1000.0,
            );
        });
    }

    fn _randomly_filled_accounting_months() -> [AccountingMonth; 12]
    {
        let seed = ClockSeed::default().next_u64();
        let mut rand = StdRand::seed(seed);

        return std::array::from_fn(|i| {
            return AccountingMonth::new(i as u8 + 1, rand.next_u16() as f64 / 11.11, rand.next_u16() as f64 / 11.11, String::new());
        });
    }

    #[test]
    fn file_parsing_defaults()
    {
        let datafile = DataFile::default_no_write_on_drop();
        datafile.write_to_custom_path(PathBuf::from("/tmp/file_parsing_defaults.yaml"));
        drop(datafile);

        let datafile = DataFile::read_from_custom_path(PathBuf::from("/tmp/file_parsing_defaults.yaml"));

        assert_eq!(datafile.accounting, Accounting::default());
        assert_eq!(datafile.investing, Investing::default());
    }

    #[test]
    fn file_parsing_rand()
    {
        // ----- Fill all fields
        let datafile = DataFile {
            accounting: Accounting {
                goal: 0.75,
                history: BTreeMap::from([(
                    2023,
                    AccountingYear {
                        year_nr: 2023,
                        months: _randomly_filled_accounting_months(),
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
                depot: Depot {
                    entries: HashMap::from([(
                        Depot::name_to_key("depot entry 1 name"),
                        DepotEntry::new(
                            InvestmentVariant::Bond,
                            String::from("depot entry 1 name"),
                            vec![SavingsPlanSection {
                                start: FastDate::new_risky(2023, 1, 1),
                                end: FastDate::new_risky(2023, 12, 1),
                                amount: 50.0,
                                interval: SavingsPlanInterval::Monthly,
                            }],
                            BTreeMap::from([(
                                2023,
                                InvestmentYear {
                                    year_nr: 2023,
                                    months: _randomly_filled_investment_months(),
                                },
                            )]),
                        ),
                    )]),
                },
            },
            write_on_drop: false,
            ..Default::default()
        };

        // ----- Write and Read again to confirm parsing works as expected
        let control = datafile.clone();
        datafile.write_to_custom_path(PathBuf::from("/tmp/file_parsing_rand.yaml"));
        drop(datafile);

        let localfile = DataFile::read_from_custom_path(PathBuf::from("/tmp/file_parsing_rand.yaml"));
        assert_eq!(localfile, control);
    }
}

#[cfg(test)]
mod sanitize_input_tests
{
    use finanzbuch_lib::SanitizeInput;

    #[test]
    fn float_in_string()
    {
        let neg_f_str = String::from("-9876,54321");
        let pos_f_str = String::from("+9876,54321");
        let other_f_str = String::from("9876,54321");

        assert_eq!(-9876.54321, SanitizeInput::string_to_f64(&neg_f_str, false).unwrap());
        assert_eq!(9876.54321, SanitizeInput::string_to_f64(&neg_f_str, true).unwrap());

        assert_eq!(9876.54321, SanitizeInput::string_to_f64(&pos_f_str, false).unwrap());
        assert_eq!(9876.54321, SanitizeInput::string_to_f64(&pos_f_str, true).unwrap());

        assert_eq!(9876.54321, SanitizeInput::string_to_f64(&other_f_str, true).unwrap());
    }

    #[test]
    fn float_to_float()
    {
        let f = -1235.019;

        assert_eq!(-1235.02, SanitizeInput::f64_to_monetary_f64(f));
        assert_eq!(1235.02, SanitizeInput::f64_to_monetary_f64_abs(f));
    }
}

#[cfg(test)]
mod depot_entry
{
    use std::collections::BTreeMap;

    use finanzbuch_lib::fast_date::FastDate;
    use finanzbuch_lib::investing::inv_variant::InvestmentVariant;
    use finanzbuch_lib::investing::savings_plan_section::SavingsPlanSection;
    use finanzbuch_lib::investing::SavingsPlanInterval;
    use finanzbuch_lib::DepotEntry;

    #[test]
    fn add_savings_plan_section_same()
    {
        let mut de = prepare_tests();

        let result = de.add_savings_plan_section(SavingsPlanSection {
            start: FastDate::new_risky(2023, 1, 1),
            end: FastDate::new_risky(2023, 12, 31),
            amount: 10.0,
            interval: SavingsPlanInterval::Monthly,
        });

        assert_eq!(result.is_err(), true);
    }
    #[test]
    fn add_savings_plan_section_overlap_start()
    {
        let mut de = prepare_tests();

        let result = de.add_savings_plan_section(SavingsPlanSection {
            start: FastDate::new_risky(2022, 6, 1),
            end: FastDate::new_risky(2023, 1, 1),
            amount: 10.0,
            interval: SavingsPlanInterval::Monthly,
        });

        assert_eq!(result.is_err(), true);
    }
    #[test]
    fn add_savings_plan_section_overlap_end()
    {
        let mut de = prepare_tests();

        let result = de.add_savings_plan_section(SavingsPlanSection {
            start: FastDate::new_risky(2023, 6, 1),
            end: FastDate::new_risky(2023, 12, 31),
            amount: 10.0,
            interval: SavingsPlanInterval::Monthly,
        });

        assert_eq!(result.is_err(), true);
    }
    #[test]
    fn add_savings_plan_section_overlap_middle()
    {
        let mut de = prepare_tests();

        let result = de.add_savings_plan_section(SavingsPlanSection {
            start: FastDate::new_risky(2023, 2, 2),
            end: FastDate::new_risky(2023, 11, 11),
            amount: 10.0,
            interval: SavingsPlanInterval::Monthly,
        });

        assert_eq!(result.is_err(), true);
    }
    #[test]
    fn add_savings_plan_section_ends_before()
    {
        let mut de = prepare_tests();

        let result = de.add_savings_plan_section(SavingsPlanSection {
            start: FastDate::new_risky(2022, 6, 1),
            end: FastDate::new_risky(2022, 12, 31),
            amount: 10.0,
            interval: SavingsPlanInterval::Monthly,
        });

        assert_eq!(result.is_ok(), true);
    }
    #[test]
    fn add_savings_plan_section_starts_after()
    {
        let mut de = prepare_tests();

        let result = de.add_savings_plan_section(SavingsPlanSection {
            start: FastDate::new_risky(2024, 1, 1),
            end: FastDate::new_risky(2024, 12, 31),
            amount: 10.0,
            interval: SavingsPlanInterval::Monthly,
        });

        assert_eq!(result.is_ok(), true);
    }

    fn prepare_tests() -> DepotEntry
    {
        let savings_plan = vec![SavingsPlanSection {
            start: FastDate::new_risky(2023, 1, 1),
            end: FastDate::new_risky(2023, 12, 31),
            amount: 10.0,
            interval: SavingsPlanInterval::Monthly,
        }];
        DepotEntry::new(InvestmentVariant::Etf, String::from("name"), savings_plan, BTreeMap::new())
    }
}

#[test]
fn hash_test()
{
    // the hashing algorithm has to be deterministic (same result across multiple program restarts)
    // but this cannot be tested automatically

    // create entry by name, create hash of name, get entry by hash and by name, should all be the same
    const NAME: &str = "Depot Test name 123 &#+.-";
    let depot_entry = DepotEntry::default(NAME, InvestmentVariant::Etf);
    let hash = Depot::name_to_key(NAME);

    let mut datafile: DataFile = DataFile::default_no_write_on_drop();
    datafile.investing.depot.add_entry(NAME, depot_entry.clone());

    assert!(datafile.investing.depot.entries.contains_key(&hash));

    let entry_from_name: Option<&DepotEntry> = datafile.investing.depot.get_entry_from_str(NAME);
    assert_ne!(entry_from_name, None);
    assert_eq!(NAME, entry_from_name.unwrap().name());
    assert_eq!(entry_from_name.unwrap(), &depot_entry);

    let entry_from_hash: Option<&DepotEntry> = datafile.investing.depot.entries.get(&hash);
    assert_ne!(entry_from_hash, None);
    assert_eq!(NAME, entry_from_hash.unwrap().name());
    assert_eq!(entry_from_hash.unwrap(), &depot_entry);
}

#[test]
fn month_compare()
{
    const MONTH: u8 = 1;
    const YEAR: u16 = 2000;

    let mut datafile = DataFile {
        version: FILE_VERSION,
        accounting: Accounting {
            history: BTreeMap::from([(YEAR, AccountingYear::default(YEAR))]),
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
        write_on_drop: false,
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

#[cfg(test)]
mod get_oldest_year_in_depo
{
    use finanzbuch_lib::investing::depot::Depot;
    use finanzbuch_lib::investing::inv_variant::InvestmentVariant;
    use finanzbuch_lib::investing::inv_year::InvestmentYear;
    use finanzbuch_lib::DataFile;
    use finanzbuch_lib::DepotEntry;
    use std::collections::BTreeMap;

    #[test]
    fn test_get_oldest_year_in_depot_with_entries_and_history()
    {
        let mut datafile = DataFile::default_no_write_on_drop();

        for i in 1..4 {
            let mut history = BTreeMap::new();
            for year in (2000 + i)..(2004 + i) {
                history.insert(year, InvestmentYear::default(year));
            }

            let name = format!("Depot {}", i);
            datafile.investing.depot.entries.insert(
                Depot::name_to_key(name.as_str()),
                DepotEntry::new(InvestmentVariant::Stock, name, vec![], history),
            );
        }

        assert_eq!(datafile.investing.depot.get_oldest_year(), Some(2001));
    }

    #[test]
    fn test_get_oldest_year_in_depot_with_entries_no_history()
    {
        let mut datafile = DataFile::default_no_write_on_drop();

        for i in 1..4 {
            let name = format!("Depot {}", i);
            datafile.investing.depot.entries.insert(
                Depot::name_to_key(name.as_str()),
                DepotEntry::new(InvestmentVariant::Stock, name, vec![], BTreeMap::new()),
            );
        }

        assert_eq!(datafile.investing.depot.get_oldest_year(), None);
    }

    #[test]
    fn test_get_oldest_year_in_depot_no_entries()
    {
        let datafile = DataFile::default_no_write_on_drop();

        assert_eq!(datafile.investing.depot.get_oldest_year(), None);
    }
}
