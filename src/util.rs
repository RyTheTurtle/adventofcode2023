use std::fs::read_to_string;
use std::path::Path;


pub fn read_lines<P: AsRef<Path>>(p: P) -> Vec<String> {
    read_to_string(p)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

pub fn partiiton_on_empty(v: &Vec<String>) -> Vec<Vec<String>> { 
    let mut result: Vec<Vec<String>> = Vec::new();
    let mut current_partition: Vec<String> = Vec::new();
    let mut iter = v.iter();
    loop { 
        match iter.next() { 
            Some(l) if l.trim() != "" => {
                current_partition.push(l.to_string());
            },
            Some(l) if l.trim() == "" => { 
                result.push(current_partition.clone());
                current_partition.clear();
            },
            None if current_partition.len() > 0 => { 
                result.push(current_partition);
                break;
            },
            _ => break
        }
    }
    result
}

// turns something like "x: 1 2 3" in to [1,2,3]
pub fn parse_number_vec_following_colon(s: &String) -> Vec<u64> {
    s.split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect()
}
