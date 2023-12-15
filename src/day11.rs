use crate::structs::cosmic_map::{self, CosmicMap};
use itertools::Itertools;
use std::collections::HashSet;

pub fn part_1(input: &Vec<String>) -> u64 {
    let mut cosmic_map = CosmicMap::from(input);
    compute_distances_between_galaxies(&cosmic_map, 2)
}

pub fn part_2(input: &Vec<String>) -> u64 {
    let mut cosmic_map = CosmicMap::from(input);
    compute_distances_between_galaxies(&cosmic_map, 1000000)
}

fn compute_distances_between_galaxies(cosmic_map: &CosmicMap, scale: isize) -> u64 {
    let galaxies = cosmic_map.get_galaxies(|c| *c == '#');
    let galaxy_pairs = create_pairs(&galaxies);
    println!("total galaxy pairs: {:?}", galaxy_pairs.len());
    // add <scale> to the row and col differences
    let mut sum: u64 = 0;
    let empty_rows = cosmic_map.get_empty_rows();
    let empty_cols = cosmic_map.get_empty_cols();
    for (i, p) in galaxy_pairs.iter().enumerate() {
        let a = p[0];
        let b = p[1];
        let mut row_diff = (a.0 - b.0).abs();
        let mut col_diff = (b.1 - a.1).abs();
        // add scale to the rows and columns, accounting for the initial row/col by
        // subtracing 1 from the scale when adding
        for row in a.0.min(b.0)..b.0.max(a.0) {
            if empty_rows.contains(&row) {
                row_diff += (scale - 1);
            }
        }
        for col in a.1.min(b.1)..b.1.max(a.1) {
            if empty_cols.contains(&col) {
                col_diff += (scale - 1);
            }
        }

        let dist = row_diff + col_diff;

        sum += dist as u64;
    }
    sum
}

// returns unique pairs of input coordinates for all combinations
fn create_pairs(coordinates: &Vec<(isize, isize)>) -> Vec<Vec<&(isize, isize)>> {
    let mut result: HashSet<Vec<&(isize, isize)>> = HashSet::new();
    for c in coordinates.into_iter().permutations(2) {
        let mut sorted = c.clone();
        sorted.sort();
        result.insert(sorted);
    }
    let mut res = Vec::from_iter(result);
    res.sort();
    res
}


#[cfg(test)]
mod tests { 
    use crate::util;

    use super::*; 
    // FIXME Move to integration tests

    #[test]
    pub fn test_part1(){
       let input =  util::read_lines("./input/11.txt");
       assert_eq!(part_1(&input), 9536038);
    }

    #[test]
    pub fn test_part2(){
        let input =  util::read_lines("./input/11.txt");
        assert_eq!(part_2(&input), 447744640566);
     }
}