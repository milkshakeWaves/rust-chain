use chrono::Utc;
use rust_chain::core::{mine_new_block, AppendToHistoryError, Block, History};

#[test]
fn create_chain_with_4_blocks() -> Result<(), AppendToHistoryError> {
    let mut hs = History::new();

    for _ in 1..4 {
        let prev_block = hs.get_last_block().unwrap();
        let height = hs.get_height();
        let timestamp = Utc::now().timestamp();
        let data = "Data to include".to_string();

        let (nonce, hash) = mine_new_block(height as u64, timestamp, &prev_block.hash, &data);
        let new_block = Block::new(prev_block, hash, timestamp, data, nonce);

        match hs.try_to_append(new_block) {
            Ok(_) => println!("Block appended successfully"),
            Err(e) => eprintln!("Error appending in history {}", e),
        }
    }
    assert_eq!(4, hs.get_height());

    Ok(())
}

#[test]
fn append_bad_block_to_history_throw_error() -> Result<(), AppendToHistoryError> {
    let mut hs = History::new();

    let prev_block = hs.get_last_block().unwrap();
    let height = hs.get_height();
    let timestamp = Utc::now().timestamp();
    let data = "Data to include".to_string();

    let (nonce, hash) = mine_new_block(height as u64, timestamp, &prev_block.hash, &data);
    let new_block = Block::new(prev_block, hash, timestamp, data, nonce);

    match hs.try_to_append(new_block) {
        Ok(_) => println!("Block appended successfully"),
        Err(e) => eprintln!("Error appending in history {}", e),
    }

    assert_eq!(2, hs.get_height());

    let parent_block = hs.get_last_block().unwrap();
    let bad_block = Block::new(
        parent_block,
        "fake-hash".to_string(),
        Utc::now().timestamp(),
        "bad-data".to_string(),
        0u64,
    );
    match hs.try_to_append(bad_block) {
        Ok(_) => panic!("Block appended successfully"),
        Err(e) => assert_eq!("Cannot append block to history", e.to_string()),
    }

    Ok(())
}
