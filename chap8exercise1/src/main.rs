use std::collections::HashMap;

fn main() {
    let v = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4];

    let median = v.len() / 2;

    println!("{median}");

    let mut map: HashMap<i32, i32> = HashMap::new();

    for n in v {
        let count = map.entry(n).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
