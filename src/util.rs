use std::collections::HashSet;
use std::fs::read_to_string;
use std::path::Path;

use crate::day5::Mapping;

pub fn read_lines<P: AsRef<Path>>(p: P) -> Vec<String> {
    read_to_string(p)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Range(pub u64, pub u64);

pub fn new_range(a: u64, b: u64) -> Range {
    Range(a.min(b), a.max(b))
}

// returns the intersect, if any, of the range
pub fn intersect(a: &Range, b: &Range) -> Option<Range> {
    if a == b {
        return Some(a.clone());
    }

    let b_starts_in_a = b.0 >= a.0 && b.0 < a.1;
    let b_ends_in_a = b.1 <= a.1 && b.1 > a.0;
    let a_starts_in_b = a.0 >= b.0 && a.0 < b.1;
    let a_ends_in_b = a.1 <= b.1 && a.1 > b.0;

    if b_starts_in_a && b_ends_in_a {
        return Some(b.clone());
    } else if a_starts_in_b && a_ends_in_b {
        return Some(a.clone());
    } else if b_starts_in_a && !b_ends_in_a {
        return Some(new_range(b.0, a.1));
    } else if a_starts_in_b && !a_ends_in_b {
        return Some(new_range(a.0, b.1));
    }

    None
}

pub fn diff_lower(a: &Range, b: &Range) -> Option<Range> {
    let b_starts_in_a = b.0 >= a.0 && b.0 < a.1;
    let a_starts_in_b = a.0 >= b.0 && a.0 < b.1;

    if b_starts_in_a {
        return Some(new_range(a.0, b.0));
    } else if a_starts_in_b {
        return Some(new_range(b.0, a.0));
    } else {
        return Some(a.min(b).clone());
    }
}

pub fn diff_upper(a: &Range, b: &Range) -> Option<Range> {
    let b_ends_in_a = b.1 <= a.1 && b.1 > a.0;
    let a_ends_in_b = a.1 <= b.1 && a.1 > b.0;

    if a.1 == b.1{ return None } else if b_ends_in_a {
        return Some(new_range(b.1+1, a.1));
    } else if a_ends_in_b {
        return Some(new_range(a.1+1, b.1));
    } else { 
        return Some(a.max(b).clone())
    }

}

pub fn get_outputs(m: &Mapping, sources: &HashSet<Range>) -> HashSet<Range>  {
    // populate stack
    let mut range_stack: Vec<Range> = Vec::new();
    let mut already_processed: HashSet<Range> = HashSet::new();
    let mut results : HashSet<Range> = HashSet::new();
    for range in sources.clone() { 
        range_stack.push(range);
    }
    // process items off the stack 
    while range_stack.len() > 0 { 
        let current_range = range_stack.pop().unwrap();
        match already_processed.get(&current_range){
            Some(r) => {continue},
            None => {already_processed.insert(current_range);}
        }
        // println!("current range:  {:?}", current_range);
        // println!("range_stack:  {:?}", range_stack);
        let mut found_intersection = false;
        for mr in &m.ranges {
            let map_range = new_range(mr.src_start, mr.src_start+mr.range);
            // println!("Checking map range {:?}", map_range);
            let intersection = intersect(&map_range, &current_range);
            let lower_diff = diff_lower(&map_range, &current_range);
            let upper_diff = diff_upper(&map_range, &current_range);
            match intersection { 
                Some(r) => { 
                    // intersection is mapped to destination result 
                    // println!("Found intersection {:?}",r);
                    found_intersection = true;
                    let dist = r.1-r.0; 
                    let offset = r.0 - mr.src_start;
                    let output_range = new_range(mr.dest_start+offset, mr.dest_start+offset+dist);
                    results.insert(output_range);
                    // any diffs are added back to stack for further processing 
                    match lower_diff {
                        Some(d) => {range_stack.push(d);},
                        None => {}
                    }
                    match upper_diff {
                        Some(d) => {range_stack.push(d);},
                        None => {}
                    } 

                    break;
                }, 
                None => { 
                    // no intersection means this range maps to itself for output 
                    continue;
                }
            }
        }
        if !found_intersection { 
            // range didn't map to any inputs
            results.insert(current_range);
        }
    } 
    results
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

#[cfg(test)]
mod tests {
    use crate::day5::MapRange;

    use super::*;

    #[test]
    pub fn new_range_right_order() {
        assert_eq!(new_range(4, 2), Range(2, 4));
    }

    #[test]
    pub fn test_range_ordering() {
        assert!(Range(2, 3) < Range(2, 4));
        assert!(Range(2, 3) == Range(2, 3));
        assert!(Range(1, 3) < Range(2, 5));
        assert!(Range(1, 5) > Range(0, 5));
    }

    #[test]
    pub fn test_intersect() {
        let test_cases = [
            (Range(1, 5), Range(3, 5), Some(Range(3, 5))),
            (Range(98, 99), Range(79, 93), None),
            (Range(1, 5), Range(3, 7), Some(Range(3, 5))),
            (Range(3, 7), Range(1, 5), Some(Range(3, 5))),
        ];

        for case in test_cases {
            assert_eq!(intersect(&case.0, &case.1), case.2);
        }
    }

    #[test]
    pub fn test_diff_lower(){
        let test_cases = [
            (Range(1, 5), Range(3, 5), Some(Range(1, 3))),
            (Range(98, 99), Range(79, 93), Some(Range(79,93))),
            (Range(1, 5), Range(3, 7), Some(Range(1, 3))),
            (Range(3, 7), Range(1, 5), Some(Range(1, 3))),
        ];

        for case in test_cases {
            assert_eq!(diff_lower(&case.0, &case.1), case.2);
        }
    }

    #[test]
    pub fn test_diff_upper(){
        let test_cases = [
            (Range(1, 5), Range(3, 5), None),
            (Range(98, 99), Range(79, 93), Some(Range(98,99))),
            (Range(1, 5), Range(3, 7), Some(Range(6,7))),
            (Range(3, 7), Range(1, 5), Some(Range(6,7))),
            (Range(79, 93), Range(56, 93), None),
            
        ];

        for case in test_cases {
            assert_eq!(diff_upper(&case.0, &case.1), case.2);
        }
    }

    #[test]
    pub fn test_get_destinations_exact_match(){ 
        // TODO implement tests
        
        // let input_map = Mapping{title: String::from("test"), vec![MapRange()]};
    }
}