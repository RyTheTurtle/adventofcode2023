use crate::structs::almanac::{Almanac, AlmanacRange, RangedAlmanac, MapRange ,Mapping};
use std::{
    collections::HashSet,
    time::{Instant},
};

pub fn part_1(input: &Vec<String>) -> u64 {
    let almanac = Almanac::from(input);
    let mut lowest: u64 = u64::MAX;
    for seed in almanac.seeds {
        let mut value = seed;
        for map in &almanac.maps {
            // println!("Finding {:?} in {:?}",value,map.title);
            value = next(&map, value);
            // println!("  Result: {:?}", value);
        }
        if value < lowest {
            lowest = value;
        }
    }

    lowest
}

// WIP. Originally brute forced, trying to get range math working correctly.
/**
 * TODO implement stack based approach for getting all outputs of a map
 *
 *  range_stack = <starting ranges>
 *  results = hashset of ranges
 *  while range_stack is not empty
 *      pop a range off the stack
 *      iterate through map ranges until we find an intersection
 *      convert intersection range to map range's destination range
 *      put "remainders" , if any, back on the stack
 */
pub fn part_2(input: &Vec<String>) -> u64 {
    let ranged_almanac: RangedAlmanac = RangedAlmanac::from(input);
    let start = Instant::now();
    let mut ranges: HashSet<AlmanacRange> = HashSet::new();
    // set the initial ranges as the seed ranges from the almanac
    for seed in ranged_almanac.seeds {
        ranges.insert(seed);
    }
    let mut almanac_map_iter = ranged_almanac.maps.iter();
    loop {
        match almanac_map_iter.next() {
            Some(m) => {
                println!("part_2: Map {:?}", m);
                println!("- processing ranges {:?}", ranges);
                ranges = get_outputs(&m, &ranges);
            }
            None => {
                break;
            }
        }
    }
    println!("Evaluated answer in {:?} ms", start.elapsed().as_millis());
    println!("Ranges: {:?}", ranges);
    return ranges.into_iter().min().unwrap().0;
}


fn is_in_range(r: &MapRange, n: u64) -> bool {
    n >= r.src_start && n < r.src_start + r.range
}

fn get_dest(r: &MapRange, n: u64) -> u64 {
    if !is_in_range(r, n) {
        return n;
    }
    let offset = n - r.src_start;
    r.dest_start + offset
}

fn next(m: &Mapping, n: u64) -> u64 {
    let range: Vec<&MapRange> = m
        .ranges
        .iter()
        .filter(|r| is_in_range(r, n))
        .collect();
    match range.get(0) {
        Some(r) => get_dest(r, n),
        None => n,
    }
}


// returns the intersect, if any, of the range
pub fn intersect(a: &AlmanacRange, b: &AlmanacRange) -> Option<AlmanacRange> {
    if a == b {
        return Some(a.clone());
    }

    let b_starts_in_a = b.0 >= a.0 && b.0 < a.1;
    let b_ends_in_a = b.1 <= a.1 && b.1 > a.0;
    let a_starts_in_b = a.0 >= b.0 && a.0 < b.1;
    let a_ends_in_b = a.1 <= b.1 && a.1 > b.0;

    if b_starts_in_a && b_ends_in_a {
        return Some(b.clone());
    } else if a_starts_in_b && a_ends_in_b {
        return Some(a.clone());
    } else if b_starts_in_a && !b_ends_in_a {
        return Some(AlmanacRange::new(b.0, a.1));
    } else if a_starts_in_b && !a_ends_in_b {
        return Some(AlmanacRange::new(a.0, b.1));
    }

    None
}

pub fn diff_lower(a: &AlmanacRange, b: &AlmanacRange) -> Option<AlmanacRange> {
    match intersect(a, b) {
        None => {
            // no intersection, take the lower of the two input ranges
            return Some(a.min(b).clone());
        }
        _ => {
            let b_starts_in_a = b.0 >= a.0 && b.0 < a.1;
            let a_starts_in_b = a.0 >= b.0 && a.0 < b.1;

            if b_starts_in_a {
                return Some(AlmanacRange::new(a.0, b.0));
            } else if a_starts_in_b {
                return Some(AlmanacRange::new(b.0, a.0));
            } else {
                return Some(a.min(b).clone());
            }
        }
    }
}

