use crate::structs::grid::{from, Grid, GridCoordinate};
use std::collections::HashSet;

// consts
const ADJ_CELL_DELTAS: [GridCoordinate; 8] = [
    GridCoordinate(1, -1),
    GridCoordinate(1, 0),
    GridCoordinate(1, 1),
    GridCoordinate(0, -1),
    GridCoordinate(0, 1),
    GridCoordinate(-1, -1),
    GridCoordinate(-1, 0),
    GridCoordinate(-1, 1),
];
const BLANK: char = '.';
const GEAR: char = '*';

pub fn part_1(input: &Vec<String>) -> u32 {
    // build a grid from the input and
    // scan grid for indices of symbols
    let grid = from(input);
    let symbol_coordinates = find(&grid, |c| c != &BLANK && c.to_digit(10) == None);

    let mut partial_part_number: HashSet<GridCoordinate> = HashSet::new();
    for symbol in symbol_coordinates {
        partial_part_number.extend(get_partial_part_numbers(symbol, &grid).iter());
    }

    let mut numbers: Vec<Vec<GridCoordinate>> = Vec::new();
    for partial in partial_part_number {
        numbers.push(get_full_number_coordinates(partial, &grid));
    }

    // dedup number points making sure a number wasn't seen multiple times by different symbols
    let mut seen: HashSet<GridCoordinate> = HashSet::new();
    let mut result = 0;

    for number in numbers {
        // verify number hasn't been seen yet
        let mut is_new_number = true;
        let mut value = 0;

        for coordinate in number {
            if seen.contains(&coordinate) {
                is_new_number = false;
                break;
            } else {
                seen.insert(coordinate);
                value = value * 10 + get(&grid, &coordinate).unwrap().to_digit(10).unwrap();
            }
        }
        if is_new_number {
            result += value;
        }
    }
    result
}

pub fn part_2(input: &Vec<String>) -> u64 {
    // build a grid from the input and
    // scan grid for indices of symbols
    let grid = from(input);
    let gear_coordinates: Vec<GridCoordinate> = find(&grid, |c| c == &GEAR);

    let mut result: u64 = 0;
    for gear in gear_coordinates {
        let partial_part_number = get_partial_part_numbers(gear, &grid);
        let mut numbers: Vec<Vec<GridCoordinate>> = Vec::new();

        for partial in partial_part_number {
            let number_points = get_full_number_coordinates(partial, &grid);
            numbers.push(number_points);
        }
        let numbers: HashSet<&Vec<GridCoordinate>> = HashSet::from_iter(numbers.iter());
        if numbers.len() == 2 {
            let mut gear_ratio: u64 = 1;
            for number in numbers {
                gear_ratio *= get_number_value(&grid, number);
            }
            result += gear_ratio;
        }
    }

    result
}

fn get_number_value(g: &Grid, c: &Vec<GridCoordinate>) -> u64 {
    let mut value: u64 = 0;
    for coord in c {
        value = value * 10 + get(g, coord).unwrap().to_digit(10).unwrap() as u64;
    }
    value
}

fn get_partial_part_numbers(gear: GridCoordinate, grid: &Grid) -> HashSet<GridCoordinate> {
    let mut partial_part_number: HashSet<GridCoordinate> = HashSet::new();

    for transform in ADJ_CELL_DELTAS {
        let check = GridCoordinate(gear.0 + transform.0, gear.1 + transform.1);

        match get(grid, &check) {
            Some(c) if c.is_digit(10) => {
                partial_part_number.insert(check);
            }
            Some(_c) => { /* no-op, not a number */ }
            None => { /* no-op, not a valid coordinate */ }
        }
    }
    partial_part_number
}

fn get_full_number_coordinates(partial: GridCoordinate, grid: &Grid) -> Vec<GridCoordinate> {
    let mut number_points: Vec<GridCoordinate> = Vec::new();
    number_points.push(partial);
    let mut dist = 1;
    loop {
        let next = GridCoordinate(partial.0, partial.1 + dist);
        match get(grid, &next) {
            Some(c) if c.is_numeric() => {
                number_points.push(next);
                dist += 1;
            }
            Some(_c) => break,
            None => break,
        }
    }
    dist = 1;
    loop {
        let next = GridCoordinate(partial.0, partial.1 - dist);
        match get(grid, &next) {
            Some(c) => {
                if c.is_numeric() {
                    number_points.push(next);
                    dist += 1;
                } else {
                    break;
                }
            }
            None => break,
        }
    }

    number_points.sort_by(|a, b| {
        return a.1.cmp(&b.1);
    });
    number_points
}

/**
 * Given a filter, return the grid coordinates that the filter function
 * returns true
 */
fn find(grid: &Grid, test: fn(&char) -> bool) -> Vec<GridCoordinate> {
    let mut symbol_coordinates: Vec<GridCoordinate> = Vec::new();

    for (r, row) in grid.rows.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if test(col) {
                symbol_coordinates.push(GridCoordinate(r as i32, c as i32));
            }
        }
    }
    symbol_coordinates
}

fn get<'a>(g: &'a Grid, c: &GridCoordinate) -> Option<&'a char> {
    match g.rows.get(c.0 as usize) {
        Some(r) => {
            return r.get(c.1 as usize);
        }
        None => {
            return None;
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;
    // FIXME Move to integration tests

    #[test]
    pub fn test_part1() {
        let input = util::read_lines("./input/3.txt");
        assert_eq!(part_1(&input), 521515);
    }

    #[test]
    pub fn test_part2() {
        let input = util::read_lines("./input/3.txt");
        assert_eq!(part_2(&input), 69527306);
    }
}
