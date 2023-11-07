# Targeted YAML Structure:

```YAML
version: 2                          # One integer, just counting up. No x.y.z versioning                           
budgeting:
  goal: 0.7                         # Represents the maximum percentage a user wants to spend of their income (per month/year)
  history:
    2023:
      year_nr: 2023
      income_sum: 1900.0
      expenses_sum: 1400.0
      months:
      - month_nr: 1
        income: 300.0               # always positive
        expenses: 400.0             # always positive
        difference: -100.0
        percentage: 1.3333          # 100% = 1.0
        note: string
investing:
  comparisons:                      # User defined growth rates to compare to
  - growth_rate: 5                  # = 5%    These will be affected by all transactions that
  - growth_rate: 7                  #         are done (planned and additional)
  hash1:                            # the hash of the name is used as an index
    name: some userdefined string   # because the name can contain whitespace
    type: Stock / Fund / Etf / Bond / Option / Commodity / Crypto
    savings-plan:
      - start_month: 1              # inclusive
        start_year: 2021
        end_month: 12               # inclusive!
        end_year: 2022
        amount: 50.00
        interval: weekly / monthly / annually
      - start_month: 7
        start_year: 2023
        end_month: none             # this will be Option<u8 / u16>
        end_year: none
        amount: 100.00
        interval: weekly / monthly / annually
    history: 
      2023:
        sum: 4263844.11767379
        months:
        - month_nr: 1
          amount: 34543.23
          price_per_unit: 123.45           # what was the price per share at the time of adding this data?
          planned_transactions: 567.89     # transactions done because of the savings plan
          additional_transactions: 890.12  # additional transactions done, dividends would go here
```

# Currently used fields:
as of commit 6fe60b35d9d84f2a70350735590db1c2273a09c8
```YAML
version: 2
budgeting:
  goal: 0.7                  # currently read only (for table output of one year)
  history:
    2023:
      year_nr: 2023
      income_sum: 1900.0
      expenses_sum: 1400.0
      months:
      - month_nr: 1
        income: 300.0
        expenses: 400.0
        difference: -100.0
        percentage: 1.3333
        # note: string
# investing:
#   comparisons:
#   - growth_rate: 5
#   - growth_rate: 7
#   hash1:
#     name: some userdefined string
#     type: Stock / Fund / Etf / Bond / Option / Commodity / Crypto
#     savings-plan:
#       - start_month: 1
#         start_year: 2021
#         end_month: 12
#         end_year: 2022
#         amount: 50.00
#         interval: weekly / monthly / annually
#       - start_month: 7
#         start_year: 2023
#         end_month: none
#         end_year: none
#         amount: 100.00
#         interval: weekly / monthly / annually
#     history:
#       2023:
#         sum: 4263844.11767379
#         months:
#         - month_nr: 1
#           amount: 34543.23
#           price_per_unit: 123.45
#           planned_transactions: 567.89
#           additional_transactions: 890.12
```
