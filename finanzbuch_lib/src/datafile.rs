extern crate dirs;

use crate::investing::Investing;
use crate::Accounting;
use serde::Deserialize;
use serde::Serialize;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

const FILENAME: &'static str = "finanzbuch.yaml";
pub const FILE_VERSION: u8 = 4;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct DataFile
{
    /// One integer, just counting up. No x.y.z
    pub version: u8,
    pub accounting: Accounting,
    pub investing: Investing,
    pub write_on_drop: bool,
}
impl Drop for DataFile
{
    fn drop(&mut self)
    {
        if self.write_on_drop {
            self.write();
        }
    }
}
impl Default for DataFile
{
    /// Will write content to file in home path, when instance is dropped
    fn default() -> Self
    {
        return Self {
            version: 4,
            accounting: Accounting::default(),
            investing: Investing::default(),
            write_on_drop: true,
        };
    }
}
impl DataFile
{
    /// Same as `default()`, but wont write content to file when instance is dropped
    ///
    /// Mainly for tests, where this is not wanted
    pub fn default_no_write_on_drop() -> Self
    {
        return Self {
            version: 4,
            accounting: Accounting::default(),
            investing: Investing::default(),
            write_on_drop: false,
        };
    }

    /// Linux / MacOS: `/home/username/finanzbuch.yaml` <br>
    /// Windows: `C:\Users\username\finanzbuch.yaml`
    pub fn home_path() -> PathBuf
    {
        return match dirs::home_dir() {
            Some(path) => path.join(FILENAME),
            None => panic!(
                "It was expected that this user has a home directory. \
                This was not the case. This program does not work without a valid home directory."
            ),
        };
    }

    /// - This is the default version of read(), searches in the users home path for the data file.
    /// - Reads file content and tries to parse it into DataFile
    /// - Returns default values if file does not exist or is empty
    pub fn read() -> Self { Self::read_from_custom_path(Self::home_path()) }

    /// - Same as read(), but with a custom path, for testing purposes
    /// - Reads file content and tries to parse it into DataFile
    /// - Returns default values if file does not exist or is empty
    pub fn read_from_custom_path(filepath: PathBuf) -> Self
    {
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

    /// - This is the default version of write(), writes into a file in the users home directory
    /// 1. Parses the existing `DataFile` into a `String`
    /// 2. Writes this `String` into the file on disk
    pub fn write(&self) { self.write_to_custom_path(Self::home_path()) }

    /// 1. Parses the existing `DataFile` into a `String`
    /// 2. Writes this `String` into the file on disk
    pub fn write_to_custom_path(&self, filepath: PathBuf)
    {
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
