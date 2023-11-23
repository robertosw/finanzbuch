use super::inv_variant::InvestmentVariant;
use super::inv_year::InvestmentYear;
use super::savings_plan_section::SavingsPlanSection;
use core::panic;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct DepotElement
{
    pub variant: InvestmentVariant,
    savings_plan: Vec<SavingsPlanSection>, // TODO this has to be sorted and checked for overlaps

    /// Key is YearNr
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
    pub fn add_savings_plan_section(&mut self, new_s: &SavingsPlanSection) -> Result<(), Option<SavingsPlanSection>>
    {
        // TODO tests for this
        // TODO check if month values are [1-12]
        // TODO if annually, check that end_month is the same as start_month

        // since months and years are inclusive, both month values cant be the same if in the same year
        let start_after_end_year: bool = new_s.start_year > new_s.end_year;
        let overlayed_months: bool = (new_s.start_year == new_s.end_year) && (new_s.start_month >= new_s.end_month);
        if start_after_end_year || overlayed_months {
            return Err(None); // start is not before end
        }

        // this entire function fails if the vec is not ordered
        // if vec is already ordered, its "just" O(n) to be sure
        Self::_order_savings_plan(&mut self.savings_plan);

        if self.savings_plan.len() == 0 {
            self.savings_plan.push(new_s.clone());
            return Ok(());
        }

        for (current_id, existing_s) in self.savings_plan.clone().iter().enumerate() {
            // "new" = new_s    "this" = existing_s
            let new_ends_before_or_at_this_start_year: bool = new_s.end_year <= existing_s.start_year;
            let new_ends_same_year_this_starts: bool = new_s.end_year == existing_s.start_year;
            let new_ends_at_or_after_this_start_month: bool = new_s.end_month >= existing_s.start_month;
            let new_starts_before_this_end_year: bool = new_s.start_year < existing_s.end_year;
            let new_ends_after_this_start_year: bool = new_s.end_year > existing_s.start_year;
            let new_starts_at_or_after_this_end_year: bool = new_s.start_year >= existing_s.end_year;
            let new_starts_same_year_this_ends: bool = new_s.start_year == existing_s.end_year;
            let new_starts_before_or_at_this_ends_month: bool = new_s.start_month <= existing_s.end_month;

            if new_ends_before_or_at_this_start_year {
                if new_ends_same_year_this_starts && new_ends_at_or_after_this_start_month {
                    return Err(Some(existing_s.clone())); // overlapping
                }

                // new_s is before existing_s
                self.savings_plan.insert(current_id, new_s.clone());
            }
            //
            else if new_starts_before_this_end_year && new_ends_after_this_start_year {
                return Err(Some(existing_s.clone())); // overlapping
            }
            //
            else if new_starts_at_or_after_this_end_year {
                if new_starts_same_year_this_ends && new_starts_before_or_at_this_ends_month {
                    return Err(Some(existing_s.clone())); // overlapping
                }

                // new section is after this existing section
                if current_id == self.savings_plan.len() - 1 {
                    // is the current section the last available? If so, insert new section after this one
                    self.savings_plan.push(new_s.clone());
                    break;
                } else {
                    continue;
                }
            } else {
                panic!(
                    "DepotElement::add_savings_plan_section() | \
                    While checking if the new section is  before / overlapping / after  the current section, this one possibility was missed.\n\
                    new section: {:?}, current section: {:?}",
                    new_s, existing_s
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
    pub fn get_planned_transactions(&self, year_nr: u16, month_nr: u8) -> f64
    {
        for section in self.savings_plan() {
            // is the given date in this section?
            if (section.start_year <= year_nr)
                && (section.end_year >= year_nr)
                && (section.start_month <= month_nr)
                && (section.end_month >= month_nr)
            {
                if section.interval == super::SavingsPlanInterval::Monthly {
                    return section.amount;
                } else if (section.interval == super::SavingsPlanInterval::Annually) && (month_nr == 12) {
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
        // 1. order by start year ascending (2020 > 2021 > 2022)
        savings_plan.sort_unstable_by(|a, b| match a.start_year.cmp(&b.start_year) {
            std::cmp::Ordering::Equal => a.start_month.cmp(&b.start_month), // order by start month ascending (if in the same year)
            other => other,
        });
    }
}
