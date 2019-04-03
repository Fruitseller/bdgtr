use std::fs::{File, OpenOptions};
use std::io::Error;

pub fn get_file_for_read(path: &str) -> Result<File, Error> {
    OpenOptions::new().read(true).open(path)
}

pub fn get_file_for_write(path: &str) -> Result<File, Error> {
    OpenOptions::new().append(true).create(true).open(path)
}
