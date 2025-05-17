use std::collections::HashMap;

pub fn my_hasmaps_funnction() {
    let mut scores: HashMap<String,i32> = HashMap::new();
    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Yellow"),20);
    scores.insert(String::from("Green"),20);

    println!("my hashmaps:{:?}",scores);

    let val = scores.get(&String::from("Blue"));

    println!("{:?}",val);

    for (key, value) in &scores {
        println!("{:?} => {:?}",key,value);
    }

    println!(".................HashMap example..........");

    let text = "Hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count +=1
    }

    for (key, value) in &map {
        println!("{:?} => {:?}",key,value);
    }

}