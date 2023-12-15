use crate::structs::almanac::{Almanac, MapRange, Mapping, RangedAlmanac, U64range};
use std::collections::HashSet;

pub fn part_1(input: &Vec<String>) -> u64 {
    let almanac = Almanac::from(input);
    let mut lowest: u64 = u64::MAX;
    for seed in almanac.seeds {
        let mut value = seed;
        for map in &almanac.maps {
            value = next(&map, value);
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
    let mut ranges: HashSet<U64range> = HashSet::new();
    // set the initial ranges as the seed ranges from the almanac
    for seed in ranged_almanac.seeds {
        ranges.insert(seed);
    }

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

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;
    // FIXME Move to integration tests

    #[test]
    pub fn test_part1() {
        let input = util::read_lines("./input/5.txt");
        assert_eq!(part_1(&input), 1181555926);
    }

    #[test]
    pub fn test_part2() {
        let input = util::read_lines("./input/5.txt");
        assert_eq!(part_2(&input), 37806486);
    }
}
