use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Yellow"),50);

    let change_value = scores.entry(String::from("Blue")).or_insert(40);
    println!("{}",change_value);
    *change_value = 40;
    
    println!("{:?}",scores);
    
}
