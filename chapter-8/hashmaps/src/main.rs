fn main() {
    println!("Hello, world!");

    use std::collections::HashMap;

    let mut scores = HashMap::new();


    scores.insert(String::from("Foo"), 23);
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Black"), 100);
    scores.insert(String::from("Purple"), 75);
    scores.insert(String::from("Orange"), 25);
    scores.insert(String::from("Bar"), 2);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    println!("================");


    let text = "hello world wonderful world i am here for the world today";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}")
}
