/*
Maze struct used for day 10
 */
#[derive(Debug, Clone)]
pub struct Maze {
    cells: Vec<char>,
    row_len: usize,
}

impl Maze {
    pub fn from(input: &Vec<String>) -> Maze {
        let row_len: usize = input.get(0).expect("no input").len();
        let cells: Vec<char> = input.iter().flat_map(|s| s.chars()).collect();

        Maze {
            row_len: row_len,
            cells: cells,
        }
    }

    /** enumerates all valid coordinates in the grid
     */
    pub fn list_coordinates(&self) -> Vec<(usize, usize)> {
        let mut result: Vec<(usize, usize)> = Vec::new();
        for i in 0..self.cells.len() {
            result.push((i / self.row_len, i % self.row_len));
        }
        result
    }

    // helper function to get value at a coordinate
    pub fn get(&self, row: usize, col: usize) -> Option<&char> {
        if col >= self.row_len {
            return None;
        }
        let index = row * self.row_len + col % self.row_len;
        self.cells.get(index)
    }

    pub fn find(&self, needle: &char) -> Option<(usize, usize)> {
        match self.cells.iter().position(|c| c == needle) {
            Some(i) => Some((i / self.row_len, i % self.row_len)),
            None => None,
        }
    }

    /**
     * Given a coordinate to start at, check valid adjacent cells and return
     * only the coordinates of the cells that are legitimately connected via pipes.
     */
    pub fn get_connected_cells(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut result: Vec<(usize, usize)> = Vec::new();
        let current_cell_type = self.get(row, col).unwrap();
        let adj_cell_deltas = Maze::adj_deltas(current_cell_type);
        for delta in adj_cell_deltas {
            let adj_row = row as isize + delta.0;
            let adj_col = col as isize + delta.1;
            if adj_row < 0 || adj_col < 0 {
                continue; // invalid point
            }
            match self.get(adj_row as usize, adj_col as usize) {
                Some(c) => {
                    // check if this cell's valid connections include the current cell
                    let reverse_deltas = Maze::adj_deltas(c);
                    for reverse_delta in reverse_deltas {
                        if adj_row + reverse_delta.0 == row as isize
                            && adj_col + reverse_delta.1 == col as isize
                        {
                            result.push((adj_row as usize, adj_col as usize));
                        }
                    }
                }
                None => continue,
            }
        }
        result
    }

    /**
     * simple helper to map a cell to the possible adjacent cells that could be validly connected.
     * The returned points are not actually checked against the actual maze, just a representation of
     * the transform that can be used to check the adjacent cell. So a vertical pipe (|)
     * will return (-1, 0), (1,0) indicating the only valid connections to check are the
     *  cells immediately above and below it
     */
    fn adj_deltas(c: &char) -> Vec<(isize, isize)> {
        match c {
            '|' => vec![
                (1, 0),
                (-1, 0),
            ],
            '-' => vec![
                (0, 1),
                (0, -1),
            ],
            'L' => vec![
                (-1, 0),
                (0, 1),
            ],
            'J' => vec![
                (-1, 0),
                (0, -1),
            ],
            '7' => vec![
                (1, 0),
                (0, -1),
            ],
            'F' => vec![
                (1, 0),
                (0, 1),
            ],
            'S' => vec![
                (1, 0),
                (-1, 0),
                (0, 1),
                (0, -1),
            ],
            _ => Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct MazePath {
    pub cells: Vec<(usize, usize)>,
}

impl MazePath {
    pub fn contains(&self, c: &(usize, usize)) -> bool {
        return self.cells.contains(c);
    }
}
