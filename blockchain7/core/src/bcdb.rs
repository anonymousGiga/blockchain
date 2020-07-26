use std::{env, fs};
use leveldb::database::Database;
// use leveldb::iterator::Iterable;
use leveldb::kv::KV;
use leveldb::options::{Options, WriteOptions, ReadOptions};
use utils::key;

pub struct BlockChainDb;

impl BlockChainDb {
    pub fn new_db(path: &str) -> Database<key::MyKey> {
        let mut dir = env::current_dir().unwrap();
        dir.push(path);

        let path_buf = dir.clone();
        fs::create_dir_all(dir).unwrap();

        let path = path_buf.as_path();
        let mut options = Options::new();
        options.create_if_missing = true;

        let database = match Database::open(path, options) {
            Ok(db) => { db },
            Err(e) => { panic!("failed to open database: {:?}", e) }
        };

        database
    }

    pub fn write_db(database: &mut Database<key::MyKey>, key: key::MyKey, value: &[u8]) {
        let write_opts = WriteOptions::new();
        match database.put(write_opts, key, &value) {
            Ok(_) => { () },
            Err(e) => { panic!("failed to write genesis block to database: {:?}", e) }
        };
    }

    pub fn read_db(database: &mut Database<key::MyKey>, key: key::MyKey) -> Option<Vec<u8>> {
        let read_opts = ReadOptions::new();
        let res = database.get(read_opts, key);

        match res {
            Ok(data) => data,
            Err(e) => {
                eprintln!("error: {}", e);
                None
            },
        }
    }
}