pub fn diff_upper(a: &AlmanacRange, b: &AlmanacRange) -> Option<AlmanacRange> {
    match intersect(a, b) {
        Some(r) if r == *a || r == *b => {
            // full intersection, no lower or upper bound diff
            return None;
        }
        None => {
            // no intersection, take the lower of the two input ranges
            return Some(a.max(b).clone());
        }
        _ => {
            let b_ends_in_a = b.1 <= a.1 && b.1 > a.0;
            let a_ends_in_b = a.1 <= b.1 && a.1 > b.0;

            if a.1 == b.1 {
                return None;
            } else if b_ends_in_a {
                return Some(AlmanacRange::new(b.1 + 1, a.1));
            } else if a_ends_in_b {
                return Some(AlmanacRange::new(a.1 + 1, b.1));
            } else {
                return Some(a.max(b).clone());
            }
        }
    }
}

pub fn get_outputs(m: &Mapping, sources: &HashSet<AlmanacRange>) -> HashSet<AlmanacRange> {
    // populate stack
    let mut range_stack: Vec<AlmanacRange> = Vec::new();
    let mut already_processed: HashSet<AlmanacRange> = HashSet::new();
    let mut results: HashSet<AlmanacRange> = HashSet::new();
    for range in sources.clone() {
        range_stack.push(range);
    }
    // process items off the stack
    while range_stack.len() > 0 {
        let current_range = range_stack.pop().unwrap();
        // just in case we end up with some duplicate ranges showing up
        match already_processed.get(&current_range) {
            Some(r) => continue,
            None => {
                already_processed.insert(current_range);
            }
        }
        // println!("current range:  {:?}", current_range);
        // println!("range_stack:  {:?}", range_stack);
        let mut found_intersection = false;
        for mr in &m.ranges {
            let map_range = AlmanacRange::new(mr.src_start, mr.src_start + mr.range);
            // println!("Checking map range {:?}", map_range);
            let intersection = intersect(&map_range, &current_range);
            let lower_diff = diff_lower(&map_range, &current_range);
            let upper_diff = diff_upper(&map_range, &current_range);
            match intersection {
                Some(r) => {
                    // intersection is mapped to destination result
                    // println!("Found intersection {:?}",r);
                    found_intersection = true;
                    let dist = r.1 - r.0 - 1;
                    let offset = r.0 - mr.src_start;
                    let output_range =
                    AlmanacRange::new(mr.dest_start + offset, mr.dest_start + offset + dist);
                    results.insert(output_range);
                    // any diffs are added back to stack for further processing only if we don't have a full match for the input range
                    /**
                     *
                     * given two ranges
                     *  aaaaaa
                     *      bbbbbbb
                     * only take upper diff
                     *
                     *      aaaaaaaa
                     *   bbbbbbb
                     *
                     * only take lower diff
                     *
                     */
                    if current_range != r {
                        match lower_diff {
                            Some(d) if current_range.0 < map_range.0 => {
                                range_stack.push(d);
                            }
                            _ => {}
                        }
                        match upper_diff {
                            Some(d) if current_range.0 > map_range.0 => {
                                range_stack.push(d);
                            }
                            _ => {}
                        }
                    }

                    break;
                }
                None => {
                    // no intersection means this range maps to itself for output
                    continue;
                }
            }
        }
        if !found_intersection {
            // range didn't map to any inputs
            results.insert(current_range);
        }
    }
    results
}
// turns something like "x: 1 2 3" in to [1,2,3]
pub fn parse_number_vec_following_colon(s: &String) -> Vec<u64> {
    s.split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::day5::MapRange;

    use super::*;

    #[test]
    pub fn new_range_right_order() {
        assert_eq!(AlmanacRange::new(4, 2), AlmanacRange(2, 4));
    }

    #[test]
    pub fn test_range_ordering() {
        assert!(AlmanacRange(2, 3) < AlmanacRange(2, 4));
        assert!(AlmanacRange(2, 3) == AlmanacRange(2, 3));
        assert!(AlmanacRange(1, 3) < AlmanacRange(2, 5));
        assert!(AlmanacRange(1, 5) > AlmanacRange(0, 5));
    }

    #[test]
    pub fn test_intersect() {
        let test_cases = [
            (AlmanacRange(1, 5), AlmanacRange(3, 5), Some(AlmanacRange(3, 5))),
            (AlmanacRange(98, 99), AlmanacRange(79, 93), None),
            (AlmanacRange(1, 5), AlmanacRange(3, 7), Some(AlmanacRange(3, 5))),
            (AlmanacRange(3, 7), AlmanacRange(1, 5), Some(AlmanacRange(3, 5))),
        ];

        for case in test_cases {
            assert_eq!(intersect(&case.0, &case.1), case.2);
        }
    }

    #[test]
    pub fn test_diff_lower() {
        let test_cases = [
            (AlmanacRange(1, 5), AlmanacRange(3, 5), Some(AlmanacRange(1, 3))),
            (AlmanacRange(98, 99), AlmanacRange(79, 93), Some(AlmanacRange(79, 93))),
            (AlmanacRange(1, 5), AlmanacRange(3, 7), Some(AlmanacRange(1, 3))),
            (AlmanacRange(3, 7), AlmanacRange(1, 5), Some(AlmanacRange(1, 3))),
        ];

        for case in test_cases {
            assert_eq!(diff_lower(&case.0, &case.1), case.2);
        }
    }

    #[test]
    pub fn test_diff_upper() {
        let test_cases = [
            (AlmanacRange(1, 5), AlmanacRange(3, 5), None),
            (AlmanacRange(98, 99), AlmanacRange(79, 93), Some(AlmanacRange(98, 99))),
            (AlmanacRange(1, 5), AlmanacRange(3, 7), Some(AlmanacRange(6, 7))),
            (AlmanacRange(3, 7), AlmanacRange(1, 5), Some(AlmanacRange(6, 7))),
            (AlmanacRange(79, 93), AlmanacRange(56, 93), None),
        ];

        for case in test_cases {
            assert_eq!(diff_upper(&case.0, &case.1), case.2);
        }
    }

    #[test]
    pub fn test_get_destinations_exact_match() {
        /* {98: 50, 99: 51, 50: 52,
           }
        */
        let input_map = Mapping {
            title: String::from("test"),
            ranges: vec![
                MapRange {
                    src_start: 98,
                    dest_start: 50,
                    range: 2,
                },
                MapRange {
                    src_start: 50,
                    dest_start: 52,
                    range: 48,
                },
            ],
        };

        let mut input_range = HashSet::new();
        input_range.insert(AlmanacRange(79, 93));
        input_range.insert(AlmanacRange(55, 68));

        let mut expected: HashSet<AlmanacRange> = HashSet::new();
        expected.insert(AlmanacRange(81, 94));
        expected.insert(AlmanacRange(57, 69));

        assert_eq!(get_outputs(&input_map, &input_range), expected)
    }

    pub fn test_get_destinations_exact_match_fertilizer() {
        /* {98: 50, 99: 51, 50: 52,
           }
        */
        let input_map = Mapping {
            title: String::from("test"),
            ranges: vec![
                MapRange {
                    src_start: 15, // [15, 52)
                    dest_start: 0, // [0, 37)
                    range: 37,
                },
                MapRange {
                    src_start: 52,  // [52, 54)
                    dest_start: 37, // [37,39)
                    range: 2,
                },
                MapRange {
                    src_start: 39, // [39,54)
                    dest_start: 0, // [0, 15)
                    range: 15,
                },
            ],
        };

        let mut input_range = HashSet::new();
        input_range.insert(AlmanacRange(81, 95));
        input_range.insert(AlmanacRange(57, 70));

        let mut expected: HashSet<AlmanacRange> = HashSet::new();
        expected.insert(AlmanacRange(81, 94));
        expected.insert(AlmanacRange(57, 69));

        assert_eq!(get_outputs(&input_map, &input_range), expected)
    }

    #[test]
    pub fn test_get_destinations_exact_match_water() {
        /* {98: 50, 99: 51, 50: 52,
           }
        */
        let input_map = Mapping {
            title: String::from("test"),
            ranges: vec![
                MapRange {
                    // dest = src - 4
                    dest_start: 49, // [49, 57)
                    src_start: 53,  // [53, 61)
                    range: 8,
                },
                MapRange {
                    dest_start: 0, // [0,41)
                    src_start: 11, // [11, 53)
                    range: 42,
                },
                MapRange {
                    dest_start: 42, // [42, 49)
                    src_start: 0,   // [0,6)
                    range: 7,
                },
                MapRange {
                    dest_start: 57, // [57, 61)
                    src_start: 7,   // [7,11)
                    range: 4,
                },
            ],
        };

        let mut input_range = HashSet::new();
        input_range.insert(AlmanacRange(81, 95));
        input_range.insert(AlmanacRange(57, 70));

        let mut expected: HashSet<AlmanacRange> = HashSet::new();
        expected.insert(AlmanacRange(81, 95));
        expected.insert(AlmanacRange(53, 56));
        expected.insert(AlmanacRange(62, 70)); // a carry

        assert_eq!(get_outputs(&input_map, &input_range), expected)
    }
}
