use super::{pager, btree::{operations}};
use std::io::Result;

#[derive(Debug)]
pub struct Config {
    pub cache_size_mb: u32,
    pub create: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            cache_size_mb: 100,
            create: true,
        }
    }
}

pub trait Connection {
    fn insert(&mut self, key: u32, value: &[u8]) -> Result<()>;
}

pub struct StorageEngineConnection {
    config: Config,
    pager: pager::Pager,
}

impl StorageEngineConnection {
    pub fn new(db_path: String, config: Config) -> Result<StorageEngineConnection> {
        let pager_config = pager::Config {
            db_file_path: db_path,
        };
        let mut pager = pager::Pager::new(pager_config)?;

        //BTreeOperations::initialize::root_node_if_needed(&mut pager);
        Ok(StorageEngineConnection {
            config,
            pager,
        })
    }

}
