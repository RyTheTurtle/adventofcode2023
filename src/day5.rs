use crate::util::{diff_lower, diff_upper, get_outputs, intersect, new_range, read_lines, Range};
use std::{
    collections::HashSet,
    time::{Duration, Instant},
};

pub fn part_1(input: &Vec<String>) -> u64 {
    let almanac = to_almanac(input);
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
    let ranged_almanac: RangedAlmanac = to_ranged_almanac(input);
    let start = Instant::now();
    let mut ranges: HashSet<Range> = HashSet::new();
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

fn to_almanac(input: &Vec<String>) -> Almanac {
    let mut result = Almanac {
        seeds: Vec::new(),
        maps: Vec::new(),
    };
    let mut input_iter = input.iter();
    // get seeds from the input iter
    parse_seeds(&mut result, &mut input_iter);
    parse_maps(&mut result, &mut input_iter);
    result
}

fn to_ranged_almanac(input: &Vec<String>) -> RangedAlmanac {
    let mut result: RangedAlmanac = RangedAlmanac {
        seeds: Vec::new(),
        maps: Vec::new(),
    };
    let mut input_iter = input.iter();
    // get seeds from the input iter
    parse_ranged_seeds(&mut result, &mut input_iter);
    parse_ranged_maps(&mut result, &mut input_iter);
    result
}

fn parse_seeds(result: &mut Almanac, input_iter: &mut std::slice::Iter<'_, String>) {
    result.seeds = input_iter
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
}

fn parse_ranged_seeds(result: &mut RangedAlmanac, input_iter: &mut std::slice::Iter<'_, String>) {
    let seed_ranges: Vec<u64> = input_iter
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let mut seeds: Vec<Range> = Vec::new();
    let mut seed_iter = seed_ranges.iter();
    loop {
        match seed_iter.next() {
            Some(start) => match seed_iter.next() {
                Some(range) => seeds.push(Range(*start, start.checked_add(*range).unwrap())),
                None => panic!("invalid seed range"),
            },
            None => break,
        }
    }
    result.seeds = seeds;
}

// just for compatability and not wanting to edit previous functions
fn parse_ranged_maps(result: &mut RangedAlmanac, input_iter: &mut std::slice::Iter<'_, String>) {
    loop {
        match input_iter.next() {
            Some(l) if l.contains("map:") => {
                let mut map: Mapping = Mapping {
                    title: String::from(l.split("map:").nth(0).unwrap().trim()),
                    ranges: Vec::new(),
                };
                loop {
                    match input_iter.next() {
                        Some(r) if r.trim().len() > 0 => {
                            let v: Vec<u64> = r
                                .split_ascii_whitespace()
                                .map(|s| s.parse::<u64>().unwrap())
                                .collect();
                            map.ranges.push(MapRange {
                                dest_start: v.get(0).unwrap().clone(),
                                src_start: v.get(1).unwrap().clone(),
                                range: v.get(2).unwrap().clone(),
                            });
                        }
                        _ => break, // reached end of the map of ranges
                    }
                }
                result.maps.push(map);
            }
            Some(l) if l.trim().len() == 0 => {
                // skip, blank space between inputs
            }
            _ => break,
        }
    }
}

fn parse_maps(result: &mut Almanac, input_iter: &mut std::slice::Iter<'_, String>) {
    loop {
        match input_iter.next() {
            Some(l) if l.contains("map:") => {
                let mut map: Mapping = Mapping {
                    title: String::from(l.split("map:").nth(0).unwrap().trim()),
                    ranges: Vec::new(),
                };
                loop {
                    match input_iter.next() {
                        Some(r) if r.trim().len() > 0 => {
                            let v: Vec<u64> = r
                                .split_ascii_whitespace()
                                .map(|s| s.parse::<u64>().unwrap())
                                .collect();
                            map.ranges.push(MapRange {
                                dest_start: v.get(0).unwrap().clone(),
                                src_start: v.get(1).unwrap().clone(),
                                range: v.get(2).unwrap().clone(),
                            });
                        }
                        _ => break, // reached end of the map of ranges
                    }
                }
                result.maps.push(map);
            }
            Some(l) if l.trim().len() == 0 => {
                // skip, blank space between inputs
            }
            _ => break,
        }
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Mapping>,
}

#[derive(Debug)]
struct RangedAlmanac {
    seeds: Vec<Range>,
    maps: Vec<Mapping>,
}

#[derive(Debug)]
pub struct Mapping {
    pub title: String,
    pub ranges: Vec<MapRange>,
}

#[derive(Debug)]
pub struct MapRange {
    pub dest_start: u64,
    pub src_start: u64,
    pub range: u64,
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
