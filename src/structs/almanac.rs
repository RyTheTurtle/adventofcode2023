use std::collections::HashSet;

/**
 * Day 5 structs for Farmer's almanac
 */

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct AlmanacRange(pub u64, pub u64);

impl AlmanacRange {
    pub fn new(a: u64, b: u64) -> AlmanacRange {
        AlmanacRange(a.min(b), a.max(b))
    }

    pub fn diff_lower(a: &AlmanacRange, b: &AlmanacRange) -> Option<AlmanacRange> {
        match AlmanacRange::intersect(a, b) {
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

    pub fn diff_upper(a: &AlmanacRange, b: &AlmanacRange) -> Option<AlmanacRange> {
        match AlmanacRange::intersect(a, b) {
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
}

#[derive(Debug)]
pub struct Almanac {
    pub seeds: Vec<u64>,
    pub maps: Vec<Mapping>,
}

impl Almanac {
    pub fn from(input: &Vec<String>) -> Almanac {
        let mut result = Almanac {
            seeds: Vec::new(),
            maps: Vec::new(),
        };
        let mut input_iter = input.iter();
        // get seeds from the input iter
        Almanac::parse_seeds(&mut result, &mut input_iter);
        Almanac::parse_maps(&mut result, &mut input_iter);
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
}

#[derive(Debug)]
pub struct RangedAlmanac {
    pub seeds: Vec<AlmanacRange>,
    pub maps: Vec<Mapping>,
}

impl RangedAlmanac {
    pub fn from(input: &Vec<String>) -> RangedAlmanac {
        let mut result: RangedAlmanac = RangedAlmanac {
            seeds: Vec::new(),
            maps: Vec::new(),
        };
        let mut input_iter = input.iter();
        // get seeds from the input iter
        RangedAlmanac::parse_ranged_seeds(&mut result, &mut input_iter);
        RangedAlmanac::parse_ranged_maps(&mut result, &mut input_iter);
        result
    }
    fn parse_ranged_seeds(
        result: &mut RangedAlmanac,
        input_iter: &mut std::slice::Iter<'_, String>,
    ) {
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
                    Some(range) => {
                        seeds.push(AlmanacRange(*start, start.checked_add(*range).unwrap()))
                    }
                    None => panic!("invalid seed range"),
                },
                None => break,
            }
        }
        result.seeds = seeds;
    }
    fn parse_ranged_maps(
        result: &mut RangedAlmanac,
        input_iter: &mut std::slice::Iter<'_, String>,
    ) {
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
}

#[derive(Debug)]
pub struct Mapping {
    pub title: String,
    pub ranges: Vec<MapRange>,
}

impl Mapping {
    /**
     * Maps the input sources to the output destinations represented as a set of
     * ranges.
     *
     *  any diffs are added back to stack for further processing only if we don't have a full match for the input range

    *  given two ranges
    *   aaaaaa
    *       bbbbbbb
    *  only take upper range
    * -----
    *       aaaaaaaa
    *    bbbbbbb
    *  only take lower diff
    *
     * Values that are not mapped to something new from this mapping are output
     * as a range of value that are the same as the input.
     */
    pub fn map_dest(&self, sources: &HashSet<AlmanacRange>) -> HashSet<AlmanacRange> {
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
            for mr in &self.ranges {
                let map_range = AlmanacRange::new(mr.src_start, mr.src_start + mr.range);
                // println!("Checking map range {:?}", map_range);
                let intersection = AlmanacRange::intersect(&map_range, &current_range);
                let lower_diff = AlmanacRange::diff_lower(&map_range, &current_range);
                let upper_diff = AlmanacRange::diff_upper(&map_range, &current_range);
                match intersection {
                    Some(r) => {
                        // intersection is mapped to destination result
                        // println!("Found intersection {:?}",r);
                        found_intersection = true;
                        let dist = r.1 - r.0 - 1;
                        let offset = r.0 - mr.src_start;
                        let output_range = AlmanacRange::new(
                            mr.dest_start + offset,
                            mr.dest_start + offset + dist,
                        );
                        results.insert(output_range);

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
}

#[derive(Debug)]
pub struct MapRange {
    pub dest_start: u64,
    pub src_start: u64,
    pub range: u64,
}

// just for compatability and not wanting to edit previous functions

#[cfg(test)]
mod tests {
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
            (
                AlmanacRange(1, 5),
                AlmanacRange(3, 5),
                Some(AlmanacRange(3, 5)),
            ),
            (AlmanacRange(98, 99), AlmanacRange(79, 93), None),
            (
                AlmanacRange(1, 5),
                AlmanacRange(3, 7),
                Some(AlmanacRange(3, 5)),
            ),
            (
                AlmanacRange(3, 7),
                AlmanacRange(1, 5),
                Some(AlmanacRange(3, 5)),
            ),
        ];

        for case in test_cases {
            assert_eq!(AlmanacRange::intersect(&case.0, &case.1), case.2);
        }
    }

    #[test]
    pub fn test_diff_lower() {
        let test_cases = [
            (
                AlmanacRange(1, 5),
                AlmanacRange(3, 5),
                Some(AlmanacRange(1, 3)),
            ),
            (
                AlmanacRange(98, 99),
                AlmanacRange(79, 93),
                Some(AlmanacRange(79, 93)),
            ),
            (
                AlmanacRange(1, 5),
                AlmanacRange(3, 7),
                Some(AlmanacRange(1, 3)),
            ),
            (
                AlmanacRange(3, 7),
                AlmanacRange(1, 5),
                Some(AlmanacRange(1, 3)),
            ),
        ];

        for case in test_cases {
            assert_eq!(AlmanacRange::diff_lower(&case.0, &case.1), case.2);
        }
    }

    #[test]
    pub fn test_diff_upper() {
        let test_cases = [
            (AlmanacRange(1, 5), AlmanacRange(3, 5), None),
            (
                AlmanacRange(98, 99),
                AlmanacRange(79, 93),
                Some(AlmanacRange(98, 99)),
            ),
            (
                AlmanacRange(1, 5),
                AlmanacRange(3, 7),
                Some(AlmanacRange(6, 7)),
            ),
            (
                AlmanacRange(3, 7),
                AlmanacRange(1, 5),
                Some(AlmanacRange(6, 7)),
            ),
            (AlmanacRange(79, 93), AlmanacRange(56, 93), None),
        ];

        for case in test_cases {
            assert_eq!(AlmanacRange::diff_upper(&case.0, &case.1), case.2);
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

        assert_eq!(input_map.map_dest(&input_range), expected)
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

        assert_eq!(input_map.map_dest(&input_range), expected)
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

        assert_eq!(input_map.map_dest(&input_range), expected)
    }
}
