extern crate dirs;

use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;

use super::accounting::Accounting;
use super::investing::Investment;

const FILENAME: &'static str = "finance-data.yaml";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DataFile {
    pub version: u8,
    pub accounting: Accounting,
    pub investing: HashMap<String, Investment>,
}
impl DataFile {
    pub fn default() -> Self {
        return Self {
            version: 2,
            accounting: Accounting::default(),
            investing: HashMap::new(),
        };
    }

    /// - Reads file content and tries to parse it into DataFile
    /// - Returns default values if file does not exist or is empty
    pub fn read() -> Self {
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
                _ => panic!("error at opening data file > {:?}", e),
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

        let datafile: Self = match serde_yaml::from_str(&content) {
            Ok(datafile) => datafile,
            Err(e) => panic!("DataFile file is borked, could not be parsed: {:?}", e),
        };

        return datafile;
    }

    /// 1. Parses the existing `DataFile` into a `String`
    /// 2. Writes this `String` into the file on disk
    pub fn write(&self) {
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
}
