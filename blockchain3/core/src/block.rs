use chrono::prelude::*;
use utils::coder;
use serde::{Deserialize, Serialize};
use crate::pow;
use std::str;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct BlockHeader {
    pub time: i64,
    pub tx_hash: [u8; 32],  //transactions data merkle root hash
    pub pre_hash: [u8; 32],
    pub bits: u32,        //target bit
    pub nonce: u32,       //nonce
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub hash: [u8; 32],
    pub data: String, //transactions data
}

impl Block {
    pub fn new_block(data: String, pre_hash: [u8; 32], bits: u32) -> Block {
        let transactions = coder::my_serialize(&data);
        let mut tx_hash: [u8; 32] = [0; 32];
        coder::get_hash(&transactions[..], &mut tx_hash);

        let time = Utc::now().timestamp();

        let mut block = Block {
            header: BlockHeader {
                time: time,
                // tx_hash: str::from_utf8(&tx_hash).unwrap().to_string(),  //transactions data merkle root hash
                tx_hash: tx_hash,
                pre_hash: pre_hash,
                bits: bits,
                nonce: 0,
            },
            hash: [0; 32],
            data: data,
        };

        let my_pow = pow::ProofOfWork::new_proof_of_work(bits);

        my_pow.run(&mut block);

        block
    }
}