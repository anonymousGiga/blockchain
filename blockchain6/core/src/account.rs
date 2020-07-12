use crate::transaction;
use serde::{Deserialize, Serialize};
use utils::coder;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Account {
    pub nonce: u64,
    pub balance: u64,
    pub address: [u8; 32],
    pub hash: [u8; 32],
    private: [u8; 32],
}

impl Account {
    pub fn new(address: [u8; 32], private: [u8; 32]) -> Account {
        let mut account = Account {
            nonce: 0,
            balance: 0,
            address: address,
            hash: [0; 32],
            private: private,
        };

        account.set_hash();
        account
    }

    fn set_hash(&mut self) {
        let account_data = coder::my_serialize(&self);
        let mut hash: [u8; 32] = [0; 32];
        coder::get_hash(&account_data[..], &mut hash);

        self.hash = hash;
    }

    pub fn send_to(&mut self, to: [u8; 32], amount: u64, fee: u64) 
        -> Result<transaction::Transaction, String> {
        if amount + fee > self.balance {
            return Err("Err".to_string());
        }     

        self.balance -= amount;
        self.balance -= fee;
        self.nonce += 1;
        self.set_hash();

        let tx = transaction::Transaction::new(self.address, 
            to, amount, fee, self.nonce, "sign".to_string());

        Ok(tx)
    }
}

#[cfg(test)]
mod tests {
    use crate::account::Account;

    #[test]
    fn account_works() {
        let mut ac = Account::new([2; 32], [0; 32]);
        let ret = ac.send_to([3; 32], 3, 1);
        assert_eq!(ret.is_err(), true);
    }
}