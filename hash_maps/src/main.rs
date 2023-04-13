use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("Team blue has {} points", score);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    //where using an i32 that implements the copy trait will not take ownership,
    //Strings are different in that they do not implement the copy trait, and a HashMap will therefore take ownership.
    //finally, if a reference to a string is passed, the reference itself must be valid for at least as long as the map.
    let field_name = String::from("Favourite color");
    let field_value = String::from("Yellow");

    let mut map = HashMap::new();

    map.insert(field_name, field_value);

    //overwriting a value
    let mut scores2 = HashMap::new();

    scores2.insert(String::from("Blue"), 10);
    //The second insert will overwrite the first, as the same key can't have two values.
    scores2.insert(String::from("Blue"), 25);

    println!("{:?}", scores2);

    //inserting a key,value pair, only if a key doesn't already exist
    let mut scores3 = HashMap::new();
    scores3.insert(String::from("Blue"), 10);

    scores3.entry(String::from("Yellow")).or_insert(50);
    scores3.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores3);

    //this will update a value based on if the old value
    let text = String::from("hello world wonderful world");

    let mut map2 = HashMap::new();

    for word in text.split_whitespace() {
        let count = map2.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map2);
}
