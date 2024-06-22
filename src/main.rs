use mods::{block, blockchain::Blockchain};

mod mods;


fn main() {
    let mut blocks = Blockchain::new();
    let _ = blocks.new_transaction(50, "A".to_string(), "system".to_string()).unwrap();
    let _ = blocks.new_transaction(50, "B".to_string(), "system".to_string()).unwrap();
    let _ = blocks.new_transaction(10, "A".to_string(), "B".to_string()).unwrap();
    let _ = blocks.new_transaction(10, "A".to_string(), "B".to_string()).unwrap();

    println!("{:#?}\n\n", blocks.check_user_utxos("B".to_string()));
    println!("{:#?}", blocks.check_user_utxos("A".to_string()));
    // println!("{:#?}", blocks)
}
