use crate::fast_date::FastDate;
use crate::CurrentDate;

use super::inv_variant::InvestmentVariant;
use super::inv_year::InvestmentYear;
use super::savings_plan_section::SavingsPlanSection;
use super::SavingsPlanInterval;
use core::panic;
use fxhash::FxHasher;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::hash::Hasher;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Depot
{
    // This key has to be something that can be used in an `id=""` in html
    /// Key is the hash of the name of the `DepotEntry`
    pub entries: HashMap<u64, DepotEntry>,
}
impl Depot
{
    pub fn new() -> Self { return Self { entries: HashMap::new() }; }

    pub fn get_entry_from_str(&self, name: &str) -> Option<&DepotEntry> { self.entries.get(&Self::name_to_key(name)) }
    pub fn get_entry_mut_from_str(&mut self, name: &str) -> Option<&mut DepotEntry> { self.entries.get_mut(&Self::name_to_key(name)) }
    pub fn add_entry(&mut self, name: &str, depot_entry: DepotEntry) { self.entries.insert(Self::name_to_key(name), depot_entry); }

    pub fn name_to_key(name: &str) -> u64
    {
        let mut hasher = FxHasher::default();
        hasher.write(name.as_bytes());
        return hasher.finish();
    }

    /// Ensures that all histories of all depot entries have the same years.
    /// Not the same content in each year, just that the same years exist.
    ///
    /// If some year did not exist in a `DepotEntry`, it will be created with default values
    ///
    /// If all DepotEntries have no history, the current_year will be added to all of them
    ///
    /// Since this modifies all DepotEntries, be sure to write to file, to keep this modification
    pub fn ensure_uniform_histories(&mut self)
    {
        let oldest_year: u16 = match self.get_oldest_year() {
            Some(y) => y,
            None => CurrentDate::current_year(), // No entry has any history, so the current_year will be added
        };

        for de in self.entries.values_mut() {
            // check if every year from oldest_year - current_year exists
            for year in oldest_year..CurrentDate::current_year() + 1 {
                if de.history.contains_key(&year) == false {
                    de.history.insert(year, InvestmentYear::default(year));
                }
            }
        }
    }

    /// returns
    /// - start (oldest) date: `FastDate` with `Day = 1`
    /// - end date = current date: `FastDate` with `Day = 1`
    /// - month count: `usize`
    ///
    /// How many months, up until the current month, are in this depot?
    /// Includes the current month.
    /// If two entries have the same month, this does not count that month twice
    pub fn get_oldest_year_and_total_month_count(&self) -> Option<(FastDate, FastDate, usize)>
    {
        let oldest_year = match self.get_oldest_year() {
            Some(y) => y as usize,
            None => return None, // All depot entries have no history so there is no data
        };
        let current_year = CurrentDate::current_year() as usize;

        // only until the current month
        let month_count = {
            // How many months would there be, if all years could have values for all 12 months already
            let x = (current_year + 1 - oldest_year) * 12;

            // and than subtract the amount of months which have not yet passed in this year
            x - (12 - CurrentDate::current_month() as usize)
        };

        let start = FastDate::new_risky(oldest_year as u16, 1, 1);
        let end = FastDate::new_risky(CurrentDate::current_year(), CurrentDate::current_month(), 1);
        return Some((start, end, month_count));
    }

