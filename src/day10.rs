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
    let rays: Vec<Vec<(usize,usize)>> = maze.list_coordinates().iter()
    .filter(|c| !pipe_cells.contains(c)).map(|c| get_ray(c, &maze)).collect();
    let mut count = 0;
    for ray in rays {
        println!("Ray: {:?}",ray);
        let mut intersections = 0;
        let mut ray_iter = ray.iter();
        loop{
            match ray_iter.next() { 
                Some(c) if pipe_cells.contains(c) => { 
                    println!("\tIntersection of pipe starting at {:?}",c);
                    let mut is_real_intersect = true;
                    let mut next_point = c;
                    loop {
                        match ray_iter.next(){
                            Some(p) if !pipe_cells.contains(p) => {
                                break;
                            },
                            Some(_) => {
                                // we hit a horizontal section of pipe, don't count as intersection
                                is_real_intersect = false;
                            },
                            None => {break;}
                        }
                    }
                    if is_real_intersect {
                        intersections += 1;
                    } else {
                        println!("\tNot a real intersection, since intersecting a horizontal part of pipe");
                    }
                },
                Some(_) => {},
                None => {
                    if intersections % 2 == 1 {
                        count += 1;
                    }
                    break;
                }
            }
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

fn get_ray(c: &(usize,usize), maze: &Maze) -> Vec<(usize,usize)> { 
    let mut result: Vec<(usize,usize)> = Vec::new();
    let mut dist = 0;
    loop {
        match maze.get(c.0, c.1+dist) {
            Some(_) => {
                result.push((c.0, c.1+dist));
            },
            None => {
                break;
            }
        }
        dist += 1;

    }
    result
}