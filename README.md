# Finanzbuch
A tool for documenting your personal finances and investments with a focus on statistics.

## Project structure
I started development with the CLI as the "frontend", just because its faster to develop and simpler. I switched to tauri when I realised that what I want with this project is really hard to implement in a nice way as a terminal app.

The library and UI code are individual Cargo projects.


## Contributing
If you wish to help with the development of this program, please read [Contribute.md](/CONTRIBUTE.md)

### User Feedback
If you have used this program and have a suggestion for something that could be done better, or ideas for more features, it would be great if you could start a [discussion](https://github.com/robertosw/finanzbuch/discussions/categories/ideas-feedback) in the Ideas & Feedback category.

## Features / Roadmap
All data is stored locally.
- [ ] File encryption
- [ ] UI Translations

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
- View monthly changes in individual portfolio entries with price and number of units
- Statistics and graphs on portfolio composition and growth

---

- [ ] Create & change comparative growth rates for graphs
- [ ] Portfolio entries
  - [x] Create
  - [x] Change data
  - [ ] Delete
  - [x] Adding historic data
  - [ ] Adding data for the current year
  - [ ] Importing data from csv
- [ ] Create & Change saving plans (start and end date, interval and amount per interval)
  - [ ] One depot entry at a time
    - [ ] Create
    - [ ] Change
    - [ ] Delete
  - [ ] Multiple depot entries in one go
    - [ ] Create
    - [ ] Change
    - [ ] Delete
- [ ] Overview
  - [ ] Factor in inflation
  - [ ] Factor in TER

### Current state
Everything in the sidebar that is greyed-out is not yet implemented.
#### Table for data of one depot entry

![image](https://github.com/robertosw/finanzbuch/assets/47303535/91243692-eebb-46a5-9acd-0cb2f989c353)
Visual precision of float values in one column and year adjusts according to the most precise value in that column and year.

<br>

### CLI - [Examples](./cli/Examples.md)
This will no longer be developed further, but its still available in the [cli](/cli) folder
