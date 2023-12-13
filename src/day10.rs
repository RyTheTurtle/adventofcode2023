use crate::structs::maze::{Maze, MazePath};
use std::{
    collections::{HashMap, HashSet, VecDeque},
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
    paths.reserve(15000);
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
    let maze = Maze::from(input);
    let start = maze
        .find(&'S')
        .expect("Should always have a starting point");
    // DFS through the whole maze as a single path
    let pipe_path: MazePath = get_pipe_path(&maze, &start);

    let pipe_cells: HashSet<(usize, usize)> = HashSet::from_iter(pipe_path.cells);
    let rays: Vec<Vec<(usize, usize)>> = maze
        .list_coordinates()
        .iter()
        .filter(|c| !pipe_cells.contains(c))
        .map(|c| get_ray(c, &maze))
        .collect();
    let mut count = 0;

    for ray in rays {
        let mut north_south_intersections = (0, 0);
        let mut ray_iter = ray.iter();
        loop {
            match ray_iter.next() {
                Some(c) if pipe_cells.contains(c) => {
                    if is_north(&maze.get(c.0, c.1).unwrap()) {
                        north_south_intersections.0 += 1;
                    }
                    if is_south(&maze.get(c.0, c.1).unwrap()) {
                        north_south_intersections.1 += 1;
                    }
                }
                Some(_) => {}
                None => {
                    break;
                }
            }
        }
        if north_south_intersections > (0, 0) {
            println!("Ray: {:?}", ray);
            println!("North/South intersections: {:?}", north_south_intersections);
        }
        if north_south_intersections
            .0
            .min(north_south_intersections.1)
            % 2
            == 1
        {
            count += 1;
        }
    }
    count as u64
}

// DFS until we reach the starting point again and return the path
// through the maze that contains all the points. Assumes we will
// have some valid path that gets us to the starting point again.
fn get_pipe_path(maze: &Maze, start: &(usize, usize)) -> MazePath {
    let mut distances: HashMap<(usize, usize), usize> = HashMap::new();
    let mut pipe_path = MazePath {
        cells: vec![*start],
    };
    pipe_path.cells.reserve(15000);
    loop {
        let current: &(usize, usize) = pipe_path
            .cells
            .last()
            .expect("path cannot be empty");
        // update distance of current to longest path we've found so far
        match distances.get(current) {
            Some(_) => break,
            None => {
                distances.insert(*current, pipe_path.cells.len() - 1);
            }
        }
        // we are told there's exactly 2 connections to each pipe
        // so we should only have one new path to follow after filtering out
        // the connection to the previous place we came from
        match maze
            .get_connected_cells(current.0, current.1)
            .into_iter()
            .filter(|c| !pipe_path.cells.contains(c))
            .nth(0)
        {
            Some(p) => pipe_path.cells.push(p),
            None => break,
        }
    }
    pipe_path
}

fn get_ray(c: &(usize, usize), maze: &Maze) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    let mut dist = 0;
    loop {
        match maze.get(c.0, c.1 + dist) {
            Some(_) => {
                result.push((c.0, c.1 + dist));
            }
            None => {
                break;
            }
        }
        dist += 1;
    }
    result
}

fn is_north(c: &char) -> bool {
    match c {
        '|' | 'L' | 'J' => true,
        _ => false,
    }
}
fn is_south(c: &char) -> bool {
    match c {
        '|' | '7' | 'F' => true,
        _ => false,
    }
}
