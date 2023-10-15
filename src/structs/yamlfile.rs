extern crate dirs;

use crate::structs::Year;
use crate::YAMLFILE_IS_INITIALIZED;

use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::sync::atomic::Ordering;

const FILENAME: &'static str = "finance-data.yaml";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct YamlFile {
    pub version: u8,
    pub goal: f64,
    pub years: HashMap<u16, Year>,
}
impl YamlFile {
    pub fn default() -> Self {
        return Self {
            version: 1,
            goal: 1.0,
            years: HashMap::new(),
        };
    }

    /// Call `.init()` after this command to read the file from disk
    pub fn new() -> Self {
        return Self::default();
    }

    /// Initializes this struct
    /// - Tries to read the yaml file (from users home directory)
    ///     - Creates the file if non-existent or empty with default values
    ///     - Will exit programm with error message if the file existed but could not be read or parsed
    /// - Will modify `self`, if the file exists and parsing was successful
    pub fn init(&mut self) {
        match YAMLFILE_IS_INITIALIZED.load(Ordering::SeqCst) {
            true => panic!("YamlFile was already initialized before!"),
            false => YAMLFILE_IS_INITIALIZED.store(true, Ordering::SeqCst),
        };

        // get path
        let filepath = match dirs::home_dir() {
            Some(path) => path.join(FILENAME),
            None => panic!(
                "It was expected that this user has a home directory. \
                This was not the case. This program does not work without a valid home directory."
            ),
        };

        // check if file exists, create with template if not
        match filepath.try_exists() {
            Ok(true) => (),
            Ok(false) => {
                println!("File does not exist, creating now");
                self.init_new_file();
                return;
            }
            Err(e) => panic!("It was not possible to check if the data file exists. Expected at {:?}. \n {e}", filepath),
        };

        let mut file = match OpenOptions::new().create(false).read(true).open(&filepath) {
            Ok(file) => file,
            Err(e) => panic!("error at opening yaml file > {:?}", e),
        };

        // if the file is empty for some reason, fill with template
        let mut content: String = String::new();
        match file.read_to_string(&mut content) {
            Ok(size) => size,
            Err(e) => panic!("error reading in file contents > {:?}", e),
        };
        if content.trim().is_empty() {
            println!("File is empty, initializing now");
            self.init_new_file();
            return;
        }

        let ymlfile: Self = match serde_yaml::from_str(&content) {
            Ok(v) => v,
            Err(e) => panic!("error reading in file contents > {:?}", e),
        };

        *self = ymlfile;
    }

    /// Fills `self` with default values and calls `self.write()` to write these default values into the file
    fn init_new_file(&mut self) {
        *self = Self::default();
        self.write();
    }

    /// 1. Parses the existing `YamlFile` into a `String`
    /// 2. Writes this `String` into the file on disk
    pub fn write(&self) {
        if YAMLFILE_IS_INITIALIZED.load(Ordering::SeqCst) == false {
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
