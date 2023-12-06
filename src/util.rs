use std::fs::read_to_string;
use std::path::Path;

pub fn read_lines<P: AsRef<Path>>(p: P) -> Vec<String> {
    read_to_string(p)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

#[derive(Debug)]
pub struct Range(pub u64, pub u64);


pub fn get_overlap(a: &Range, b: &Range) -> Option<Range> { 
    let mut result = Range(0,0);

    // if a overlaps b, easy. Otherwise, just flip inputs for sanity 
    if a.0 > b.1  || b.1 < a.0 {
        return None; 
    }  

    if a.0 >= b.0 && a.0 < b.1 { 
        result.0 = a.0; 
        if a.1 < b.1 { 
            result.1 = a.1 
        } else {
            result.1 = b.1 
        }
    } else {
       result = get_overlap(b, a).unwrap();
    }

    Some(result)
}