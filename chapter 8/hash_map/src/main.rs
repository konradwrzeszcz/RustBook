use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("team1", 12);
    scores.insert("team2", 3);
    print_hash_map(&scores);

    let teams = vec!["team1".to_string(), "team2".to_string()];
    let initial_scores = vec![12, 3];
    //collect() method can return different data structure, so we have to explicitly show type of 'scores'
    //in this example type of 'scores' is determined by print_hash_map() function
    let scores = teams.iter().zip(initial_scores.iter()).collect();
    print_hash_map(&scores);

    let team1 = scores.get(&"team1".to_string());
    match team1 {
        Some(team) => println!("Team name: {}", team),
        None => println!("Team doesn't exist"),
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
    
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

fn print_hash_map<K: std::fmt::Debug, V: std::fmt::Debug>(hm: &HashMap<K, V>){
    for (key, value) in hm {
        println!("key: {:?}, value: {:?}", key, value);
    }
}
