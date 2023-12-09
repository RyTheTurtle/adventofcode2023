/**
 * Day 5 structs for Farmer's almanac 
 */

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct AlmanacRange(pub u64, pub u64);

impl AlmanacRange {
    pub fn new(a: u64, b: u64) -> AlmanacRange { 
        AlmanacRange(a.min(b), a.max(b))
    }
}

#[derive(Debug)]
pub struct Almanac {
    pub seeds: Vec<u64>,
    pub maps: Vec<Mapping>,
}

impl Almanac { 
    pub fn from (input: &Vec<String>) -> Almanac {
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
}

#[derive(Debug)]
pub struct RangedAlmanac {
    pub seeds: Vec<AlmanacRange>,
    pub maps: Vec<Mapping>,
}

impl RangedAlmanac { 
    pub fn from (input: &Vec<String>) -> RangedAlmanac {
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

pub fn parse_ranged_seeds(result: &mut RangedAlmanac, input_iter: &mut std::slice::Iter<'_, String>) {
    let seed_ranges: Vec<u64> = input_iter
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let mut seeds: Vec<AlmanacRange> = Vec::new();
    let mut seed_iter = seed_ranges.iter();
    loop {
        match seed_iter.next() {
            Some(start) => match seed_iter.next() {
                Some(range) => seeds.push(AlmanacRange(*start, start.checked_add(*range).unwrap())),
                None => panic!("invalid seed range"),
            },
            None => break,
        }
    }
    result.seeds = seeds;
}

// just for compatability and not wanting to edit previous functions
pub fn parse_ranged_maps(result: &mut RangedAlmanac, input_iter: &mut std::slice::Iter<'_, String>) {
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

pub fn parse_maps(result: &mut Almanac, input_iter: &mut std::slice::Iter<'_, String>) {
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

