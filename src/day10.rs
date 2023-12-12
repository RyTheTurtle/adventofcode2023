use crate::structs::maze::{Maze, MazePath};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    thread::current,
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

/**
 * Need to find the area of cells enclosed in the loop.
 * - DFS to find the actual path for the loop
 * - for every non pipe piont, BFS until we either
 *   1. reach edge of maze (not contained)
 *   2. reach only pipes, (all points in here are contained in maze)
 */
pub fn part_2(input: &Vec<String>) -> u64 {
    let maze = Maze::from(input);
    let start = maze
        .find(&'S')
        .expect("Should always have a starting point");
    // DFS through the whole maze as a single path
    let pipe_path: MazePath = get_pipe_path(&maze, &start);

    let pipe_cells: HashSet<(usize, usize)> = HashSet::from_iter(pipe_path.cells);
    let mut not_enclosed_cells: HashSet<(usize, usize)> = HashSet::new();
    let mut enclosed_cells: HashSet<(usize, usize)> = HashSet::new();

    // DFS every cell in the grid to determine if it can escape or not. If a cell on a DFS path
    // hits a cell that we know is not enclosed, we can stop early and assume it and all cells
    // on the current path are able to escape. Likewise, if we hit a cell that we've already determined
    // is enclosed, then we can assume the entire path of cells we've checked are also enclosed.
    for coordinate in maze.list_coordinates() {
        println!("checking coordinate {:?}", coordinate);
        let path = MazePath {
            cells: vec![coordinate],
        };

        match dfs_to_outside_maze(
            &maze,
            path,
            &enclosed_cells,
            &not_enclosed_cells,
            &pipe_cells,
        ) {
            Some(valid_path) => {
                // we found a way out, add to not enclosed cells
                not_enclosed_cells.extend(valid_path.cells);
            }
            None => {
                if !pipe_cells.contains(&coordinate) {
                    enclosed_cells.insert(coordinate);
                }
            }
        }
    }
    println!("Enclosed cells: {:?}", enclosed_cells);
    println!("Not enclosed cells: {:?}", not_enclosed_cells);
    enclosed_cells.len() as u64
}

fn dfs_to_outside_maze(
    m: &Maze,
    path: MazePath,
    blocked: &HashSet<(usize, usize)>,
    validated: &HashSet<(usize, usize)>,
    pipe: &HashSet<(usize, usize)>,
) -> Option<MazePath> {
    println!("dfs: path: {:?}", path);
    println!("\tBlocked cells: {:?}", blocked);
    println!("\tValidated cells: {:?}", validated);
    
    let current_cell = path
        .cells
        .last()
        .expect("path should never be empty");
    // end condition: we've reached a cell known to be blocked
    if blocked.contains(current_cell) {
        return None;
    }
    // end condition: we've reached a cell known to be escapable from the maze
    if validated.contains(current_cell) {
        return Some(path);
    }
    // end condition: we've hit a pipe and cannot proceed further
    if pipe.contains(current_cell) {
        return None;
    }
    // end condition: we've reached a point outside of the map
    match m.get(current_cell.0, current_cell.1) {
        None => {
            return Some(path);
        }
        _ => {}
    }

    let adjacent_coordinates = get_adj_coords(&current_cell);
    for coordinate in adjacent_coordinates {
        // end condition, an adjacent coordinate is outside of the maze
        // if either coordinate is negative, we short circuit to avoid overflow
        if coordinate.0 < 0 || coordinate.1 < 0 {
            return Some(path);
        }
        let usized_coordinate = (coordinate.0 as usize, coordinate.1 as usize);
        // skip any coordinate that's already in our path to avoid circular loops
        if path.cells.contains(&usized_coordinate) {
            continue;
        }
        let mut next_path = MazePath {
            cells: path.cells.clone(),
        };
        next_path.cells.push(usized_coordinate);
        match dfs_to_outside_maze(m, next_path, blocked, validated, pipe) {
            Some(p) => {
                // found a maze path, stop here
                return Some(p);
            }
            None => continue,
        }
    }

    None
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

fn get_adj_coords(start: &(usize, usize)) -> Vec<(isize, isize)> {
    let adj_deltas: Vec<(isize, isize)> = vec![
        (1, 0),
        (-1, 0),
        (0, -1),
        (0, 1),
    ];
    let adjacent_coords: Vec<(isize, isize)> = adj_deltas
        .into_iter()
        .map(|c| ((start.0 as isize + c.0), (start.1 as isize + c.1)))
        .collect();
    println!("\tAdj cells: {:?}", adjacent_coords);
    adjacent_coords
}
