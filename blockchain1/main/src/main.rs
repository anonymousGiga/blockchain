use core::blockchain;
use std::thread;
use std::time::Duration;

fn main() {
    let mut bc = blockchain::BlockChain::new_blockchain();

    println!("start mining .... ");
    thread::sleep(Duration::from_secs(5));
    bc.add_block(String::from("a -> b: 5 btc"));
    println!("produce a block !");

    println!("");
    println!("start mining .... ");
    thread::sleep(Duration::from_secs(5));
    bc.add_block("c -> d: 1 btc".to_string());
    println!("produce a block !");

    for b in bc.blocks {
        println!("++++++++++++++++++++++++++++++++++++++++++++");
        println!("{:#?}", b);
        println!("");
    }
}
