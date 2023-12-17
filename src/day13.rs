

use crate::util::partiiton_on_empty;

pub fn part_1(input: &Vec<String>) -> u64 {
    let mut result = 0;
    let mut no_reflection_cnt = 0;
    let patches = partiiton_on_empty(input);
    for patch in patches { 
        match get_vertical_line_reflection(&patch, 0, 1) {
            Some(p) => {
                result += p as u64;
            },
            None => match get_horizontal_line_reflection(&patch, 0, 1) {
                Some(p) => {
                    result += (100 * p) as u64;
                }
                None => {
                    // println!("No reflection found for {:?}",patch);
                    println!("No reflection found for ");
                    print_block(&patch);
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

fn get_vertical_line_reflection(patch: &Vec<String>, left_straddle: usize, right_straddle: usize) -> Option<usize> { 
    let patch_width = patch[0].len();
    if right_straddle >= patch_width {
        return None;
    }
    let mut is_reflection = true; 
    'dist: for distance in 0..patch_width { 
        // bounds check
        if left_straddle < distance || right_straddle + distance >= patch_width {
            break;
        }
        for line in patch { 
            let c1 = line.chars().nth(left_straddle-distance).unwrap();
            let c2 = line.chars().nth(right_straddle+distance).unwrap();
            if c1 != c2 { 
                is_reflection = false;
                continue 'dist;
            }
        }
    }
    if is_reflection { 
        return Some(right_straddle);
    }
    return get_vertical_line_reflection(patch, left_straddle+1, right_straddle+1);
}

fn get_horizontal_line_reflection(patch: &Vec<String>, top_straddle: usize, bottomn_straddle: usize) -> Option<usize> {
    let patch_len = patch.len();
    if bottomn_straddle >= patch_len {
        return None; 
    }
    let mut is_reflection = true; 
    for distance in 0..patch_len {
        // bounds check 
        if top_straddle < distance || distance + bottomn_straddle >= patch_len {
            break;
        }
        let top = patch.get(top_straddle - distance).unwrap();
        let bottom = patch.get(bottomn_straddle+distance).unwrap();
        if top != bottom { 
            is_reflection = false;
            break;
        } 
    }
    if is_reflection{ 
        return Some(bottomn_straddle);
    }
    return get_horizontal_line_reflection(patch, top_straddle+1, bottomn_straddle+1);
}

fn print_block(s: &Vec<String>){
    println!("Block:::");
    for line in s { 
        println!("{}",line);
    }
    println!(":::")
}