use chrono::Utc;
use rust_chain::core::{mine_new_block, Block, History, NaiveReorgStrategy};

fn main() {
    println!("Starting the rust chain...");

    let mut h = History::new(Box::new(NaiveReorgStrategy {}));

    loop {
        let prev_block = h.get_last_block().unwrap();
        let height = h.get_height();
        let timestamp = Utc::now().timestamp();
        let txs = Vec::new();

        if height == 5 {
            return;
        }

        println!("Start computing hash...");
        let (nonce, hash) = mine_new_block(height as u64, timestamp, &prev_block.hash, &txs);
        println!("Computed hash");
        let new_block = Block::new(prev_block, hash, timestamp, txs, nonce);
        println!("Appending new block");
        match h.try_to_append(new_block) {
            Ok(_) => println!("Block appended successfully"),
            Err(e) => eprintln!("Error occurred while trying to append a new block: {}", e)
        }
    }
}
