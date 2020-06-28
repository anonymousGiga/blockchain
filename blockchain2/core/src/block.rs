use chrono::prelude::*;
use utils::coder;
use serde::{Deserialize, Serialize};
use crate::pow;
use std::str;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct BlockHeader {
    pub time: i64,
    pub tx_hash: String,  //transactions data merkle root hash
    pub pre_hash: String,
    pub bits: u32,        //target bit
    pub nonce: u32,       //nonce
}

#[derive(Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub hash: String,
    pub data: String, //transactions data
}

impl Block {
    pub fn new_block(data: String, pre_hash: String, bits: u32) -> Block {
        let transactions = coder::my_serialize(&data);
        let mut tx_hash: [u8; 32] = [0; 32];
        coder::get_hash(&transactions[..], &mut tx_hash);

        let time = Utc::now().timestamp();

        let mut block = Block {
            header: BlockHeader {
                time: time,
                // tx_hash: str::from_utf8(&tx_hash).unwrap().to_string(),  //transactions data merkle root hash
                tx_hash: tx_hash.iter().map(|&c| c as char).collect::<String>(),
                pre_hash: pre_hash,
                bits: bits,
                nonce: 0,
            },
            hash: "".to_string(),
            data: data,
        };

        let my_pow = pow::ProofOfWork::new_proof_of_work(bits);

        my_pow.run(&mut block);

        block
    }
}