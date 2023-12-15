use std::vec;

use itertools::Itertools;
use regex::Regex;

pub fn part_1(input: &Vec<String>) -> u64 {
    // we know it's one long line
    input
        .get(0)
        .expect("should have one input line")
        .split(",")
        .into_iter()
        .map(|i| holiday_hash(&i.chars().collect::<Vec<char>>(), 0))
        .sum::<u64>()
}

pub fn part_2(input: &Vec<String>) -> u64 {
    let instructions: Vec<(String, u64, LensOp, u64)> = input //convert input in to a list of instructions
        .get(0)
        .expect("should have one input line")
        .split(",")
        // convert each instruction in to (label, op, optional length)
        .map(get_instruction)
        .collect();
    // we have a maximum of 256 cells 
    let mut boxes: Vec<Vec<Lense>> = Vec::new(); 
    for _ in 0..256 { 
        boxes.push(Vec::new());
    }
    for i in instructions { 
        match i.2 { 
            LensOp::ADD => { 
                match boxes.iter_mut().nth(i.1 as usize) { 
                    Some(b) => { 
                        match b.iter_mut().find(|l| l.lbl == i.0) { 
                            Some(mut l)=> {
                                // just swap out the focal lense 
                                l.focal_len = i.3;
                            },
                            None => {
                                b.push( Lense { lbl: i.0, focal_len: i.3 });
                            }
                        }
                    },
                    None => {panic!("Invalid boxid");}
                }
            },

            LensOp::REMOVE => { 
                match boxes.iter_mut().nth(i.1 as usize) { 
                    Some(b) => { 
                        match b.iter_mut().position(|l| l.lbl == i.0) { 
                            Some(mut l)=> {
                                // just swap out the focal lense 
                                b.remove(l);
                            },
                            None => {/* no op */}
                        }
                    },
                    None => {panic!("Invalid boxid");}
                }
            
            }
        }
    }
    let mut total_focusing_power: u64 = 0;
    for (i,b) in boxes.iter().enumerate() { 
        if b.len() > 0 { 
            println!("Box {:?} {:?}",i,b); 
        }
        let box_nr = (i + 1 )as u64;
        for (j,lense) in b.iter().enumerate() {
            let lense_slot: u64 = (j+1 )as u64;
            let focal = lense.focal_len; 
            total_focusing_power += (box_nr * lense_slot * focal) as u64;
        }

    }
    total_focusing_power
}
#[derive(Debug)]
enum LensOp {
    REMOVE,
    ADD,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Lense { 
    lbl: String , 
    focal_len: u64
}

fn get_instruction(i: &str) -> (String, u64, LensOp, u64) {
    let re = Regex::new("^([a-zA-Z]+)([=-]){1}([0-9]?)").unwrap();
    match re.captures(i).map(|c| c.extract::<3>()) {
        Some((_, [label, op, lense])) => {
            let lbl_hash = holiday_hash(&String::from(label).chars().collect(), 0);
            let op = match op {
                "=" => LensOp::ADD,
                _ => LensOp::REMOVE,
            };
            let lense = lense.parse::<u64>().unwrap_or_default();
            return (String::from(label), lbl_hash, op, lense);
        }
        None => panic!("Pattern did not match"),
    }
}
fn holiday_hash(input: &Vec<char>, result: u64) -> u64 {
    match input.get(0) {
        None => {
            return result;
        }
        Some(c) => {
            let mut updated_result = result + *c as u64;
            updated_result *= 17 as u64;
            updated_result %= 256;
            return holiday_hash(&input[1..].to_vec(), updated_result);
        }
    }
}
