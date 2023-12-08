use crate::util;
 
pub fn part_1(v: &Vec<String>) -> u32 {
    v.iter().map(get_calibration_value).sum()
}

pub fn part_2(v: &Vec<String>) -> u32 {
    v.iter().map(get_calibration_value_v2).sum()
}

fn get_calibration_value(s: &String) -> u32 {
    const BLANK: char = '-';
    let mut digits: (char, char) = (BLANK, BLANK);
    for c in s.chars() {
        if c.is_digit(10) {
            if digits.0 == BLANK {
                digits.0 = c;
            }
            digits.1 = c;
        }
    }
    let res = combine_digits(&digits);
    res
}

fn combine_digits(digits: &(char, char)) -> u32 {
    match digits.0.to_digit(10) {
        Some(d) => match digits.1.to_digit(10) {
            Some(d2) => d * 10 + d2,
            None => panic!("Invalid second digit"),
        },
        None => panic!("Invalid first digit"),
    }
}

fn get_calibration_value_v2(s: &String) -> u32 {
    const BLANK: char = '-';
    let mut digits: (char, char) = (BLANK, BLANK);
    digits.0 = get_first_digit_from_text(s.as_bytes()).unwrap();
    // same logic but working backwords through the string
    for pos in (0..s.len()).rev() {
        match get_first_digit_from_text(s[pos..].as_bytes()) {
            None => {}
            Some(d) => digits.1 = d,
        }
        if digits.1 != BLANK {
            break;
        }
    }
    let res = combine_digits(&digits);
    res
}

fn get_first_digit_from_text(s: &[u8]) -> Option<char> {
    // (spelling, digit representation)
    // Instead of adding conditionals for doing different logic for finding
    // an ascii alphabetic character vs an ascii numeric character, I just
    // added the string representations that are actually numbers. This simplifies
    // the processing
    let digits = [
        ("one", '1'),
        ("1", '1'),
        ("two", '2'),
        ("2", '2'),
        ("three", '3'),
        ("3", '3'),
        ("four", '4'),
        ("4", '4'),
        ("five", '5'),
        ("5", '5'),
        ("six", '6'),
        ("6", '6'),
        ("seven", '7'),
        ("7", '7'),
        ("eight", '8'),
        ("8", '8'),
        ("nine", '9'),
        ("9", '9'),
    ];

    let mut pos = 0;
    let mut max_pos = s.len();
    while pos < max_pos {
        for digit in digits {
            if s[pos..].starts_with(digit.0.as_bytes()) {
                return Some(digit.1);
            }
        }
        pos += 1;
    }
    return None;
}
