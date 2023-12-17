use itertools::Itertools;

use crate::util::print_block;

pub fn part_1(input: &Vec<String>) -> u64 {
    get_columns_tilted_north(&mut input.clone())
        .iter()
        .map(compute_column_load)
        .sum::<usize>() as u64
}

pub fn part_2(_input: &Vec<String>) -> u64 {
    0
}

fn get_columns_tilted_north(input: &mut Vec<String>) -> Vec<Vec<u8>> {
    let column_size = input.get(0).expect("input should not be empty").len();
    let mut columns: Vec<Vec<u8>> = Vec::new();
    for col in 0..column_size {
        let mut col_chars = get_col(input, col);
        tilt_to_front(&mut col_chars);
        columns.push(col_chars);
    }
    columns
}

fn get_col(input: &Vec<String>, col: usize) -> Vec<u8> {
    input.iter().map(|s| s.chars().nth(col).unwrap()).map(|c| c as u8).collect()
}

fn is_rounded_rock(i: u8) -> bool {
    i == 'O' as u8
}

// recursively process vector until impossible to shift left anymore
// for any cells
fn tilt_to_front(input: &mut Vec<u8>) {
    for i in 1..input.len() {
        if is_rounded_rock(input[i]) && input[i - 1] == '.' as u8 {
            input.swap(i - 1, i);
            tilt_to_front(input);
        }
    }
}

fn compute_column_load(input: &Vec<u8>) -> usize {
    let mut result = 0;
    for (idx, val) in input.iter().enumerate() {
        if is_rounded_rock(*val) {
            result += (input.len() - idx);
        }
    }
    result
}
