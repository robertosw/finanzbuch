use serde::Deserialize;
use serde::Serialize;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::process::exit;
use std::time::Instant;
use tinyrand::Rand;
use tinyrand::RandRange;
use tinyrand::Seeded;
use tinyrand::StdRand;
use tinyrand_std::ClockSeed;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
struct YamlFile {
    version: String,
    goal: f32,
    years: Vec<YamlYear>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
struct YamlYear {
    year: u16,
    income: f64,
    expenses: f64,
    months: [YamlMonth; 12],
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
struct YamlMonth {
    month: u8,
    income: f64,
    expenses: f64,
    difference: f64,
    percentage: f64,
}
impl YamlMonth {
    fn default(month: u8) -> Self {
        return YamlMonth {
            month,
            income: 0.0,
            expenses: 0.0,
            difference: 0.0,
            percentage: 0.0,
        };
    }

    fn default_months() -> [Self; 12] {
        return [
            YamlMonth::default(1),
            YamlMonth::default(2),
            YamlMonth::default(3),
            YamlMonth::default(4),
            YamlMonth::default(5),
            YamlMonth::default(6),
            YamlMonth::default(7),
            YamlMonth::default(8),
            YamlMonth::default(9),
            YamlMonth::default(10),
            YamlMonth::default(11),
            YamlMonth::default(12),
        ];
    }
}

const FILE: &'static str = "/root/project/sample.yaml";

fn main() {
    let (income, expenses, month, year) = generate_random_input();
    println!("in {}, out {}, month {}, year {}", income, expenses, month, year);

    let sample = YamlFile {
        version: String::from("0.0.1"),
        goal: 0.85,
        years: vec![
            YamlYear {
                year: 2022,
                income: 0.0,
                expenses: 0.0,
                months: YamlMonth::default_months(),
            },
            YamlYear {
                year: 2023,
                income: 0.0,
                expenses: 0.0,
                months: YamlMonth::default_months(),
            },
        ],
    };

    write(sample);
    let ymlfile = read();

    println!("{:?}", ymlfile);
}

/// return values
/// - income, expenses, month, year
fn generate_random_input() -> (f64, f64, u8, u16) {
    let seed = ClockSeed::default().next_u64();
    let mut rand = StdRand::seed(seed);
    let rand_month: u8 = rand.next_range(1 as usize..13 as usize) as u8;
    let rand_year: u16 = rand.next_range(2000 as usize..2024 as usize) as u16;
    let rand_income: f64 = rand.next_u32() as f64 * 1000000.0 / 11.11;
    let rand_expenses: f64 = rand.next_u32() as f64 * 1000000.0 / 11.11;
    return (rand_income, rand_expenses, rand_month, rand_year);
}

fn read() -> YamlFile {
    let mut file = match OpenOptions::new().create(false).read(true).open(FILE) {
        Ok(file) => file,
        Err(e) => {
            println!("error at opening yaml file > {:?}", e);
            exit(1);
        }
    };

    let mut content: String = String::new();
    match file.read_to_string(&mut content) {
        Ok(size) => size,
        Err(e) => {
            println!("error reading in file contents > {:?}", e);
            exit(1);
        }
    };

    let ymlfile: YamlFile = match serde_yaml::from_str(&content) {
        Ok(v) => v,
        Err(e) => {
            println!("error reading in file contents > {:?}", e);
            exit(1);
        }
    };

    return ymlfile;
}

fn write(ymlfile: YamlFile) {
    let yaml = match serde_yaml::to_string(&ymlfile) {
        Ok(v) => v,
        Err(e) => {
            println!("error at serde_yaml::to_string > {:?}", e);
            exit(1);
        }
    };

    match OpenOptions::new().create(true).truncate(true).write(true).open(FILE) {
        Ok(mut file) => {
            match file.write_all(yaml.as_bytes()) {
                Ok(_) => {}
                Err(e) => {
                    println!("error at writing yaml file > {:?}", e);
                    exit(1);
                }
            };
        }
        Err(e) => {
            println!("error at opening yaml file > {:?}", e);
            exit(1);
        }
    };
}
