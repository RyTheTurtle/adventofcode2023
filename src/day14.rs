use std::{fmt::Debug, collections::HashSet, time::Instant, ops::Index};

use itertools::Itertools;
use memoize::memoize;

use crate::util::print_block;

pub fn part_1(input: &Vec<String>) -> u64 {
    let mut dish = Dish::from(input.clone());
    dish = tilt_north(dish);
    dish.get_load() as u64
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
struct Dish {
    row_width: usize,
    cells: Vec<u8>,
}

impl From<Vec<String>> for Dish {
    fn from(value: Vec<String>) -> Self {
        Dish {
            row_width: value[0].len(),
            cells: value.iter().flat_map(|s| s.chars().map(|c| c as u8)).collect(),
        }
    }
}

impl Dish {
    // helper to print debugging info
    pub fn print_debug_layout(&self) {
        let cells: Vec<char> = self.cells.iter().map(|b| *b as char).collect();
        let mut cnt = 0;
        for c in cells {
            print!("{}", c);
            cnt += 1;
            if cnt >= self.row_width {
                println!("");
                cnt = 0;
            }
        }
    }

    pub fn get_load(&self) -> usize {
        let num_rows = self.cells.len() / self.row_width;
        let mut result = 0;
        // println!("Total rows: {}", num_rows);
        for i in 0..num_rows {
            let row = &self.cells[i * self.row_width..i * self.row_width + self.row_width];
            // println!("{:?}", String::from_utf8(row.to_vec()));
            let stones = row.iter().filter(|s| **s == ('O' as u8)).count();
            result += (stones * (num_rows - i));
        }
        return result;
    }
}

pub fn part_2(input: &Vec<String>) -> u64 {
    let mut dish: Dish = Dish::from(input.clone());
    let mut cache: HashSet<Dish> = HashSet::new();
    let total_iterations = 1_000_000_000;
    let mut cycle_size = 0;
    let mut numerator = 0;
    let start = Instant::now();
    // worst case, if we never had a cycle...
    let mut hist: Vec<Dish> = Vec::new();
    let mut last_idx = 0;
    let mut finished_loops = 0;
    for i in 0..total_iterations{ 
        println!("Processed {}, elapsed: {}ms", i, start.elapsed().as_millis()); 
        finished_loops = i;
        dish = cycle(dish);
        let cloned = dish.clone();
        if !cache.contains(&cloned){
            cache.insert(cloned);
            hist.push(dish.clone());
        } else {
            println!("Found a cycle at {}, cycle size {}", i, cache.len());
            last_idx = hist.iter().find_position(|d| **d == dish).unwrap().0;
            break;
        }
    }
    let result_idx = (last_idx) + (total_iterations-1-finished_loops) % (finished_loops - last_idx);
    hist.get(result_idx).unwrap().get_load() as u64
}

#[memoize]
fn cycle(dish: Dish) -> Dish {
    tilt_east(tilt_south(tilt_west(tilt_north(dish))))
}

// recursively shift cells up north using math
// until there's no more valid shifts to perform
fn tilt_north(dish: Dish) -> Dish {
    let mut dish = dish;
    let mut swapped = false;
    for i in dish.row_width..dish.cells.len() {
        let current = dish.cells[i];
        if current == 'O' as u8 {
            // compute row directly above
            let row_num = i / dish.row_width;
            let col_num = i % dish.row_width;
            // println!("Found rock at {:?} {:?}",row_num, col_num);
            // check if we can roll the stone one place north
            let row_above = row_num - 1;
            let above_idx = (row_above * dish.row_width + col_num);
            // println!("Checking above cell {:?},{:?} at index {:?}",row_above, col_num, above_idx);
            if dish.cells[above_idx] == '.' as u8 {
                // println!("Valid, rolling rock up");
                dish.cells.swap(above_idx, i);
                swapped = true;
            }
        }
    }
    if swapped { 
        return tilt_north(dish);
    }
    dish
}

// recursively shift cells down south using math
// until there's no more valid shifts to perform
fn tilt_south(dish: Dish) -> Dish {
    let mut dish = dish;
    let mut swapped = false;
    for i in (0..dish.cells.len() - dish.row_width).rev() {
        let current = dish.cells[i];
        if current == 'O' as u8 {
            // compute row directly below
            let row_num = i / dish.row_width;
            let col_num = i % dish.row_width;
            // println!("Found rock at {:?} {:?}",row_num, col_num);
            // check if we can roll the stone one place north
            let row_below = row_num + 1;
            let below_idx = (row_below * dish.row_width + col_num);
            // println!("Checking below cell {:?},{:?} at index {:?}",row_below, col_num, below_idx);
            if dish.cells[below_idx] == '.' as u8 {
                // println!("Valid, rolling rock up");
                dish.cells.swap(below_idx, i);
                swapped = true;
            }
        }
    }
    if swapped { 
        return tilt_south(dish);
    }
    dish
}

fn tilt_east(dish: Dish) -> Dish {
    let mut dish = dish;
    let mut swapped = false;
    for i in (0..dish.cells.len() - 1).rev() {
        let current = dish.cells[i];
        if current == 'O' as u8 {
            // compute how far left we can shift the cell
            let row_num = i / dish.row_width;
            let col_num = i % dish.row_width;
            // println!("Found rock at {:?} {:?}",row_num, col_num);
            let candidate_cell_idx = row_num * dish.row_width + (col_num + 1);
            if candidate_cell_idx / dish.row_width != row_num {
                //we were at the end of the line, don't count it
                continue;
            }
            if dish.cells[candidate_cell_idx] == '.' as u8 {
                dish.cells.swap(candidate_cell_idx, i);
                swapped = true;
            }
        }
    }
    if swapped {
        return tilt_east(dish);
    }
    dish
}

fn tilt_west(dish: Dish) -> Dish {
    let mut dish = dish;
    let mut swapped = false;
    for i in (1..dish.cells.len()).rev() {
        let current = dish.cells[i];
        if current == 'O' as u8 {
            // compute how far left we can shift the cell
            let row_num = i / dish.row_width;
            let col_num = i % dish.row_width;
            if col_num == 0 {
                continue; // can't shift something further west than col 0
            }
            // println!("Found rock at {:?} {:?}",row_num, col_num);
            let candidate_cell_idx = row_num * dish.row_width + (col_num - 1);
            if dish.cells[candidate_cell_idx] == '.' as u8 {
                dish.cells.swap(candidate_cell_idx, i);
                swapped = true;
            }
        }
    }
    if swapped {
        return tilt_west(dish);
    }
    dish
}
