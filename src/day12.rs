use memoize::memoize;
use regex::{self, Regex};
use std::{collections::HashSet, fmt::Error};

use crate::structs::camel_card::get;

/**
 * we know backtracing brute force won't work for large scale.
 */
pub fn part_1(input: &Vec<String>) -> u64 {
    let mut result: u64 = 0;
    for line in input {
        let parts: Vec<&str> = line.split(" ").collect();
        let scheme = parts[0].clone().to_string();
        let counts: Vec<usize> = parts[1]
            .clone()
            .to_string()
            .split(",")
            .map(|c| c.parse::<usize>().unwrap())
            .collect();
        let arrangements = get_arrangements(scheme, counts).unwrap();
        result += arrangements;
        println!("{:?} = {:?}", line, arrangements);
    }
    return result;
}

pub fn part_2(input: &Vec<String>) -> u64 {
    0
}

// followed https://www.youtube.com/watch?v=g3Ms5e7Jdqo for explanation
#[memoize]
fn get_arrangements(input: String, nums: Vec<usize>) -> Result<u64, regex::Error> {
    println!("input: {:?} {:?}", input, nums);
    if nums.len() == 0 {
        if !input.contains('#') {
            return Ok(1);
        } else {
            return Ok(0);
        }
    }
    let broken_re = Regex::new("([#?])+")?;
    let mut result = 0;
    let expected_springs = nums[0];
    match broken_re.find(&input) {
        Some(m) => {
            let arrangements = validate_arrangements(m.as_str().to_string(), expected_springs);
            println!("\t{:?} arrangements: {:?}", m, arrangements);
            result += arrangements;
            result += get_arrangements(
                input[m.range().start + expected_springs..].to_string(),
                nums[1..].to_vec(),
            )
            .unwrap();
        }
        None => {
            return Ok(result);
        }
    }

    return Ok(result);
}

// this will probably be needed in part 2
#[memoize]
fn validate_arrangements(input: String, num: usize) -> u64 {
    // println!("validate arrangement input: {:?} , {:?}",input,num);
    let broken_re = Regex::new("([#])+").unwrap();
    match input.find("?") {
        None => {
            match broken_re.find(&input) {
                Some(m) if m.len() == num => {
                    // println!("Found a valid combination {:?}", input);
                    return 1;
                }
                _ => {
                    // println!("Found a invalid combination {:?}", input);
                    return 0;
                }
            }
        }
        Some(i) => {
            return validate_arrangements(input.replacen("?", "#", 1), num)
                + validate_arrangements(input.replacen("?", ".", 1), num);
        }
    }
}
