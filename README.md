# Finanzbuch
A tool for documenting your personal finances and investments with a focus on statistics.

## Project structure
I started development with the CLI as the "frontend", just because its faster to develop and simpler. I switched to tauri when I realised that what I want with this project is really hard to implement in a nice way as a terminal app.

The library and UI code as different Cargo projects, to maintain a clear cut between the two.


## Contributing
If you wish to help with the development of this app, please read [Contribute.md](/CONTRIBUTE.md)

### User Feedback
If you have used this software and have a suggestion for something that could be done better, or ideas for more features, it would be great if you could start a discussion in the `Ideas & Feedback` category.

## Features / Roadmap
All data is stored locally.
- [ ] File encryption
- [ ] Translations

### Accounting

- Track monthly income, expenses and savings goals.
- Store and view regular income and expenses so you can plan ahead

---

- [ ] Save income and expenses per month
  - [ ] With a note for each month
- [ ] Set a goal for the maximum % of income spent
- [ ] View annual summary with monthly calculated data such as difference, % of revenue spent and target achieved
  - [ ] Simple table for one year
    - Sum Income/Expenses, their Difference, Percentage and Goal
    - Median of the fields above
  - [ ] Display reccurring income and expenses
- [ ] Diagrams to visualise patterns
- [ ] Store and edit reccurring income and expenses
- [ ] Import CSV file containing transactional data into one month
  - File will be treated as if all data is for one month only. After importing the user has to select a column that will be summed up into income and expenses for that month. These values will be saved into the stated month.

<br>

### Investing
- Depot overview with total value, savings rate and growth - all per month
- View monthly changes in individual portfolio entries with price and number of units
- Statistics and graphs on portfolio composition and growth

---

- [ ] Create & change comparative growth rates for graphs
- Portfolio entries
  - [ ] Create
  - [ ] Change data
  - [ ] Change name and type
  - [ ] Delete
  - [ ] Adding new data
  - [ ] Importing data from csv
- Create & Change saving plans (start and end date, interval and amount per interval)
  - One depot entry at a time
    - [ ] Create
    - [ ] Change
    - [ ] Delete
  - Multiple depot entries in one go
    - [ ] Create
    - [ ] Change
    - [ ] Delete
- Output overview of depot entries, their data and savings plans
  - [ ] All data
  - [ ] Within a specified timeframe

<br>

### CLI - [Examples](./cli/Examples.md)
This will no longer be developed further. Available features are shown below
