I removed all fields that are simply calculated values from a collection of other fields. I noticed that I already had a bug somewhere, that caused the sum of income / expenses in one year to not change after the value in a month changed from one non-zero value to another non-zero value. To avoid such problems in the future, all these values will be calculated in runtime and will get their own methods

# YAML File structure
```YAML
version: 2                           
accounting:
  goal: 0.75
  history:
    2023:
      year_nr: 2023
      months:
      - month_nr: 1
        income: 0.0
        expenses: 0.0
        note: ''
      - month_nr: 2
        # ...
  recurring_income:
  - name: name for recurring income
    quantity: 5.0
    recurrence: Week
    interval: 1
    frequency: 5
  recurring_expenses:
    # like income
investing:
  comparisons:
  - 5
  - 8  
  depot:
    depot entry 1 name:
      variant: Bond
      savings_plan:
      - start_month: 1
        start_year: 2023
        end_month: 12
        end_year: 2023
        amount: 50.0
        interval: Monthly
      - # ...
      history:
        2023:
          year_nr: 2023
          months:
          - month_nr: 1
            amount: 0.0
            price_per_unit: 0.0
            additional_transactions: 0.0
          - month_nr: 2
            # ...
```

<br>

# Rust structs

```YAML
version: u8
accounting: Accounting
  goal: f64
  history: HashMap<u16, AccountingYear>
    u16:
      year_nr: u16
      months: [AccountingMonth; 12]
      - month_nr: u8
        income: f64
        expenses: f64
        note: String
  recurring_income: Vec<RecurringInOut>
  - name: String
    quantity: f64
    recurrence: Recurrence
    interval: u16
    frequency: u16
  recurring_expenses: Vec<RecurringInOut>
investing: Investing
  comparisons: Vec<u8>
  - u8
  - u8
  depot: HashMap<String, DepotElement>
    name: String
      variant: InvestmentVariant
      savings-plan: Vec<SavingsPlanSection>
        - start_month: u8
          start_year: u16
          end_month: u8
          end_year: u16
          amount: f64
          interval: SavingsPlanInterval
      history: HashMap<u16, InvestmentYear>
        u16:
          year_nr: u16
          months: [InvestmentMonth; 12]
          - month_nr: u8
            amount: f64
            price_per_unit: f64
            additional_transactions: f64
```
<br>

