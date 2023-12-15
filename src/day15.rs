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
    let instructions: Vec<(u64, LensOp, u64)> = input //convert input in to a list of instructions
        .get(0)
        .expect("should have one input line")
        .split(",")
        // convert each instruction in to (label, op, optional length)
        .map(get_instruction)
        .collect();
    for i in instructions {
        println!("Instruction: {:?}", i);
    }
    0
}
#[derive(Debug)]
enum LensOp {
    REMOVE,
    ADD,
}

fn get_instruction(i: &str) -> (u64, LensOp, u64) {
    let re = Regex::new("^([a-zA-Z]+)([=-]){1}([0-9]?)").unwrap();
    match re.captures(i).map(|c| c.extract::<3>()) {
        Some((_, [label, op, lense])) => {
            let lbl_hash = holiday_hash(&String::from(label).chars().collect(), 0);
            let op = match op {
                "=" => LensOp::ADD,
                _ => LensOp::REMOVE,
            };
            let lense = lense.parse::<u64>().unwrap_or_default();
            return (lbl_hash, op, lense);
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
