# Targeted YAML Structure:

```YAML
version: 2                          # One integer, just counting up. No x.y.z versioning                           
accounting:
  goal: 0.7                         # Represents the maximum percentage a user wants to spend of their income (per month/year)
  history:
    2023:
      year_nr: 2023
      income_sum: 1900.0            # will most probably not change, so save starttime by saving here
      expenses_sum: 1400.0
      months:
      - month_nr: 1
        income: 300.0               # always positive
        expenses: 400.0             # always positive
        difference: -100.0
        percentage: 1.3333          # 100% = 1.0
investing:
    # What about time-planned purchases
  name1:
    type: share / fund / etf
    history: 
      2023:
        sum: 4263844.11767379
        months:
        - month_nr: 1
          amount: 34543.234234
          price_per_unit: 123.435   # what was the price per share at the time of adding this data?
          quantity_sold: 0          # Adjust prognosis accordingly to quantity sold in this month

```
