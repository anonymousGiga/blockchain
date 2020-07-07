use leveldb::database::Database;
use crate::block;
use crate::bcdb;
use utils::{coder, key};
use bigint::U256;

pub struct BlockChain {
    pub blocks: Vec<block::Block>,
    curr_bits: u32,
    blocks_db: Box<Database<key::MyKey>>, 
    curr_hash: [u8; 32],
}

// const INIT_BITS: u32 = 0x1d00ffff; //Actually should use this value
const INIT_BITS: u32 = 0x2100FFFF;

impl BlockChain {
    fn write_block(database: &mut Database<key::MyKey>, block: &block::Block) {
        let key = key::MyKey{val: U256::from(block.hash)};
        let value = coder::my_serialize(&block);

        bcdb::BlockChainDb::write_db(database, key, &value);
    }

    fn write_tail(mut database: &mut Database<key::MyKey>, block: &block::Block) {
        let key = key::MyKey{val: U256::from("tail".as_bytes())};
        let value = coder::my_serialize(&(block.hash));
        bcdb::BlockChainDb::write_db(&mut database, key, &value);
    }

    pub fn add_block(&mut self, data: String) {
        // println!("++++++++ self.blocks.len ==== {}", self.blocks.len());
        // let pre_block = &self.blocks[self.blocks.len() - 1];
        // let new_block = block::Block::new_block(data, pre_block.hash.clone(), self.curr_bits);
        let new_block = block::Block::new_block(data, self.curr_hash, self.curr_bits);
        
        //write block to db
        BlockChain::write_block(&mut (self.blocks_db), &new_block);

        //write tail
        BlockChain::write_tail(&mut (self.blocks_db), &new_block);

        self.curr_hash = new_block.hash;

        // push block to blockchain
        self.blocks.push(new_block); 
    }

    fn new_genesis_block() -> block::Block {
        block::Block::new_block("This is genesis block".to_string(), [0; 32], INIT_BITS)
    }

    pub fn new_blockchain() -> BlockChain {
        //create a database
        let mut database = bcdb::BlockChainDb::new_db("blockchain_db");

        //create genesis
        let genesis = BlockChain::new_genesis_block();

        //write genesis to database
        BlockChain::write_block(&mut database, &genesis);

        //write the end block hash to databese
        BlockChain::write_tail(&mut database, &genesis);

        let hash = genesis.hash;
        BlockChain {
            blocks: vec![genesis],
            curr_bits: INIT_BITS,
            blocks_db: Box::new(database),
            curr_hash: hash,
        }
    }
}