use crate::structs::Year;
use crate::Month;
use crate::YamlFile;

#[test]
fn month_compare() {
    const MONTH: u8 = 1;
    const YEAR: u16 = 2000;

    let mut ymlfile = YamlFile {
        version: 1,
        goal: 0.0,
        years: vec![Year {
            year_nr: YEAR,
            income: 0.0,
            expenses: 0.0,
            months: Month::default_months(),
        }],
    };

    match ymlfile.years.iter().position(|y| y.year_nr == YEAR) {
        Some(index) => match ymlfile.years.get_mut(index) {
            Some(ymlyear) => {
                let month = &mut ymlyear.months[MONTH as usize - 1];

                // I just created this test because I wasn't sure that this comparison is done correctly
                // other languages might have compared the datatype of both sides and would always say its the same
                assert!(*month == Month::default(month.month_nr));
                assert_ne!(*month, Month::default(month.month_nr + 1));
            }
            None => (),
        },
        None => (),
    }
}

#[test]
fn input_number_filter() {
    let mut s = String::from(" asdasd 339,59 €	").replace(",", ".");
    s.retain(|c| c == '.' || c.is_numeric() || c == ',');
    assert_eq!(s, "339.59");
}
