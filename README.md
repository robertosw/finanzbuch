# Finanzbuch
A tool for documenting your personal finances and investments with a focus on statistics.

### Project structure
At the beginning of this project I imagined that this tool would get a GUI. After some time working on this project, I focused more on getting the "logic" done, so I split this into the library code and a simple terminal interface.

I try to keep the library code universal enough so that it is easy to develop a GUI later using tools like [Tauri](https://github.com/tauri-apps/tauri) or [Egui](https://github.com/emilk/egui).

## Features / Roadmap
- [ ] File encryption
- [ ] Translations

### CLI - [Examples](./cli/Examples.md)

#### Accounting
- [x] Save income and expenses per month
  - [ ] With a note
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

#### Investing
- [ ] Create & Change comparison growth rates for graphs
- Create & Change depot entries
  - [x] Create
  - [ ] Change
  - [ ] Delete
- Create & Change saving plans (start and end date, interval and amount per interval)
  - One depot entry at a time
    - [x] Create
    - [ ] Change
    - [ ] Delete
  - Multiple depot entries in one go
    - [ ] Create
    - [ ] Change
    - [ ] Delete
- Storing monthly data for each depot entry (current price per share, current number of shares and transactions made in addition to the savings plan for this month)
  - [ ] Input new data to existent depot entry
  - [ ] Change data in existent depot entry
- Output overview of depot entries, their data and savings plans
  - [x] All data
  - [ ] Within a specified timeframe
- [ ] Graphs for depot entries

## [Contribute](/CONTRIBUTE.md)

### User Feedback
If you have used this software and have a suggestion for something that could be done better, or ideas for more features, it would be great if you could start a discussion in the `Ideas & Feedback` category.

### Development

- Please create a new branch for new changes and create a pull request if you finished your work on a feature.
- A overview of the structure of the data file and how it is handled in rust can be found in [finanzbuch_lib/development.md](finanzbuch_lib/development.md)

#### Working in a container
You need `xhost` and `docker` installed on your host.
If your host system is linux with an XServer, you can run `./compose-with-xorg.sh` in the root of the project to get setup with everything this project needs. This script changes your XServer rules to allow other software (in this case docker) to connect to it. This is needed so that the tauri window will be displayed in your host.
I use the DevContainer Plugin for VS Codes to work directly inside the docker container. Inside the container, the project is at `/root/project`.

To run the created binary on your host, check the generated file with `ldd filename` to see if you have all necessary libraries on your host system installed.

#### Working on your machine
If you want to work on your host, just take a look into the [Dockerfile](./Dockerfile) to see what you need to setup.

