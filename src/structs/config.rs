extern crate dirs;

use crate::structs::Year;

use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

const FILENAME: &'static str = "finance-data.yaml";
pub static CONFIG_IS_INITIALIZED: AtomicBool = AtomicBool::new(false);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub version: u8,
    pub goal: f64,
    pub years: HashMap<u16, Year>,
}
impl Drop for Config {
    fn drop(&mut self) {
        CONFIG_IS_INITIALIZED.store(false, Ordering::SeqCst);
    }
}
impl Config {
    pub fn default() -> Self {
        return Self {
            version: 1,
            goal: 1.0,
            years: HashMap::new(),
        };
    }

    /// - Reads file content and tries to parse it into Config
    /// - Returns default values if file does not exist or is empty
    pub fn new() -> Self {
        match CONFIG_IS_INITIALIZED.load(Ordering::SeqCst) {
            true => panic!("Config was already initialized before!"),
            false => CONFIG_IS_INITIALIZED.store(true, Ordering::SeqCst),
        };

        // get path
        let filepath = match dirs::home_dir() {
            Some(path) => path.join(FILENAME),
            None => panic!(
                "It was expected that this user has a home directory. \
            This was not the case. This program does not work without a valid home directory."
            ),
        };

        let mut file = match OpenOptions::new().create(false).read(true).open(&filepath) {
            Ok(file) => file,
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => return Self::default(),
                _ => panic!("error at opening yaml file > {:?}", e),
            },
        };

        // read content
        let mut content: String = String::new();
        match file.read_to_string(&mut content) {
            Ok(size) => size,
            Err(e) => panic!("error reading in file contents > {:?}", e),
        };
        if content.trim().is_empty() {
            return Self::default();
        }

        let config: Self = match serde_yaml::from_str(&content) {
            Ok(config) => config,
            Err(e) => panic!("Config file is borked, could not be parsed: {:?}", e),
        };

        return config;
    }

    /// 1. Parses the existing `YamlFile` into a `String`
    /// 2. Writes this `String` into the file on disk
    pub fn write(&self) {
        if CONFIG_IS_INITIALIZED.load(Ordering::SeqCst) == false {
            panic!("Attempted to write to uninitialized YamlFile!");
        };

        let filepath = dirs::home_dir()
            .expect("It was expected that this user has a home directory. This was not the case. This program does not work without a valid home directory.")
            .join(FILENAME);

        let mut file = match OpenOptions::new().create(true).truncate(true).write(true).open(&filepath) {
            Ok(file) => file,
            Err(e) => panic!("error at opening yaml file > {:?}", e),
        };

        // parse data
        let yaml = match serde_yaml::to_string(self) {
            Ok(v) => v,
            Err(e) => panic!("error at serde_yaml::to_string > {:?}", e),
        };

        // write data
        match file.write_all(yaml.as_bytes()) {
            Ok(_) => (),
            Err(e) => panic!("error at writing yaml file > {:?}", e),
        };

        println!("Data written into {:?}", &filepath);
    }

    /// - if the year does not already exist, adds it to `YamlFile.years` with default values
    /// - changes nothing if the year exists
    /// - returns the year as a mutable reference (`&mut Year`)`
    ///   - this allows function chaining: `YamlFile.add_or_get_year().function_on_year()`
    pub fn add_or_get_year(&mut self, year_nr: u16) -> &mut Year {
        if self.years.contains_key(&year_nr) == false {
            self.years.insert(year_nr, Year::default(year_nr));
        }

        match self.years.get_mut(&year_nr) {
            Some(y) => return y,
            None => panic!("The year {year_nr} was just created but could not be retrieved from HashMap"),
        };
    }
}
