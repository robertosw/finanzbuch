mod structs;
mod yaml_file;
mod year;

use crate::structs::Month;
use crate::structs::YamlFile;
use crate::structs::Year;
use tinyrand::Rand;
use tinyrand::RandRange;
use tinyrand::Seeded;
use tinyrand::StdRand;
use tinyrand_std::ClockSeed;

fn main() {
    let (input_income, input_expenses, input_month_nr, input_year_nr): (f64, f64, u8, u16) = generate_random_input();
    println!("in {}, out {}, month {}, year {}", input_income, input_expenses, input_month_nr, input_year_nr);

    let calc_difference: f64 = input_income - input_expenses;
    let calc_percentage: f64 = input_expenses / input_income;
    println!("diff {}, perc {}", calc_difference, calc_percentage);

    // read file and sort ascending
    let mut ymlfile = YamlFile::read();

    // check if the targeted year already exists
    match ymlfile.years.iter().position(|e: &Year| e.year_nr == input_year_nr) {
        // the given year exists in ymlfile.years
        Some(index) => match ymlfile.years.get_mut(index) {
            Some(ymlyear) => ymlyear.insert_or_override_month(Month {
                month_nr: input_month_nr,
                income: input_income,
                expenses: input_expenses,
                difference: calc_difference,
                percentage: calc_percentage,
            }),
            None => panic!("This case should never happen"),
        },
        // the given year does not exist in ymlfile.years
        None => ymlfile.add_year_with_month(
            input_year_nr,
            Month {
                month_nr: input_month_nr,
                income: input_income,
                expenses: input_expenses,
                difference: calc_difference,
                percentage: calc_percentage,
            },
        ),
    }

    // beim einfügen in ein Jahr und Monat überprüfen ob in dem Monat schon Werte waren
    // Wenn nicht, zur Jahres summe einfach die Monatswerte aufaddieren
    // Wenn Monat überschrieben, dann erst Differenz zu vorherigen Werten berechnen, überschreiben und im Jahr aufaddieren
    ymlfile.write();
}

/// return values
/// - income, expenses, month, year
fn generate_random_input() -> (f64, f64, u8, u16) {
    let seed = ClockSeed::default().next_u64();
    let mut rand = StdRand::seed(seed);
    let rand_month: u8 = rand.next_range(1 as usize..13 as usize) as u8;
    let rand_year: u16 = rand.next_range(2000 as usize..2024 as usize) as u16;
    let rand_income: f64 = rand.next_u16() as f64 / 11.11;
    let rand_expenses: f64 = rand.next_u16() as f64 / 11.11;
    return (rand_income, rand_expenses, rand_month, rand_year);
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
