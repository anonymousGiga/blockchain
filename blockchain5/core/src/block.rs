use chrono::prelude::*;
use utils::coder;
use serde::{Deserialize, Serialize};
use crate::pow;
use std::str;
use crate::transaction;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct BlockHeader {
    pub time: i64,
    pub tx_hash: [u8; 32],  //transactions data merkle root hash
    pub pre_hash: [u8; 32],
    pub bits: u32,        //target bit
    pub nonce: u32,       //nonce
    pub state_root: [u8; 32],//after all transaction, the merkle hash of all account state
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub hash: [u8; 32],
    pub transactions: Vec<transaction::Transaction>,
}

impl Block {
    fn min(a: u64, b: u64) -> u64 {
        if a >= b {
            a
        } else {
            b
        }
    }

    fn make_merkle_hash(vtxs: &Vec<transaction::Transaction>) -> [u8; 32] {
        if vtxs.len() == 0 {
           return [0; 32];
        }
    
        let mut vec_merkle_tree: Vec<[u8; 32]> = Vec::new();
        for tx in vtxs.iter() {
            vec_merkle_tree.push(tx.hash);
        }
    
        let mut j: u64 = 0;
        let mut size = vec_merkle_tree.len();
    
        while size > 1 {
            let mut i: u64 = 0;
            let temp_size = size as u64;
            while i < temp_size {
                let i2 = Block::min(i+1, temp_size-1);
                let index1: usize = (j+i) as usize;
                let index2: usize = (j+i2) as usize;
                let merge: ([u8; 32], [u8; 32]) = (vec_merkle_tree[index1], vec_merkle_tree[index2]);
                let merge_serialize = coder::my_serialize(&merge);
                let mut merge_hash: [u8; 32] = [0; 32];
                coder::get_hash(&merge_serialize[..], &mut merge_hash);
                vec_merkle_tree.push(merge_hash);
                i += 2;
            }
    
            j += temp_size;
            size = (size+1)/2;
        } 
    
        let mut merkle_hash: [u8; 32] = [0; 32];
        match vec_merkle_tree.pop() {
            None => println!("vec_merkle_tree is empty!"), 
            Some(t) => merkle_hash = t,
        }
        //println!("merkle hash = {:?}", merkle_hash);
        merkle_hash
    }

    pub fn new_block_template(transactions: Vec<transaction::Transaction>, 
        pre_hash: [u8; 32], bits: u32) -> Block {
        let tx_hash: [u8; 32] = Block::make_merkle_hash(&transactions);

        Block {
            header: BlockHeader {
                time: Utc::now().timestamp(),
                tx_hash: tx_hash,
                pre_hash: pre_hash,
                bits: bits,
                nonce: 0,
                state_root: [0; 32], //after all transaction, the merkle hash of all account state 
            },
            hash: [0; 32],
            transactions: transactions,
        }
    }

    pub fn new_block(transactions: Vec<transaction::Transaction>, 
        pre_hash: [u8; 32], bits: u32) -> Block {

        let mut block = Block::new_block_template(transactions, 
            pre_hash, bits);

        // mining a block
        let my_pow = pow::ProofOfWork::new_proof_of_work(bits);
        my_pow.run(&mut block);

        block
    }
}