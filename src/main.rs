use std::collections::HashMap;

use mods::blockchain::Blockchain;

mod mods;


fn main() {
    let mut blocks = Blockchain::new();
    let mut heshmap = HashMap::new();
    heshmap.insert("system", Blockchain::generate_key_pair());
    heshmap.insert("A", Blockchain::generate_key_pair());
    heshmap.insert("B", Blockchain::generate_key_pair());
    heshmap.insert("C", Blockchain::generate_key_pair());
    println!("{:#?}", heshmap);
    
    let _ = blocks.new_transaction(50, "A".to_string(), "system".to_string(), heshmap["system"].1, heshmap["system"].0);
    let _ = blocks.new_transaction(50, "B".to_string(), "system".to_string(), heshmap["system"].1, heshmap["system"].0);
    let _ = blocks.new_transaction(10, "A".to_string(), "B".to_string(), heshmap["B"].1, heshmap["B"].0);
    let _ = blocks.new_transaction(10, "A".to_string(), "B".to_string(), heshmap["B"].1, heshmap["B"].0);
    let _ = blocks.new_transaction(5, "C".to_string(), "B".to_string(), heshmap["B"].1, heshmap["B"].0);

    // println!("{:#?}\n\n", blocks.get_user_utxos("B".to_string()));
    // println!("{:#?}\n\n", blocks.get_user_utxos("A".to_string()));
    println!("{:#?}\n\n", blocks);
    println!("A: {:?}", blocks.get_user_money("A".to_string()));
    println!("B: {:?}", blocks.get_user_money("B".to_string()));
    println!("C: {:?}", blocks.get_user_money("C".to_string()));
}
