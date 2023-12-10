use std::collections::HashSet;

/**
 * Day 5 structs for Farmer's almanac
 */

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct u64Range(pub u64, pub u64);

impl u64Range {
    pub fn new(a: u64, b: u64) -> u64Range {
        u64Range(a.min(b), a.max(b))
    }
    /**
     * Returns the intersection of a range. Consider two ranges A and B
     *    
     *  AAAAAAAAA
     *      BBBBBBBB
     *      -----       <- this is the intersection
     *  AAAAAAAAAA
     *  BBBBBBB
     *  -------         <- this is the intersection
     *
     *  AAAAAAAAAA
     *      BBBB   
     *      ----        <- this is the intersection
     *
     * AAAAA
     *      BBBBB       <- this has no intersection    
     */
    pub fn intersect(a: &u64Range, b: &u64Range) -> Option<u64Range> {
        let no_overlap = a < b && a.1 <= b.0;
        let no_overlap = no_overlap || b < a && b.1 <= a.0;
        if no_overlap {
            return None;
        }
        let lower = a.0.max(b.0);
        let upper = a.1.min(b.1);
        let result = u64Range::new(lower, upper);

        Some(result)
    }

    /*
     * Given ranges A and B with intersection i
     *    aaaaaaaa
     *         bbbbbbb
     *        iii
     *    |--|  <- what we're trying to capture
     */
    pub fn diff_lower(a: &u64Range, b: &u64Range) -> Option<u64Range> {
        match u64Range::intersect(a, b) {
            None => Some(a.min(b).clone()),
            Some(i) => {
                if i.0 == a.0 && a.0 == b.0 {
                    return None;
                }
                Some(u64Range::new(i.0.min(a.0).min(b.0), i.0))
            }
        }
    }

