use utils::coder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub hash: [u8; 32],     
    pub from: [u8; 32],       
    pub to: [u8; 32],         
    pub amount: u64,        
    pub fee: u64,
    pub nonce: u64, //transaction number of the from account
    pub sign: String,
}

impl Transaction {
    pub fn new(from: [u8; 32], 
        to: [u8; 32], 
        amount: u64, 
        fee: u64, 
        nonce: u64, 
        sign: String) -> Self {
        let mut tx = Transaction {
            hash: [0; 32],     
            from,       
            to,         
            amount,        
            fee,
            nonce,
            sign,
        };
    
        tx.set_hash();
        tx
    }

    pub fn is_coinbase(&self) -> bool {
        (self.from == [0; 32]) && (self.to != [0; 32])
    }

    pub fn set_hash(&mut self) {
        let tx_data = coder::my_serialize(&self);
        let mut hash: [u8; 32] = [0; 32];
        coder::get_hash(&tx_data[..], &mut hash);

        self.hash = hash;
    }
}

#[cfg(test)]
mod tests {
    use crate::transaction::Transaction;

    #[test]
    fn transaction_works() {
        let tx = Transaction::new([0;32], 
            [1;32], 3, 1,
             0, "".to_string());

        assert_eq!(true, tx.is_coinbase());
    }
}