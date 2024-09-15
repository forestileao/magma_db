use self::constants::{PAGE_SIZE, DATABASE_HEADER_SIZE};
use self::db_header::DatabaseHeader;
use std::os::unix::fs::FileExt;

use constants::{METAPAGE_ID, METAPAGE_SIZE};
use log::{ debug, info };
use std::fs::File;
use std::io::{ Error as IOError, Write };
use std::result::Result;

use super::btree::constants::NODE_HEADER_SIZE;
use super::btree::node::NodeHeader;

pub mod constants;
mod db_header;

type PagerResult<T> = Result<T, IOError>;
pub type PageBuffer = Vec<u8>;

pub struct Config {
    pub db_file_path: String,
}

pub struct Pager {
    db_file: File,
}

impl Pager {
    pub fn new(config: Config) -> PagerResult<Self> {
        debug!("Trying to open filaae {}. Or create if it doesn't exist.", &config.db_file_path);

        let db_file = File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(&config.db_file_path)?;

        info!("Database file in {} has been opened successfully", &config.db_file_path);

        let mut pager = Pager { db_file };
        pager.init_db_file_if_new()?;

        Ok(pager)
    }

    pub fn is_initialized(&self) -> bool {
        // Try to load the metapage:
        let read_result = self.read_page(METAPAGE_ID);

        if read_result.is_err() {
            return false;
        }

        let page_buffer = read_result.unwrap();
        let start_offset = DATABASE_HEADER_SIZE as usize;
        let end_offset = start_offset + (NODE_HEADER_SIZE as usize);
        let node_header_buffer = &page_buffer[start_offset..end_offset];

        let result = bincode::deserialize::<NodeHeader>(node_header_buffer);
        result.is_ok()
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

    fn new_page_buffer() -> PageBuffer {
        vec![0_u8; PAGE_SIZE as usize]
    }

    fn read_page(&self, page_id: u32) -> PagerResult<PageBuffer> {
        let mut page_buffer = Self::new_page_buffer();
        let file_offset = (page_id * PAGE_SIZE) as u64;

        let result = self.db_file.read_exact_at(&mut page_buffer, file_offset);

        if let Err(error) = result {
            let error_kind = error.kind();
            // TODO: Handle the case where the page is not found
            // return match error_kind {
            //     ErrorKind::UnexpectedEof => Err(PageNotFound),
            //     _ => todo!("We dont support handling any IO errors"),
            // };
            todo!("We dont support handling any Read Page errors by now :D");
        }

        Ok(page_buffer)
    }

    fn new_page_payload_buffer(&self, page_id: Option<u32>) -> PageBuffer {
        let page_size = match page_id {
            Some(METAPAGE_ID) => METAPAGE_SIZE,
            _ => PAGE_SIZE,
        };

        vec![0_u8; page_size as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    use tempfile::tempdir;

    fn create_temp_db_file() -> (tempfile::TempDir, String) {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.db");
        (dir, file_path.to_str().unwrap().to_string())
    }

    #[test]
    fn test_pager_new_creates_file() {
        let (dir, file_path) = create_temp_db_file();

        let config = Config {
            db_file_path: file_path.clone(),
        };

        let pager = Pager::new(config).unwrap();

        assert!(Path::new(&file_path).exists());
        assert!(pager.db_file.metadata().unwrap().len() > 0);

        drop(pager);
        dir.close().unwrap();
    }

    #[test]
    fn test_pager_init_db_file_if_new() {
        let (dir, file_path) = create_temp_db_file();

        let config = Config {
            db_file_path: file_path.clone(),
        };

        let pager = Pager::new(config).unwrap();

        // Check if the file size is equal to PAGE_SIZE after initialization
        assert_eq!(pager.db_file.metadata().unwrap().len(), PAGE_SIZE as u64);

        dir.close().unwrap();
    }

    #[test]
    fn test_pager_init_db_file_if_not_new() {
        let (dir, file_path) = create_temp_db_file();

        // Create a non-empty file
        fs::write(&file_path, "Some initial content").unwrap();

        let config = Config {
            db_file_path: file_path.clone(),
        };

        let pager = Pager::new(config).unwrap();

        // Check if the file size remains unchanged
        assert_eq!(pager.db_file.metadata().unwrap().len(), "Some initial content".len() as u64);

        dir.close().unwrap();
    }

    #[test]
    fn test_new_page_buffer() {
        let buffer = Pager::new_page_buffer();
        assert_eq!(buffer.len(), PAGE_SIZE as usize);
        assert!(buffer.iter().all(|&x| x == 0));
    }

    #[test]
    fn test_pager_db_header_serialization() {
        let (dir, file_path) = create_temp_db_file();

        let config = Config {
            db_file_path: file_path.clone(),
        };

        let pager = Pager::new(config).unwrap();

        // Instead of reading directly, we'll check the file contents
        let file_contents = fs::read(&file_path).unwrap();

        // Verify file size is at least PAGE_SIZE
        assert!(file_contents.len() >= PAGE_SIZE as usize);

        // Deserialize the DatabaseHeader from the buffer
        let deserialized_header: DatabaseHeader = bincode::deserialize(&file_contents[..DATABASE_HEADER_SIZE]).unwrap();

        // Check if the deserialized header matches the default header
        assert_eq!(deserialized_header, DatabaseHeader::default());

        drop(pager);
        dir.close().unwrap();
    }
}
