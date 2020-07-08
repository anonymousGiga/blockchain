use core::blockchain;
use core::transaction;
use std::thread;
use std::time::Duration;

fn main() {
    let mut bc = blockchain::BlockChain::new_blockchain();

    println!("start mining .... ");
    thread::sleep(Duration::from_secs(5));
    let tx = transaction::Transaction::new([2; 32], 
        [3; 32], 3, 1, 0, "".to_string());
    bc.add_block(vec![tx]);
    println!("produce a block !");

    println!("");
    println!("start mining .... ");
    thread::sleep(Duration::from_secs(5));
    let tx = transaction::Transaction::new([4; 32], 
        [5; 32], 5, 1, 0, "".to_string());
    bc.add_block(vec![tx]);
    println!("produce a block !");

    for b in bc.blocks {
        println!("++++++++++++++++++++++++++++++++++++++++++++");
        println!("{:?}", b);
        println!("");
    }
}