    /*
     * Given ranges A and B with intersection i
     *    aaaaaaa
     *        bbbbbbb
     *        iii
     *           |--|  <- what we're trying to capture
     */
    pub fn diff_upper(a: &u64Range, b: &u64Range) -> Option<u64Range> {
        match u64Range::intersect(a, b) {
            None => Some(a.max(b).clone()),

            Some(i) => {
                if i.1 == a.1 && a.1 == b.1 {
                    // no upper bound diff
                    return None;
                }
                Some(u64Range::new(i.1, a.1.max(b.1)))
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
    pub seeds: Vec<u64Range>,
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
        let mut seeds: Vec<u64Range> = Vec::new();
        let mut seed_iter = seed_ranges.iter();
        loop {
            match seed_iter.next() {
                Some(start) => match seed_iter.next() {
                    Some(range) => seeds.push(u64Range(*start, start.checked_add(*range).unwrap())),
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
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
    pub fn map_dest(&self, sources: &HashSet<u64Range>) -> HashSet<u64Range> {
        // populate stack
        let mut range_stack: Vec<u64Range> = Vec::new();
        let mut already_processed: HashSet<u64Range> = HashSet::new();
        let mut results: HashSet<u64Range> = HashSet::new();
        for range in sources.clone() {
            range_stack.push(range);
        }
        // process items off the stack
        while range_stack.len() > 0 {
            let current_range = range_stack.pop().unwrap();
            println!(
                "map_dest({:?}) Evaluating input range {:?}",
                self.title, current_range
            );

            // just in case we end up with some duplicate ranges showing up
            match already_processed.get(&current_range) {
                Some(r) => {
                    println!("\tAlready processed range {:?}", r);
                    continue;
                }
                None => {
                    already_processed.insert(current_range);
                }
            }

            let mut found_intersection = false;
            for mr in &self.ranges {
                let map_range = u64Range::new(mr.src_start, mr.src_start + mr.range);
                println!("\tChecking map range {:?}", map_range);
                let intersection = u64Range::intersect(&map_range, &current_range);
                let lower_diff = u64Range::diff_lower(&map_range, &current_range);
                let upper_diff = u64Range::diff_upper(&map_range, &current_range);
                match intersection {
                    Some(r) => {
                        // intersection is mapped to destination result
                        println!("\tFound intersection {:?}", r);
                        found_intersection = true;
                        let output_range = mr.get_dest(&r);
                        results.insert(output_range);

                        if current_range != r {
                            println!("\tcurrent range is not fully enclosed in map range");
                            match lower_diff {
                                Some(d) if current_range.0 < map_range.0 => {
                                    println!(
                                        "\tpushing unmatched lower range {:?} to processing stack",
                                        d
                                    );
                                    range_stack.push(d);
                                }
                                _ => {}
                            }
                            match upper_diff {
                                Some(d) if current_range.0 > map_range.0 => {
                                    println!(
                                        "\tpushing unmatched upper range {:?} to processing stack",
                                        d
                                    );
                                    range_stack.push(d);
                                }
                                _ => {}
                            }
                        }
                    }
                    None => {
                        // no intersection means this range maps to itself for output
                        continue;
                    }
                }
            }
            if !found_intersection {
                // range didn't map to any inputs
                println!("\tNo intersection found with current range, ");
                results.insert(current_range);
            }
        }
        results
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MapRange {
    pub dest_start: u64,
    pub src_start: u64,
    pub range: u64,
}

impl MapRange {
    fn get_dest(&self, source: &u64Range) -> u64Range {
        let offset: u64 = source.0 - self.src_start; 
        let start: u64 = self.dest_start + offset; 
        let dist: u64 = source.1 - source.0;
        return u64Range::new(start, start + dist);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn new_range_right_order() {
        assert_eq!(u64Range::new(4, 2), u64Range(2, 4));
    }

    #[test]
    pub fn test_range_ordering() {
        assert!(u64Range(2, 3) < u64Range(2, 4));
        assert!(u64Range(2, 3) == u64Range(2, 3));
        assert!(u64Range(1, 3) < u64Range(2, 5));
        assert!(u64Range(1, 5) > u64Range(0, 5));
    }

    #[test]
    pub fn test_intersect() {
        let test_cases = [
            (u64Range(1, 5), u64Range(3, 5), Some(u64Range(3, 5))),
            (u64Range(98, 99), u64Range(79, 93), None),
            (u64Range(1, 5), u64Range(3, 7), Some(u64Range(3, 5))),
            (u64Range(3, 7), u64Range(1, 5), Some(u64Range(3, 5))),
        ];

        for case in test_cases {
            assert_eq!(u64Range::intersect(&case.0, &case.1), case.2);
        }
    }

    #[test]
    pub fn test_diff_lower() {
        let test_cases = [
            (u64Range(1, 5), u64Range(3, 5), Some(u64Range(1, 3))),
            (u64Range(98, 99), u64Range(79, 93), Some(u64Range(79, 93))),
            (u64Range(1, 5), u64Range(3, 7), Some(u64Range(1, 3))),
            (u64Range(3, 7), u64Range(1, 5), Some(u64Range(1, 3))),
        ];

        for case in test_cases {
            assert_eq!(
                u64Range::diff_lower(&case.0, &case.1),
                case.2,
                "Failed for {:?}",
                case
            );
        }
    }

    #[test]
    pub fn test_diff_upper() {
        let test_cases = [
            (u64Range(1, 5), u64Range(3, 5), None),
            (u64Range(98, 99), u64Range(79, 93), Some(u64Range(98, 99))),
            (u64Range(1, 5), u64Range(3, 7), Some(u64Range(5, 7))),
            (u64Range(3, 7), u64Range(1, 5), Some(u64Range(5, 7))),
            (u64Range(79, 93), u64Range(56, 93), None),
        ];

        for case in test_cases {
            assert_eq!(
                u64Range::diff_upper(&case.0, &case.1),
                case.2,
                "Failed for case {:?}",
                case
            );
        }
    }

    #[test]
    pub fn test_map_range() {
        let input = MapRange {
            dest_start: 52,
            src_start: 50,
            range: 10,
        };
        let src: u64Range = u64Range::new(51, 55);
        let expected: u64Range = u64Range::new(53, 57);
        assert_eq!(input.get_dest(&src), expected);
    }
}
