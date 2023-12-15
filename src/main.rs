use std::{time::Instant, ops::ControlFlow};

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod structs;
mod util;
fn main() {
    println!("Advent of Code 2023");
    let day: u8 = std::env::args()
        .nth(1)
        .expect("Missing day argument")
        .parse()
        .expect("Day should be a number");
    let part: u8 = std::env::args()
        .nth(2)
        .expect("Missing part argument")
        .parse()
        .expect("part should be a number");
    solve_day(day, part);
}

fn solve_day(day: u8, part: u8)  {
    println!("Day {:?} Part {:?} : ", day, part);
    let input = util::read_lines(format!("./input/{}.txt", day));
    let start = Instant::now();
    let result: u64 = match day {
        1 => match part {
            1 => day1::part_1(&input) as u64,
            2 => day1::part_2(&input) as u64,
            _ => panic!("Invalid part"),
        },
        2 => match part {
            1 => day2::part_1(&input) as u64,
            2 => day2::part_2(&input) as u64,
            _ => panic!("Invalid part"),
        },
        3 => match part {
            1 => day3::part_1(&input) as u64,
            2 => day3::part_2(&input),
            _ => panic!("Invalid part"),
        },
        4 => match part {
            1 => day4::part_1(&input),
            2 => day4::part_2(&input),
            _ => panic!("Invalid part"),
        },
        5 => match part {
            1 => day5::part_1(&input),
            2 => day5::part_2(&input),
            _ => panic!("Invalid part"),
        },
        6 => match part {
            1 => day6::part_1(&input),
            2 => day6::part_2(&input),
            _ => panic!("Invalid part"),
        },
        7 => match part {
            1 => day7::part_1(&input),
            2 => day7::part_2(&input),
            _ => panic!("Invalid part"),
        },
        8 => match part {
            1 => day8::part_1(&input),
            2 => day8::part_2(&input),
            _ => panic!("Invalid part"),
        },
        10 => match part {
            1 => day10::part_1(&input),
            2 => day10::part_2(&input),
            _ => panic!("Invalid part"),
        },
        11 => match part {
            1 => day11::part_1(&input),
            2 => day11::part_2(&input),
            _ => panic!("Invalid part"),
        },
        12 => match part {
            1 => day12::part_1(&input),
            2 => day12::part_2(&input),
            _ => panic!("Invalid part"),
        },
        13 => match part {
            1 => day13::part_1(&input),
            2 => day13::part_2(&input),
            _ => panic!("Invalid part"),
        },
        14 => match part {
            1 => day14::part_1(&input),
            2 => day14::part_2(&input),
            _ => panic!("Invalid part"),
        },
        15 => match part {
            1 => day15::part_1(&input),
            2 => day15::part_2(&input),
            _ => panic!("Invalid part"),
        },
        16 => match part {
            1 => day16::part_1(&input),
            2 => day16::part_2(&input),
            _ => panic!("Invalid part"),
        },
        _ => 0,
    };
    if result == 0 {
        let result = match day {
            9 => match part {
                1 => day9::part_1(&input),
                2 => day9::part_2(&input),
                _ => panic!("Invalid part"),
            },
            _ => 0,
        };
        println!("{:?}", result);
        println!("Took {:?} ms", start.elapsed().as_millis());
    }
    println!("{:?}", result);
    println!("Took {:?} ms", start.elapsed().as_millis());

}