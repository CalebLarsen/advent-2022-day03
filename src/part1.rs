use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let in_file = &args[1];
    let in_data = fs::read_to_string(in_file).unwrap();

    let a = "a".chars().nth(0).unwrap() as u32;
    let A = "A".chars().nth(0).unwrap() as u32;
    println!("a val: {} -- A val: {}", a, A);
    let mut sum: u32 = 0;
    let items: Vec<&str> = in_data.split("\n").collect();
    for item in items.iter() {
        let len = item.len();
        if len == 0 {
            continue;
        }
        let first = item.get(..len / 2).unwrap();
        let second = item.get(len / 2..).unwrap();
        let mut first_set: HashSet<char> = HashSet::new();
        let mut second_set: HashSet<char> = HashSet::new();
        for c in first.chars() {
            first_set.insert(c);
        }
        for c in second.chars() {
            second_set.insert(c);
        }
        for c in first_set.intersection(&second_set) {
            let ch = u32::from(*c);
            if c.is_ascii_uppercase() {
                sum += ch - A + 1 + 26;
            } else {
                sum += ch - a + 1;
            }
        }
    }
    println!("Result: {}", sum);
}
