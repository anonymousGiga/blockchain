use crate::transaction;
use crate::block;
use crate::pow;

pub struct Miner {
    address: [u8; 32],
}

impl Miner {
    pub fn new(address: [u8; 32]) -> Miner {
        Miner{ address }
    }

    fn produce_block(transactions: Vec<transaction::Transaction>, 
        pre_hash: [u8; 32], bits: u32, height: u64) -> block::Block {

        let mut block = block::Block::new_block_template(transactions, 
            pre_hash, bits, height);

        let my_pow = pow::ProofOfWork::new_proof_of_work(bits);
        my_pow.run(&mut block);

        block
    }

    pub fn mine_block(&self, transactions: &mut Vec<transaction::Transaction>, 
        pre_hash: [u8; 32], bits: u32, height: u64) -> block::Block {

        let coinbase = transaction::Transaction::new([0; 32], 
            self.address, 0, 0, 0, "coinbase".to_string());
        
        let mut txs: Vec<transaction::Transaction> = Vec::new();
        txs.push(coinbase);
        txs.append(transactions);

        let bits = bits;//really, should check the bits need modify

        Miner::produce_block(txs, pre_hash, bits, height)
    }
}
