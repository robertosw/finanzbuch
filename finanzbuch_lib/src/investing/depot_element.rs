use crate::FastDate;

use super::inv_variant::InvestmentVariant;
use super::inv_year::InvestmentYear;
use super::savings_plan_section::SavingsPlanSection;
use super::SavingsPlanInterval;
use core::panic;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct DepotElement
{
    pub variant: InvestmentVariant,
    savings_plan: Vec<SavingsPlanSection>, // this has to be sorted after every modification

    /// Key is `YearNr`
    pub history: HashMap<u16, InvestmentYear>,
}
impl DepotElement
{
    pub fn new(variant: InvestmentVariant, mut savings_plan: Vec<SavingsPlanSection>, history: HashMap<u16, InvestmentYear>) -> Self
    {
        Self::_order_savings_plan(&mut savings_plan);
        return Self {
            variant,
            savings_plan,
            history,
        };
    }

    pub fn default(variant: InvestmentVariant) -> Self
    {
        return Self {
            variant,
            savings_plan: vec![],
            history: HashMap::new(),
        };
    }

    pub fn savings_plan(&self) -> &[SavingsPlanSection] { self.savings_plan.as_ref() }

    /// Will only return with `Err(Some(SavingsPlanSection))` if the given `section`'s start / end date is inside an existing section.
    /// If this is the case, the existing section is returned.
    ///
    /// If the given section has a wrong format (eg. start after end), `Err(None)` will be returned
    pub fn add_savings_plan_section(&mut self, mut new: SavingsPlanSection) -> Result<(), Option<SavingsPlanSection>>
    {
        // Since the given FastDate's are already checked for correct month and day values, ::new_risky can be used here

        // TODO tests for this

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
                    "DepotElement::add_savings_plan_section() | \
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
