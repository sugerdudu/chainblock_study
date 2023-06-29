use std::time;
use core::blockchain;
use time::Duration;
use std::thread;

fn main() {
    let mut bc = blockchain::BlockChain::new_blockchain();

    println!("开始挖矿");
    thread::sleep(Duration::from_secs(3));
    bc.add_block("转账10个比特币".to_string());

    thread::sleep(Duration::from_secs(3));
    bc.add_block("转账20个比特币".to_string());

    // bc.add_block("转账30个比特币".to_string());
    // bc.add_block("转账40个比特币".to_string());
    // bc.add_block("转账50个比特币".to_string());

    for x in bc.blocks {
        println!("============================================");
        println!("{:#?}", x);
    }

    println!("Hello, world!");
    // println!("{:?}", bc);
}
