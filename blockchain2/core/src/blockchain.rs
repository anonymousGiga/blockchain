use crate::block;

pub struct BlockChain {
    pub blocks: Vec<block::Block>,
    pub curr_bits: u32,
}

// const INIT_BITS: u32 = 0x1d00ffff; //Actually should use this value
const INIT_BITS: u32 = 0x2100FFFF;

impl BlockChain {
    pub fn add_block(&mut self, data: String) {
        let pre_block = &self.blocks[self.blocks.len() - 1];
        let new_block = block::Block::new_block(data, pre_block.hash.clone(), self.curr_bits);
        self.blocks.push(new_block);
    }

    fn new_genesis_block() -> block::Block {
        block::Block::new_block("This is genesis block".to_string(), "".to_string(), INIT_BITS)
    }

    pub fn new_blockchain() -> BlockChain {
        BlockChain {
            blocks: vec![BlockChain::new_genesis_block()],
            curr_bits: INIT_BITS,
        }
    }
}