use std::collections::HashSet;
use std::fs::read_to_string;
use std::path::Path;

pub fn read_lines<P: AsRef<Path>>(p: P) -> Vec<String> {
    read_to_string(p)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
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
