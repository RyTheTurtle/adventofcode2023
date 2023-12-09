use crate::structs::game::{Game, GameCubeCount, Round};
use crate::util;

pub fn part_1(input: &Vec<String>) -> u32 {
    input
        .into_iter()
        .map(build_game)
        .filter(is_valid_game)
        .map(|g: Game| g.id)
        .sum::<u32>()
}

pub fn part_2(input: &Vec<String>) -> u32 {
    input
        .into_iter()
        .map(build_game)
        .map(min_cubes_required)
        .map(get_power)
        .sum::<u32>()
}

fn build_game(input: &String) -> Game {
    let input_parts: Vec<&str> = input.split(":").collect();
    let game_id_parts: Vec<&str> = input_parts
        .get(0)
        .unwrap()
        .split_ascii_whitespace()
        .collect();
    let game_id: u32 = game_id_parts.get(1).unwrap().parse().unwrap();
    let raw_rounds: Vec<&str> = input_parts.get(1).unwrap().split(";").collect();
    let rounds: Vec<Round> = raw_rounds.iter().map(build_round).collect();

    let result = Game {
        id: game_id,
        rounds: rounds,
    };
    result
}

fn build_round(input: &&str) -> Round {
    let mut result: Round = Round {
        red_count: 0,
        green_count: 0,
        blue_count: 0,
    };
    let round_parts = input.split(",");
    for part in round_parts {
        let dice_and_count: Vec<&str> = part.split_ascii_whitespace().collect();
        match dice_and_count
            .get(1)
            .unwrap()
            .to_string()
            .as_str()
        {
            "blue" => {
                result.blue_count = dice_and_count.get(0).unwrap().parse().unwrap();
            }
            "red" => {
                result.red_count = dice_and_count.get(0).unwrap().parse().unwrap();
            }
            "green" => {
                result.green_count = dice_and_count.get(0).unwrap().parse().unwrap();
            }
            _ => {}
        }
    }
    return result;
}

fn is_valid_game(g: &Game) -> bool {
    // simple hardcoding limits for part 1
    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;

    for round in &g.rounds {
        if round.blue_count > MAX_BLUE || round.red_count > MAX_RED || round.green_count > MAX_GREEN
        {
            return false;
        }
    }

    return true;
}

fn min_cubes_required(g: Game) -> GameCubeCount {
    let mut min_red = 0;
    let mut min_blue = 0;
    let mut min_green = 0;
    for round in &g.rounds {
        if round.red_count > min_red {
            min_red = round.red_count;
        }
        if round.blue_count > min_blue {
            min_blue = round.blue_count;
        }
        if round.green_count > min_green {
            min_green = round.green_count;
        }
    }
    GameCubeCount(min_red, min_green, min_blue)
}

fn get_power(g: GameCubeCount) -> u32 {
    g.0 * g.1 * g.2
}
