#[derive(Debug)]
pub struct OasisReport {
    pub points: Vec<PointHistory>,
}

impl OasisReport {
    pub fn from(input: &Vec<String>) -> OasisReport {
        OasisReport {
            points: input.iter().map(PointHistory::from).collect(),
        }
    }
}

#[derive(Debug)]
pub struct PointHistory {
    values: Vec<i64>,
}

impl PointHistory {
    pub fn from(input: &String) -> PointHistory {
        PointHistory {
            values: input.split_ascii_whitespace().map(|c| c.parse::<i64>().unwrap()).collect(),
        }
    }

    pub fn extrapolate(&self) -> i64 {
        let stack = self.get_extrapolation_diffs();
        let mut result = 0;
        for extrapolation in stack.iter().rev() {
            result += extrapolation.last().unwrap();
        }
        result
    }

    pub fn extrapolate_backward(&self) -> i64 {
        let stack = self.get_extrapolation_diffs();
        println!("{:?}", stack);
        let mut result = 0;
        let mut previous: i64;
        for extrapolation in stack.iter().rev() {
            previous = result;
            result = extrapolation.first().unwrap() - previous;
        }
        result
    }

    fn get_extrapolation_diffs(&self) -> Vec<Vec<i64>> {
        let mut stack: Vec<Vec<i64>> = Vec::new();
        stack.push(self.values.clone());
        loop {
            let next = PointHistory::diffs(&stack.last().unwrap());
            if next.iter().all(|i| *i == 0) {
                stack.push(next);
                break;
            } else {
                stack.push(next);
            }
        }
        stack
    }

    fn diffs(v: &Vec<i64>) -> Vec<i64> {
        let mut result: Vec<i64> = Vec::new();
        for (i, val) in v.iter().enumerate() {
            match v.get(i + 1) {
                Some(j) => result.push(*j - val),
                None => break,
            }
        }
        result
    }
}
