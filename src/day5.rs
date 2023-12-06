use crate::util;
use std::{time::{Duration, Instant}, collections::HashSet};

pub fn solve() {
    println!("Day 5\n====");
    let input = util::read_lines("./input/5.txt");
    println!("Input line size: {}", input.len());
    println!("Part 1\n---");
    let part1 = part_1(&input);
    println!("Result: {}", part1);
    println!("Part 2\n---");
    let part2 = part_2(&input);
    println!("Result: {}\n====", part2);
}

fn part_1(input: &Vec<String>) -> u64 {
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

fn part_2(input: &Vec<String>) -> u64 {
    let ranged_almanac : RangedAlmanac = to_ranged_almanac(input);
    let almanac : Almanac = to_almanac(input);
    // brute force....fuck it 
    let mut lowest: u64 = u64::MAX;
    let mut seeds_to_test : HashSet<u64> = HashSet::new(); 
    println!("Enumerating seeds that need to be tested...");
    let  seed_enumer_start = Instant::now(); 
    for r in ranged_almanac.seeds.iter() { 
        println!("{:?}", r);
        for i in r.0..r.1 { 
            seeds_to_test.insert(i);
        }
    }
    println!("  Took {:?}s to evaluate range", seed_enumer_start.elapsed().as_secs());
    println!("total seeds to test : {:?}",seeds_to_test.len());
    let answer_eval = Instant::now();
    for seed in seeds_to_test { 
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
    println!("Evaluated answer in {:?}s", answer_eval.elapsed().as_secs());
    lowest
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
    let mut seeds: Vec<util::Range> = Vec::new();
    let mut seed_iter = seed_ranges.iter();
    loop { 
        match seed_iter.next() { 
            Some(start) => { 
                match seed_iter.next() { 
                    Some(range) => { 
                        seeds.push(util::Range(*start, start.checked_add(*range).unwrap() ))
                    },
                    None => panic!("invalid seed range")
                }
            },
            None => break
        }
    }
    result.seeds = seeds;
}

// just for compatability and not wanting to edit previous functions
fn parse_ranged_maps(result: &mut RangedAlmanac,  input_iter: &mut std::slice::Iter<'_, String>) {
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

fn parse_maps(result: &mut Almanac,  input_iter: &mut std::slice::Iter<'_, String>) {
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
    seeds: Vec<util::Range>,
    maps: Vec<Mapping>
}

#[derive(Debug)]
struct Mapping {
    title: String,
    ranges: Vec<MapRange>,
}


#[derive(Debug)]
struct MapRange {
    dest_start: u64,
    src_start: u64,
    range: u64,
}

fn is_in_range(r: &MapRange, n: u64) -> bool { 
    n >= r.src_start && n < r.src_start+r.range 
}

fn get_dest(r: &MapRange, n: u64) -> u64 { 
    if !is_in_range(r, n) { 
        return n;
    }
    let offset = n - r.src_start; 
    r.dest_start + offset 
}

fn next(m: &Mapping, n: u64) -> u64 {
    let range: Vec<&MapRange> = m.ranges.iter().filter(|r| is_in_range(r, n)).collect();
    match range.get(0) { 
        Some(r) => get_dest(r, n),
        None => n
    }
}