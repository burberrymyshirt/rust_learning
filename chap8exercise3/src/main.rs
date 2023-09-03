use std::collections::HashMap;
use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let employees = vec!["Joanne", "Tanja", "goated", "sauce"];
    let departments: Vec<&str> = vec!["Sales", "deez", "nutz"];
    let mut working_in: HashMap<String, String> = HashMap::new();
}
