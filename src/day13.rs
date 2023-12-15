

use crate::util::partiiton_on_empty;

pub fn part_1(input: &Vec<String>) -> u64 {
    let mut result = 0;
    let mut no_reflection_cnt = 0;
    let patches = partiiton_on_empty(input);
    for patch in patches { 
        match get_horizontal_reflection(&patch) {
            Some(p) => {
                result += p as u64;
            }
            None => match get_vertical_reflection(&patch) {
                Some(p) => {
                    result += (100 * p) as u64;
                }
                None => {
                    // println!("No reflection found for {:?}",patch);
                    no_reflection_cnt += 1; 
                }
            },
        }
    }
    println!("Total patches with no reflection: {:?}", no_reflection_cnt);
    result
}

pub fn part_2(_input: &Vec<String>) -> u64 {
    0
}


fn get_vertical_reflection(input: &Vec<String>) -> Option<usize> {
    let mut pivot = 1;
    // split input in half on a horizontal line and
    // check to see if both sides are equal. Progressively
    // shorten the pivot line until we run out of possible reflections
    while pivot < input.len() {
        // we need to do this check to make sure we do the proper 
        // math for finding partial reflections either above or below 
        // the halfway point
        if pivot > input.len() / 2 {
            let mut bottom = input[pivot..].iter();
            let mut top = input[pivot - bottom.len()..pivot].iter().rev();
            if top.all(|t| t == bottom.next().unwrap()) {
                return Some(pivot);
            }
        } else {
            let mut top = input[..pivot].iter().rev();
            let mut bottom = input[pivot..pivot+top.len()].iter();
            
            if top.all(|t| t == bottom.next().unwrap()) {
                return Some(pivot);
            }
        }
        
        pivot+= 1;
    }

    None
}

fn get_horizontal_reflection(input: &Vec<String>) -> Option<u64> {
    let pivot =  1;

    return match get_horizontal_pivot(input.get(0).expect("input should not be empty"), pivot) {
        Some(p) => {
            if input
                .iter()
                .all(|i| get_horizontal_pivot(i, pivot) == Some(p))
            {
                Some(p as u64)
            } else {
                None
            }
        }
        None => None,
    };
}

fn get_horizontal_pivot(input: &String, pivot: usize) -> Option<usize> {
    if pivot >= input.len() {
        return None;
    }
    let right: &str;
    let left: &str; 
    if pivot > input.len() / 2 {
        right = &input[pivot..];
        left = &input[pivot - right.len()..pivot];
    } else {
        left = &input[..pivot];
        right = &input[pivot..pivot+left.len()+1];
        
    }
    
    return match String::from(right)
        == String::from(left)
            .chars()
            .rev()
            .collect::<String>()
    {
        true => Some(pivot),
        false => get_horizontal_pivot(input, pivot + 1),
    }
}
