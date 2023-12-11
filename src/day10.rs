use crate::structs::maze::{Maze, MazePath};
use std::{
    collections::{HashMap, VecDeque},
    vec,
};

pub fn part_1(input: &Vec<String>) -> u64 {
    let maze = Maze::from(input);
    let start = maze
        .find(&'S')
        .expect("Should always have a starting point");
    // BFS searching tracking the distance of cells at each point
    let mut paths: VecDeque<MazePath> = VecDeque::new();
    paths.push_back(MazePath { cells: vec![start] });
    let mut distances: HashMap<(usize, usize), usize> = HashMap::new();
    let mut path_op_count: u128 = 0;
    loop {
        match paths.pop_front() {
            Some(p) => {
                path_op_count += 1;
                let current = p.cells.last().expect("path cannot be empty");
                // update distance of current to longest path we've found so far
                match distances.get(current) {
                    Some(_) => continue,
                    None => {
                        distances.insert(*current, p.cells.len() - 1);
                    }
                }
                // update paths stack for next values
                let connected_cells: Vec<(usize, usize)> = maze
                    .get_connected_cells(current.0, current.1)
                    .into_iter()
                    .filter(|c| !p.contains(c))
                    .collect();
                for connection in connected_cells {
                    let mut new_path = MazePath {
                        cells: p.cells.clone(),
                    };
                    new_path.cells.push(connection.clone());
                    paths.push_back(new_path);
                }
            }
            None => break, // finished all path processing
        }
    }
    println!("Total path evaluation operations: {:?}", path_op_count);
    *distances.values().max().unwrap() as u64
}

pub fn part_2(input: &Vec<String>) -> u64 {
    0
}
