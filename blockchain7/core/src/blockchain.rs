use leveldb::database::Database;
use crate::block;
use crate::bcdb;
use utils::{coder, key};
use bigint::U256;
use crate::transaction;
use crate::pow;
use std::collections::HashMap;
use std::sync::Mutex;

pub struct BlockChain {
    //pub blocks: Vec<block::Block>,
    block_index: Mutex<HashMap<[u8; 32], block::Block>>,
    blocks_db: Box<Database<key::MyKey>>, 
    pub genesis_hash: [u8; 32],
    pub curr_hash: [u8; 32],
    pub curr_bits: u32,
    pub curr_height: u64, 
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

    //pub fn add_block(&mut self, transactions: Vec<transaction::Transaction>) {
    //    // let tail_block = read_db();
    //    // let height = tail_block.block_header.height;
    //    let height = 0;
    //    let new_block = 
    //        block::Block::new_block(transactions, self.curr_hash, self.curr_bits, height+1);
    //    
    //    self.input_block(new_block).unwrap(); 
    //}

    pub fn input_block(&mut self, block: block::Block) -> Result<(), String>{
        // //validate the block
        // if check_block(&block).is_err() {
        //     return Err("block is invalid".to_string());
        // }

        //write block to db
        BlockChain::write_block(&mut (self.blocks_db), &block);

        if block.header.height > self.curr_height {
            //write tail
            BlockChain::write_tail(&mut (self.blocks_db), &block);
            self.curr_hash = block.hash;
            self.curr_bits = block.header.bits;
            self.curr_height = block.header.height;

            //再判断是否需要回朔
        }

        //
        BlockChain::update_map(&mut self.block_index, block.clone());

        //// push block to blockchain
        //self.blocks.push(block); 

        Ok(())
    }

    fn get_genesis_block() -> block::Block {
        let tx = transaction::Transaction::new([0; 32], 
            [0; 32], 0, 0, 0, "This is genesis".to_string());

        let mut bc = block::Block::new_block_template(vec![tx], [0; 32], INIT_BITS, 0);
        let data = pow::ProofOfWork::prepare_data(&mut bc, 0);
        let mut hash: [u8; 32] = [0; 32];
        coder::get_hash(&data[..], &mut hash);
        bc.hash = hash;
        
		bc
    }

    fn update_map(map: & mut Mutex<HashMap<[u8; 32], block::Block>>, 
        block: block::Block) {
        let mut map = map.lock().unwrap();
        map.insert(block.hash, block);
    }
    
    pub fn new_blockchain() -> BlockChain {
        //create a database
        let mut database = bcdb::BlockChainDb::new_db("blockchain_db");

        //get genesis
        let genesis = BlockChain::get_genesis_block();

        //write genesis to database
        BlockChain::write_block(&mut database, &genesis);

        //write the end block hash to databese
        BlockChain::write_tail(&mut database, &genesis);

        let hash = genesis.hash;

        let mut block_index = Mutex::new(HashMap::new());
        BlockChain::update_map(&mut block_index, genesis.clone());

        BlockChain {
            //blocks: vec![genesis.clone()],
            block_index: block_index,
            genesis_hash: hash,
            curr_bits: INIT_BITS,
            blocks_db: Box::new(database),
            curr_hash: hash,
            curr_height: 0,
        }
    }

    pub fn print(&self) {
        let mut hash = self.curr_hash;
        let mut blocks: Vec<block::Block> = Vec::new();

        let map = self.block_index.lock().unwrap();
        loop {
            if let Some(b) = map.get(&hash) {
                hash = b.header.pre_hash;
                blocks.push(b.clone());
            } else {
                panic!("found block error");
            }


            if hash == self.genesis_hash {
                break;
            }
        } 

        blocks.reverse();

        for b in blocks {
            println!("++++++++++++++++++++++++++++++++++++++++++++");
            println!("{:?}", b);
            println!("");
        }
    }


}
