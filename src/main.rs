use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let in_file = &args[1];
    let in_data = fs::read_to_string(in_file).unwrap();

    let a = "a".chars().nth(0).unwrap() as u32;
    let cap_a = "A".chars().nth(0).unwrap() as u32;
    println!("a val: {} -- A val: {}", a, cap_a);
    let mut sum: u32 = 0;
    let mut count: u32 = 0;
    let mut elves: Vec<&str> = Vec::new();
    let items: Vec<&str> = in_data.split("\n").collect();
    for item in items.iter() {
        elves.push(item);
        count += 1;
        if count == 3 {
            let mut first_set: HashSet<char> = HashSet::new();
            let mut second_set: HashSet<char> = HashSet::new();
            let mut third_set: HashSet<char> = HashSet::new();
            for (i, s) in elves.iter().enumerate() {
                if i == 0 {
                    for c in s.chars() {
                        first_set.insert(c);
                    }
                } else if i == 1 {
                    for c in s.chars() {
                        second_set.insert(c);
                    }
                } else if i == 2 {
                    for c in s.chars() {
                        third_set.insert(c);
                    }
                }
            }
            for c in &(&first_set & &second_set) & &third_set {
                let ch = u32::from(c);
                println!("char: {}", c);
                if c.is_ascii_uppercase() {
                    sum += ch - cap_a + 1 + 26;
                } else {
                    sum += ch - a + 1;
                }
            }
            count = 0;
            elves.clear();
        }
    }
    println!("Result: {}", sum);
}
