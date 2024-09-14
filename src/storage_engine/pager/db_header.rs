use super::constants::{MAGIC_HEADER_STRING, PAGE_SIZE};
use serde::{Deserialize, Serialize};

/// Represents the header of the database file.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DatabaseHeader {
    /// Magic string to identify the database file format.
    pub magic_header: [u8; MAGIC_HEADER_STRING.len()],
    /// Size of each page in the database.
    pub page_size: usize,
}

impl Default for DatabaseHeader {
    fn default() -> Self {
        DatabaseHeader {
            magic_header: MAGIC_HEADER_STRING,
            page_size: PAGE_SIZE,
        }
    }
}
