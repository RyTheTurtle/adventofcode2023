use std::fs::read_to_string;
use std::path::Path;

pub fn read_lines<P: AsRef<Path>>(p: P) -> Vec<String> {
    read_to_string(p)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

#[derive(Debug, Copy, Clone)]
pub struct Range(pub u64, pub u64);

// returns a range representing the intersection of range b on to a
pub fn intersect(a: &Range, b: &Range) -> Option<Range> { 
    let b_all_greater_than_a = b.0 >= a.1 ;
    let b_all_less_than_a = b.1 < a.0; 
    if b_all_greater_than_a || b_all_less_than_a { 
        return  None;
    }
    return Some(Range(b.0.max(a.0), b.1.min(a.1)));

}

pub fn differences(a: &Range, b: &Range) -> Vec<Range> { 
    let mut differences: Vec<Range> = Vec::new();
    match intersect(a, b) { 
        Some(_) => { 
            let lower_diff  = Range(b.0.min(a.0), b.0.max(a.0));
            let upper_diff = Range(b.1.min(a.1) + 1, b.1.max(a.1));
            differences.push(lower_diff);
            differences.push(upper_diff);
        },
        None => {
            differences.push(*a);
            differences.push(*b);
        }
    }
    
    differences
}

// turns something like "x: 1 2 3" in to [1,2,3]
pub fn parse_number_vec_following_colon(s: &String) -> Vec<u64> { 
    s.split(":").nth(1).unwrap().trim().split_ascii_whitespace().map(|n| n.parse::<u64>().unwrap()).collect()
}