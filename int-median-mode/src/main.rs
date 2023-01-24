use std::collections::HashMap;

fn main() {
    let mut collection = [
        0, 5, 10, 23, 214, 15323, 23, 23, 1513, 123, 475, 908, 1, 1, 1, 1, 1, 1, 1, 74, 33, 73, 75,
    ];
    collection.sort();
    get_median(collection.to_vec());
    get_mode(collection.to_vec());
}

fn get_median(collection: Vec<i32>) -> Vec<i32> {
    let len = collection.len();
    let median = collection[len / 2];
    println!("median: {median}");
    return collection;
}

fn get_mode(collection: Vec<i32>) -> Vec<i32> {
    let mut map = HashMap::new();
    for i in &collection {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    
    // surely, i can do better than this
    let mut mode = 0;
    let mut amount = 0;
    let mut iters = 0;
    for i in map.values() {
        if i > &amount {
             amount = *i;
             mode = map[&iters];
        }
        iters += 1;
    }
    println!("mode: {mode}");
    return collection;
}
