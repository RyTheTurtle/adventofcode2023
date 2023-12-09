use crate::structs::almanac::{Almanac, AlmanacRange, MapRange, Mapping, RangedAlmanac};
use std::{collections::HashSet, time::Instant};

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
    let mut ranges: HashSet<AlmanacRange> = HashSet::new();
    // set the initial ranges as the seed ranges from the almanac
    for seed in ranged_almanac.seeds {
        ranges.insert(seed);
    }
    println!("Starting ranges: {:?}",ranges);

    let mut almanac_map_iter = ranged_almanac.maps.iter();
    loop {
        match almanac_map_iter.next() {
            Some(m) => { 
                ranges = m.map_dest(&ranges);
            }
            None => {
                break;
            }
        }
    }
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