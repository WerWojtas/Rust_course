use std::collections::HashMap;
use reqwest;


fn words_stats(words: &Vec<String>) -> HashMap<String, i32> {
    let mut map = HashMap::new();
    for e in words {
        if map.contains_key(e) {
            let count = map.get_mut(e).unwrap();
            *count += 1;
        } else {
            map.insert(e.clone(), 1);
        }
    }
    map
}

fn words_stats_entry(words: &Vec<String>) -> HashMap<String, i32> {
    let mut map = HashMap::new();
    for e in words {
        *map.entry(e.clone()).or_insert(0) += 1;
    }
    map
}

fn hash_map_words_stats_poem() {
    let response = reqwest::blocking::get("https://wolnelektury.pl/media/book/txt/pan-tadeusz.txt").expect("Cannot get poem from a given URL");
    let poem = response.text().unwrap();
 
    let stats = words_stats_entry(&split_to_words(&poem));
    let sorted_stats = sort_stats(&stats);
    println!("{:?}", &sorted_stats[..20]);
}
 
 
fn split_to_words(s: &str) -> Vec<String> {
    let mut words = Vec::new();
    for word in s.split_whitespace() {
        let unified_word = word.trim_matches(|c| char::is_ascii_punctuation(&c)).to_lowercase();
        words.push(unified_word);
    }
    words
}
 
fn sort_stats(stats : &HashMap<String, i32>) -> Vec<(&str, i32)> {
    let mut sorted_stats : Vec<(&str, i32)> = Vec::new();
    for (word, count) in stats.iter() {
        sorted_stats.push((word, *count));
    }
 
    sorted_stats.sort_by(|(_, c1), (_, c2)| c2.partial_cmp(c1).expect("FAILED"));
 
    sorted_stats
}

fn main() {
    let mut map = HashMap::new();
    map.insert(String::from("world"), 1);
    map.insert(String::from("rust"), 2);
    map.remove("world");
    println!("{:?}", map);
    let cnt : Option<&i32> = map.get("rust"); // returns an optional reference to a value in the map
    match cnt {
        Some(c) => println!("Count: {}", c),
        None => println!("Not found")
    }
    for (text, number) in &map { // immutable borrow 
        println!("{} has {} occurrences", text, number);
    }
    let entry = map.entry(String::from("world")); // returns an Entry
    println!("{:?}", entry);
    let word = "world";
    *map.entry(word.to_owned()).or_insert(0) += 1;
    println!("{:?}", map);
    map.entry(word.to_owned()).and_modify(|counter| *counter += 1).or_insert(1);
    println!("{:?}", map);
    let word = "hello";
    map.entry(word.to_owned()).and_modify(|counter| *counter += 1).or_insert(1);
    println!("{:?}", map);

    hash_map_words_stats_poem();
}