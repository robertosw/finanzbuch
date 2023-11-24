# Finanzbuch
A tool for documenting your personal finances and investments with a focus on statistics.

### Project structure
At the beginning of this project I imagined that this tool would get a GUI. After some time working on this project, I focused more on getting the "logic" done, so I split this into the library code and a simple terminal interface.

I try to keep the library code universal enough so that it is easy to develop a GUI later using tools like [Tauri](https://github.com/tauri-apps/tauri) or [Egui](https://github.com/emilk/egui).

## Features
- [ ] File encryption

### Accounting
- [x] Save income and expenses per month
- [ ] Set a goal for the maximum % of income spent (per month and year)
- [x] Display monthly data with calculated difference, percentage of income spent and if the goal has been achieved for that month
- [x] Display an overview of one year with some statistics
  - Sum Income/Expenses, their Difference, Percentage and Goal
  - Median of the fields above
- [x] Display a graph for the year overview, showing income and expenses per month
- [ ] Save reccurring income and expenses
- [ ] Display reccurring income and expenses in the monthly/year overview
- [x] Import CSV file containing transactional data into one month
  - File will be treated as if all data is for one month only. After importing the user has to select a column that will be summed up into income and expenses for that month. These values will be saved into the stated month.

### Investing
