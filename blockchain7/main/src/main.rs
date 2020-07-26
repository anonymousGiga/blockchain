use core::{mycore, transaction};
use std::thread;
use std::time::Duration;
use cli::cli::Cli;

fn main() {
    let mut co = mycore::Core::new();

    println!("start mining .... ");
    thread::sleep(Duration::from_secs(5));
    let tx = transaction::Transaction::new([2; 32], 
        [3; 32], 3, 1, 0, "".to_string());
    co.mining(& mut vec![tx]);
    println!("produce a block !");

    println!("");
    println!("start mining .... ");
    thread::sleep(Duration::from_secs(5));
    let tx = transaction::Transaction::new([4; 32], 
        [5; 32], 5, 1, 0, "".to_string());
    co.mining(& mut vec![tx]);
    println!("produce a block !");

    //print all blocks
    co.print();
    
    Cli::start();
}