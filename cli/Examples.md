## Menu - 0.1.4
```Console
You can cancel at any moment using Ctrl+C, because data is only written at the moment one dialogue is finished.
Options with ! are not yet implemented.

? Please select an option ›
  Exit
❯ Accounting - In:   Import values for one month from csv file
  Accounting - In:   Manually input values for one month
  Accounting - Out:  Output a table and graph for one year
  Investing - In:    Create new entry in depot
  Investing - In:  ! Set values for comparisons
  Investing - In:    Add new savings plan to one depot entry
  Investing - In:  ! Modify one savings plan of one depot entry
  Investing - In:  ! Input values of one depot entry
  Investing - Out:   Output all saving plans of one depot entry
  Investing - Out:   Show information of individual depot entries
  Investing - Out: ! Show depot overview
```

## Accounting Table & Graph - 0.1.4
Data is generated from lib test `defaults_file_write_read_all()`
```Console
The goal is to spend less than 75 % of monthly income

  Month  |   Income   |  Expenses  | Difference | Percentage | Goal met?
 ------- | ---------- | ---------- | ---------- | ---------- | ---------
 2023  1 |    4226.10 |    2635.01 |    1591.09 |       62 % | true
 2023  2 |    5651.67 |    5437.17 |     214.49 |       96 % | false
 2023  3 |    1827.99 |    3289.56 |   -1461.57 |      179 % | false
 2023  4 |     281.28 |    2825.47 |   -2544.19 |     1004 % | false
 2023  5 |    4991.09 |    4259.32 |     731.77 |       85 % | false
 2023  6 |    1567.69 |    2240.05 |    -672.37 |      142 % | false
 2023  7 |    3337.80 |     666.88 |    2670.93 |       19 % | true
 2023  8 |    2172.10 |     168.86 |    2003.24 |        7 % | true
 2023  9 |     253.74 |    3091.27 |   -2837.53 |     1218 % | false
 2023 10 |    3520.70 |    1527.90 |    1992.80 |       43 % | true
 2023 11 |    5853.56 |    2818.72 |    3034.83 |       48 % | true
 2023 12 |    1970.84 |    1817.19 |     153.65 |       92 % | false

    2023 |   Income   |  Expenses  | Difference | Percentage | Goal met?
 ------- | ---------- | ---------- | ---------- | ---------- | ---------
     Sum |   35654.55 |   30777.41 |    4877.14 |       86 % |  5 / 12  
  Median |    2754.95 |    2726.87 |    1791.94 |       88 % |    42 %  
```
![image](https://github.com/robertosw/Finanzbuch/assets/47303535/1f3ff969-f1a4-426a-b0a8-c8ea53898f6e)


