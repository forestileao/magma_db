use self::constants::{PAGE_SIZE, DATABASE_HEADER_SIZE};
use self::db_header::DatabaseHeader;

use log::{ debug, info };
use std::fs::File;
use std::io::{ Error as IOError, Write };
use std::result::Result;
use std::vec;

mod constants;
mod db_header;

type PagerResult<T> = Result<T, IOError>;


pub struct Config {
    pub db_file_path: String,
}

pub struct Pager {
    db_file: File,
}

impl Pager {
    pub fn new(config: Config) -> PagerResult<Self> {
        debug!("Trying to open file {}. Or create if it doesn't exist.", &config.db_file_path);

        let db_file = File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(&config.db_file_path)?;

        info!("Database file in {} is opened successfully", &config.db_file_path);

        let mut pager = Pager { db_file };
        pager.init_db_file_if_new()?;

        Ok(pager)
    }

    fn init_db_file_if_new(&mut self) -> PagerResult<()> {
        let file_metadata = self.db_file.metadata()?;
        let file_length = file_metadata.len();

        if file_length > 0 {
            info!("Database file is not empty. It's file size is: {}.", file_length);
            return Ok(());
        }

        info!("Database file is empty. Initializing metapage.");
        let mut metapage_buffer = Self::new_page_buffer();
        let db_header = DatabaseHeader::default();

        let encoded_db_header: Vec<u8> = bincode::serialize(&db_header).unwrap();

        // Ensure the encoded header doesn't exceed the reserved space
        // to prevent overwriting the first page's metadata
        assert!(encoded_db_header.len() <= DATABASE_HEADER_SIZE,
            "Encoded header size ({}) exceeds reserved space ({})",
            encoded_db_header.len(), DATABASE_HEADER_SIZE);

        metapage_buffer[..encoded_db_header.len()].copy_from_slice(&encoded_db_header);
        self.db_file.write(&metapage_buffer)?;

        info!("Database header has been written to metapage.");
        Ok(())
    }

    fn new_page_buffer() -> [u8; PAGE_SIZE] {
        [0; PAGE_SIZE]
    }
}
