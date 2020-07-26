use crate::transaction;
use crate::blockchain;
use crate::miner;

pub struct Core {
    blockchain: blockchain::BlockChain,
    miner: miner::Miner,
}

const MINDER_ADDRESS: [u8; 32] = [8; 32];

impl Core {
    pub fn new() -> Core {
        Core {
            blockchain: blockchain::BlockChain::new_blockchain(),
            miner: miner::Miner::new(MINDER_ADDRESS),
        } 
    }

    pub fn mining(& mut self, transactions: &mut Vec<transaction::Transaction>) {
        let b = self.miner.mine_block(transactions, 
            self.blockchain.curr_hash, 
            self.blockchain.curr_bits, 
            self.blockchain.curr_height + 1);
            
        self.blockchain.input_block(b).unwrap();
    } 

    pub fn print(&self) {
        self.blockchain.print();
    }
}