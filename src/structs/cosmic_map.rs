use core::fmt;
use std::fmt::Write;

#[derive(Debug)]
pub struct CosmicMap {
    coordinates: Vec<Vec<char>>,
}

impl From<&Vec<String>> for CosmicMap {
    fn from(value: &Vec<String>) -> Self {
        let result = CosmicMap {
            coordinates: value.iter().map(|s| s.chars().collect()).collect(),
        };
        result
    }
}

impl fmt::Display for CosmicMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.coordinates {
            for col in row {
                f.write_char(*col)?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

impl CosmicMap {
    pub fn get_empty_rows(&self) -> Vec<isize> {
        let mut result: Vec<isize> = Vec::new();
        for (i, row) in self.coordinates.iter().enumerate() {
            if row.iter().all(|c| c == &'.') {
                result.push(i as isize);
            }
        }
        result
    }

    pub fn get_empty_cols(&self) -> Vec<isize> {
        let mut result: Vec<isize> = Vec::new();
        let column_range = self.coordinates.get(0).expect("map should not be empty").len();
        for column in 0..column_range {
            if self.coordinates.iter().all(|r| r.get(column).unwrap() == &'.') {
                result.push(column as isize);
            }
        }
        result
    }

    pub fn get_galaxies(&self, test: fn(&char) -> bool) -> Vec<(isize, isize)> {
        let mut result: Vec<(isize, isize)> = Vec::new();
        for (i, row) in self.coordinates.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if test(col) {
                    result.push((i as isize, j as isize));
                }
            }
        }
        result
    }
}
