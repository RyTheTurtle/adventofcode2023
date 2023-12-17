use crate::util::partiiton_on_empty;

pub fn part_1(input: &Vec<String>) -> u64 {
    let mut result = 0;
    let mut no_reflection_cnt = 0;
    let patches = partiiton_on_empty(input);
    for patch in patches {
        result += get_reflection_summary(&patch, usize::MAX).1 as u64;
    }
    println!("Total patches with no reflection: {:?}", no_reflection_cnt);
    result
}

pub fn part_2(input: &Vec<String>) -> u64 {
    let mut result = 0;
    let patches = partiiton_on_empty(input);
    for (i, patch) in patches.iter().enumerate() {
        let original_reflection = get_reflection_summary(&patch, usize::MAX);
        let smudge_fixes = get_possible_smudges(&patch);
        let mut fixed = false;

        for sf in smudge_fixes {
            let smudge_fix = get_reflection_summary(&sf, original_reflection.1);
            if smudge_fix.1 > 0 {
                result += smudge_fix.1 as u64;
                fixed = true;
                break;
            }
        }
        if !fixed {
            println!("No fix found at index {}!", i);
            print_block(&patch);
            println!("-----------------")
        }
    }
    result as u64
}

// generate possible smudges for a particular patch
fn get_possible_smudges(patch: &Vec<String>) -> Vec<Vec<String>> {
    let mut result: Vec<Vec<String>> = Vec::new();
    // rebuild the patch swapping one char at a time. Painstakingly slow
    // but gets around the borrow checker for now
    let joined = patch.join(" ");
    for i in 0..joined.len() {
        match joined.chars().nth(i) {
            Some(c) if c == ' ' => continue,
            Some(c) if c == '.' => {
                let mut cloned_patch = joined.clone();
                cloned_patch.replace_range(i..i + 1, "#");
                result.push(cloned_patch.split(" ").map(|s| s.to_string()).collect());
            }
            Some(c) if c == '#' => {
                let mut cloned_patch = joined.clone();
                cloned_patch.replace_range(i..i + 1, ".");
                result.push(cloned_patch.split(" ").map(|s| s.to_string()).collect());
            }
            _ => break,
        }
    }
    return result;
}

fn get_reflection_summary(patch: &Vec<String>, exclude: usize) -> (usize, usize) {
    let mut vertical = match get_vertical_line_reflection(&patch, 0, 1, exclude) {
        Some(p) => p,
        None => 0,
    };
    // don't exclude vertical if the exclude is from a horizontal
    if exclude >= 100 {
        vertical = match get_vertical_line_reflection(&patch, 0, 1, usize::MAX) {
            Some(p) => p,
            None => 0,
        };
    }

    let mut horizontal = match get_horizontal_line_reflection(&patch, 0, 1, exclude) {
        Some(p) => p,
        None => 0,
    };
    // don't exclude horizontal if the exclude is from a vertical
    if exclude < 100 {
        horizontal = match get_horizontal_line_reflection(&patch, 0, 1, usize::MAX) {
            Some(p) => p,
            None => 0,
        };
    }

    if horizontal > 0 {
        return (horizontal, horizontal * 100);
    }
    return (vertical, vertical);
}

fn get_vertical_line_reflection(
    patch: &Vec<String>,
    left_straddle: usize,
    right_straddle: usize,
    exclude: usize,
) -> Option<usize> {
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
            let c1 = line.chars().nth(left_straddle - distance).unwrap();
            let c2 = line.chars().nth(right_straddle + distance).unwrap();
            if c1 != c2 {
                is_reflection = false;
                continue 'dist;
            }
        }
    }
    if is_reflection && right_straddle != exclude {
        return Some(right_straddle);
    }
    return get_vertical_line_reflection(patch, left_straddle + 1, right_straddle + 1, exclude);
}

fn get_horizontal_line_reflection(
    patch: &Vec<String>,
    top_straddle: usize,
    bottomn_straddle: usize,
    exclude: usize,
) -> Option<usize> {
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
        let bottom = patch.get(bottomn_straddle + distance).unwrap();
        if top != bottom {
            is_reflection = false;
            break;
        }
    }
    if is_reflection && bottomn_straddle != (exclude / 100) {
        return Some(bottomn_straddle);
    }
    return get_horizontal_line_reflection(patch, top_straddle + 1, bottomn_straddle + 1, exclude);
}

fn get_all_horizontal_reflection(patch: &Vec<String>) -> Vec<usize> {
    let mut result = Vec::new();
    for i in 0..patch.len() {
        match get_horizontal_line_reflection(patch, 0, 1, i) {
            Some(p) => result.push(p),
            _ => continue,
        }
    }
    return result;
}

fn get_all_vertical_reflection(patch: &Vec<String>) -> Vec<usize> {
    let mut result = Vec::new();
    for i in 0..patch.len() {
        match get_vertical_line_reflection(patch, 0, 1, i) {
            Some(p) => result.push(p),
            _ => continue,
        }
    }
    return result;
}

fn print_block(s: &Vec<String>) {
    println!("Block:::");
    for line in s {
        println!("{}", line);
    }
    println!(":::")
}