    /// Across all DepotEntries, get the oldest year
    pub fn get_oldest_year(&self) -> Option<u16>
    {
        // go through all DepotEntries and see what the oldest month and year with values are
        let oldest_year: u16 = self.entries.values().fold(u16::MAX, |accumulator_oldest_year: u16, de: &DepotEntry| {
            // Go through all depot entries and note the oldest year in the accumulator
            // The start value is just there to have a value in the variable, if there are no depot entries
            match de.history.first_key_value() {
                Some((year, _)) => accumulator_oldest_year.min(*year),
                None => accumulator_oldest_year,
            }
        });

        if oldest_year == u16::MAX {
            return None;
        }

        return Some(oldest_year);
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct DepotEntry
{
    // dont allow the name to be changed, because the key in the HashMap gets generated from this name
    // if this data is given out, then the name changes, then this element cannot be found anymore, because the hash didnt change
    name: String,
    pub variant: InvestmentVariant,
    savings_plan: Vec<SavingsPlanSection>, // this has to be sorted after every modification

    /// Key is `YearNr`
    ///
    /// It NOT is guaranteed that all `DepotEntry`'s have the same years.
    /// This can be ensured by running `Depot.ensure_uniform_histories()`
    ///
    /// A year will always have all 12 months.
    pub history: BTreeMap<u16, InvestmentYear>,
    // I switched from HashMap to BTreeMap, to ensure the following attributes:
    // - It stores key-value pairs
    //     -> lookup, insertion and deletion can be done via a key
    // - sorted by keys, so that .iter() will always return the same order
    // - no two keys can be of the same value (only one key 2022 is allowed)
}
impl DepotEntry
{
    // ---------- Initialisation ----------
    pub fn new(variant: InvestmentVariant, name: String, mut savings_plan: Vec<SavingsPlanSection>, history: BTreeMap<u16, InvestmentYear>) -> Self
    {
        Self::_order_savings_plan(&mut savings_plan);
        return Self {
            variant,
            name,
            savings_plan,
            history,
        };
    }

    pub fn default(name: &str, variant: InvestmentVariant) -> Self
    {
        return Self {
            variant,
            name: String::from(name),
            savings_plan: vec![],
            history: BTreeMap::new(),
        };
    }

    /// Uses the same values as DepotEntry::default, but will add the current year with default values
    pub fn default_with_current_year(name: &str, variant: InvestmentVariant) -> Self
    {
        let mut this = Self::default(name, variant);
        let y = CurrentDate::current_year();
        this.history.insert(y, InvestmentYear::default(y));
        return this;
    }

    // ---------- Getters ----------
    pub fn name(&self) -> &str { &self.name }
    pub fn savings_plan(&self) -> &[SavingsPlanSection] { self.savings_plan.as_ref() }

    // ---------- Remaining Methods ----------

    /// Will only return with `Err(Some(SavingsPlanSection))` if the given `section`'s start / end date is inside an existing section.
    /// If this is the case, the existing section is returned.
    ///
    /// If the given section has a wrong format (eg. start after end), `Err(None)` will be returned
    pub fn add_savings_plan_section(&mut self, mut new: SavingsPlanSection) -> Result<(), Option<SavingsPlanSection>>
    {
        // Since the given FastDate's are already checked for correct month and day values, ::new_risky can be used here

        // end is before start
        if new.end <= new.start {
            return Err(None);
        }

        // if annually, check that end is one year ahead of start, if not, override end
        if new.interval == SavingsPlanInterval::Annually {
            if new.end.year() == new.start.year() {
                new.end = FastDate::new_risky(new.start.year() + 1, new.start.month(), new.start.day());
                println!(
                    "Annual interval was selected, but the end date is not at least one year after the start date. \
                    End date has been set to: {}-{}-{}",
                    new.end.year(),
                    new.end.month(),
                    new.end.day()
                );
            }

            if (new.end.month() != new.start.month()) || (new.end.day() != new.start.day()) {
                new.end = FastDate::new_risky(new.end.year(), new.start.month(), new.start.day());
                println!(
                    "Annual interval was selected, but the section does not end on the same month and date at which it starts. \
                    End date has been set to: {}-{}-{}",
                    new.end.year(),
                    new.end.month(),
                    new.end.day()
                );
            }
        }

        // since months and years are inclusive, both month values cant be the same if in the same year
        if new.start > new.end || new.end < new.start {
            return Err(None); // start is not before end
        }

        // this entire function fails if the vec is not ordered
        // if vec is already ordered, its "just" O(n) to be sure
        Self::_order_savings_plan(&mut self.savings_plan);

        if self.savings_plan.len() == 0 {
            self.savings_plan.push(new.clone());
            return Ok(());
        }

        for (current_id, this) in self.savings_plan.clone().iter().enumerate() {
            //
            if new.end < this.start {
                // new is before this section
                self.savings_plan.insert(current_id, new.clone());
            }
            //
            else if (new.end == this.start) || (new.start < this.end && new.end > this.start) || (new.start == this.end) {
                // overlapping (either because some dates are the same (not allow because inclusive), or because some dates are inside the other timeframe)
                return Err(Some(this.clone()));
            }
            //
            else if new.start > this.end {
                // new section is after this existing section
                if current_id == self.savings_plan.len() - 1 {
                    // is the current section the last available? If so, insert new section after this one
                    self.savings_plan.push(new.clone());
                    break;
                } else {
                    continue;
                }
            } else {
                panic!(
                    "DepotEntry::add_savings_plan_section() | \
                    While checking if the new section is  before / overlapping / after  the current section, this one possibility was missed.\n\
                    new section: {:?}, current section: {:?}\n\
                    Please report this exact message to the developers.",
                    new, this
                );
            }
        }

        Self::_order_savings_plan(&mut self.savings_plan);
        return Ok(());
    }

    /// - Checks if there exists any savings plan for the given date
    /// - If there is a plan, but this plan is annually, the savings plans amount will only be returned if `month_nr` is `12`
    /// - If the plan is monthly, the savings plans amount is returned
    /// - If there is no plan, `0.0` is returned
    pub fn get_planned_transactions(&self, date: FastDate) -> f64
    {
        for section in self.savings_plan() {
            // is the given date in this section?
            if (section.start <= date) && (date <= section.end) {
                if section.interval == super::SavingsPlanInterval::Monthly {
                    return section.amount;
                } else if (section.interval == super::SavingsPlanInterval::Annually) && (date.month() == 12) {
                    return section.amount;
                }
            }
        }

        // no section that contains the date was found
        return 0.0;
    }

    /// orders the given `savings_plan` ascending
    fn _order_savings_plan(savings_plan: &mut Vec<SavingsPlanSection>)
    {
        // 1. order by start date ascending (2020 > 2021 > 2022)
        savings_plan.sort_unstable_by(|a, b| a.start.cmp(&b.start));
    }
}
