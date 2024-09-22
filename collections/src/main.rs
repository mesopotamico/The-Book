use std::collections::HashMap;

fn main() {
    let s1: String = String::from("hello");
    let s2: String = String::from("world!");
    
    let mut scores = HashMap::new();
    
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("red"), 50);

    let team_name = String::from("blue");
    let score = scores.get(&team_name).copied().unwrap_or(0); 
    println!("The scores are {:?}", scores);

}

