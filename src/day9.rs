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
