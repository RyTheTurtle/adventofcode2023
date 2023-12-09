/*
Grid structures for day 3
*/

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Grid {
    pub rows: Vec<Vec<char>>,
}

pub fn from(input: &Vec<String>) -> Grid {
    let mut grid_rows: Vec<Vec<char>> = Vec::new();
    for row in input {
        let r: Vec<char> = row.chars().collect();
        grid_rows.push(r);
    }

    Grid { rows: grid_rows }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
pub struct GridCoordinate(pub i32, pub i32);
