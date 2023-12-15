use crate::structs::oasis_report::OasisReport;

pub fn part_1(input: &Vec<String>) -> i64 {
    let report = OasisReport::from(input);
    report
        .points
        .iter()
        .map(|p| p.extrapolate())
        .sum()
}

pub fn part_2(input: &Vec<String>) -> i64 {
    OasisReport::from(input)
        .points
        .iter()
        .map(|p| p.extrapolate_backward())
        .sum()
}


#[cfg(test)]
mod tests { 
    use crate::util;

    use super::*; 
    // FIXME Move to integration tests

    #[test]
    pub fn test_part1(){
       let input =  util::read_lines("./input/9.txt");
       assert_eq!(part_1(&input), 2005352194);
    }

    #[test]
    pub fn test_part2(){
        let input =  util::read_lines("./input/9.txt");
        assert_eq!(part_2(&input), 1077);
     }
}