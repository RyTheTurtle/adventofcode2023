use crate::util;

pub fn part_1(input: &Vec<String>) -> u64 {
    // parse inputs
    let times = util::parse_number_vec_following_colon(&input[0]);
    let distances = util::parse_number_vec_following_colon(&input[1]);
    let mut ways_to_beat_record: Vec<u64> = Vec::new();
    for (i, time) in times.into_iter().enumerate() {
        let target_distance = distances[i];
        let mut winning_options = 0;
        for j in 0..time {
            if j * (time - j) > target_distance {
                winning_options += 1;
            }
        }
        ways_to_beat_record.push(winning_options);
    }
    return ways_to_beat_record
        .iter()
        .fold(1, |acc, &e| acc * e);
}

pub fn part_2(input: &Vec<String>) -> u64 {
    // parse inputs
    let times = util::parse_number_vec_following_colon(&input[0]);
    let distances = util::parse_number_vec_following_colon(&input[1]);
    let mut ways_to_beat_record: Vec<u64> = Vec::new();
    for (i, time) in times.into_iter().enumerate() {
        let target_distance = distances[i];
        let mut winning_options = 0;
        for j in 0..time {
            if j * (time - j) > target_distance {
                winning_options += 1;
            }
        }
        ways_to_beat_record.push(winning_options);
    }
    return *ways_to_beat_record.get(0).unwrap();
}


#[cfg(test)]
mod tests { 
    use crate::util;

    use super::*; 
    // FIXME Move to integration tests

    #[test]
    pub fn test_part1(){
       let input =  util::read_lines("./input/6.txt");
       assert_eq!(part_1(&input), 1624896);
    }

    #[test]
    pub fn test_part2(){
        let input =  util::read_lines("./input/6.txt");
        assert_eq!(part_2(&input), 32583852);
     }
}