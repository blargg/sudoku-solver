use std::collections::BTreeSet;

#[derive(Clone)]
struct Grid<A> {
    data: [[A; 9]; 9],
}

type CellAssignment = BTreeSet<i32>;

fn empty_cell() -> CellAssignment {
    let mut empty = BTreeSet::new();
    for i in 1..=9 {
        empty.insert(i);
    }

    empty
}

fn parse_value(c: char) -> Option<i32> {
    if c == '1' {
        return Some(1);
    }
    if c == '2' {
        return Some(2);
    }
    if c == '3' {
        return Some(3);
    }
    if c == '4' {
        return Some(4);
    }
    if c == '5' {
        return Some(5);
    }
    if c == '6' {
        return Some(6);
    }
    if c == '7' {
        return Some(7);
    }
    if c == '8' {
        return Some(8);
    }
    if c == '9' {
        return Some(9);
    }

    return None;
}

impl Grid<CellAssignment> {
    fn parse(text: &str) -> Self {
        let mut puzzle = Grid::empty();
        let data = &mut puzzle.data;
        for (row_num, line) in text.lines().enumerate() {
            for (column, c) in line.chars().take(9).enumerate() {
                let n = parse_value(c);
                if let Some(n) = n {
                    data[row_num][column] = [n].into();
                }
            }
        }

        puzzle
    }

    fn to_string(&self) -> String {
        fn to_char(assignment: &CellAssignment) -> String {
            if assignment.len() == 1 {
                return format!("{}", assignment.first().unwrap());
            } else {
                return "x".to_owned();
            }
        }

        self.data
            .iter()
            .map(|line| line.iter().map(|x| to_char(x)).collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn solve(&self) -> Option<Self> {
        let mut puzzle = self.clone();
        puzzle.apply_constraints_all_cells();

        if puzzle.complete() {
            return Some(puzzle.clone());
        }

        if puzzle.is_impossible() {
            return None;
        }

        let (row, col) = puzzle.most_constrained_variable()?;
        // TODO, it would be better to order the next choice by which has the fewest possibilities.
        // TODO do we have to try all possible orders for variable assignment?
        for possible in puzzle.data[row][col].iter() {
            let mut next = puzzle.clone();
            next.data[row][col].clear();
            next.data[row][col].insert(*possible);
            let possible_solution = next.solve();
            if possible_solution.is_some() {
                return possible_solution;
            }
        }
        // We tried all possibilities, there were no solutions.
        return None;
    }

    /// Creates a new, empty puzzle.
    fn empty() -> Self {
        Self {
            data: std::array::from_fn(|_| std::array::from_fn(|_| empty_cell())),
        }
    }

    /// A board is complete when there is a valid assignment to every cell.
    fn complete(&self) -> bool {
        for row in &self.data {
            for cell in row {
                if !is_assigned(cell) {
                    return false;
                }
            }
        }

        return true;
    }

    // This is true when the puzzle is not possible to solve.
    // There is some variable that has no valid values.
    fn is_impossible(&self) -> bool {
        for row in &self.data {
            for cell in row {
                // There is some variable where there is no possible value. This is no longer solvable
                if cell.len() == 0 {
                    return true;
                }
            }
        }

        return false;
    }

    fn get(&self, x: usize, y: usize) -> &CellAssignment {
        return &self.data[x][y];
    }

    // This is incomplete. Just checks if any cell is empty.
    fn is_valid(&self) -> bool {
        for row in &self.data {
            for value in row {
                if value.len() == 0 {
                    return false;
                }
            }
        }

        return true;
    }

    fn is_assigned(&self, x: usize, y: usize) -> bool {
        is_assigned(&self.data[x][y])
    }

    fn get_assigned(&self, x: usize, y: usize) -> i32 {
        assert!(
            self.data[x][y].len() == 1,
            "should have exactly one value. Values: {:?}",
            self.data[x][y]
        );
        *self.data[x][y]
            .first()
            .expect("There should be at least one value possible for this cell")
    }

    /// For the given assigned cell, remove that value from the possible other values from the
    /// other cells in the same row, column, and 3x3 cell.
    ///
    /// Property: This should never add new possibilities to a cell.
    fn apply_constraints(&mut self, x: usize, y: usize) {
        if !self.is_assigned(x, y) {
            return;
        }
        self.update_row(x, y);
        self.update_col(x, y);
        self.update_cell(x, y);
    }

    /// Applys the contraints of all assigned cells.
    fn apply_constraints_all_cells(&mut self) {
        for row in 0..9 {
            for col in 0..9 {
                self.apply_constraints(row, col);
            }
        }
    }

    fn update_row(&mut self, x: usize, y: usize) {
        let assigned = self.get_assigned(x, y);
        for row in 0..9 {
            // Skip the row where the value is assigned.
            if row == x {
                continue;
            }
            self.data[row][y].remove(&assigned);
        }
    }

    fn update_col(&mut self, x: usize, y: usize) {
        let assigned = self.get_assigned(x, y);
        for col in 0..9 {
            // Skip the row where the value is assigned.
            if col == y {
                continue;
            }
            self.data[x][col].remove(&assigned);
        }
    }

    fn update_cell(&mut self, x: usize, y: usize) {
        let assigned = self.get_assigned(x, y);
        for (row, col) in all_in_large_cell(x, y) {
            if (row, col) == (x, y) {
                continue;
            }
            self.data[row][col].remove(&assigned);
        }
    }

    fn most_constrained_variable(&self) -> Option<(usize, usize)> {
        let mut min_num_vars = None;
        let mut min_cell = None;
        for row in 0..9 {
            for col in 0..9 {
                let cur_cell = &self.data[row][col];
                if is_determined(cur_cell) {
                    continue;
                }
                if min_num_vars
                    .map(|cur_min| self.data[row][col].len() < cur_min)
                    .unwrap_or(true)
                {
                    min_cell = Some((row, col));
                    min_num_vars = Some(self.data[row][col].len());
                }
            }
        }

        min_cell
    }
}

fn is_assigned(cell: &CellAssignment) -> bool {
    cell.len() == 1
}

// A cell is determined when there are 1 or 0 possibilities for assignment.
// 1 means the cell is assigned. 0 Means there is no possible assignment.
fn is_determined(cell: &CellAssignment) -> bool {
    cell.len() <= 1
}

fn most_constraining_fn() {
    todo!()
}

fn all_in_large_cell(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    // The initial starting cell for the 3x3 cell block.
    let row = x - x % 3;
    let col = y - y % 3;
    (0..3).flat_map(move |row_offset| {
        (0..3).map(move |col_offset| (row + row_offset, col + col_offset))
    })
}

fn main() {
    // TODO load from a file.
    // let puzzle = Grid::empty();

    let lines = [
        "x1xxx6xxx",
        "7xx3xx8x5",
        "xxxxxx79x",
        "17x5xxxx9",
        "9x3x27xx8",
        "xxx1xxxxx",
        "8x5xx1x3x",
        "xxx97xx8x",
        "xxxxx59x2",
    ];
    let puzzle = Grid::parse(&lines.join("\n"));
    // TODO fuzz test that none of the assigned cells change.
    // TODO fuzz test that the solution is valid.

    assert!(puzzle.is_valid(), "The puzzle must be valid at the start");
    if let Some(puzzle) = puzzle.solve() {
        println!("{}", puzzle.to_string());
    } else {
        println!("There is no valid solution");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_trivial() {
        // This puzzle is already solved. The solver should just return it.
        let lines = [
            "123456789",
            "456789123",
            "789123456",
            "234567891",
            "567891234",
            "891234567",
            "345678912",
            "678912345",
            "912345678",
        ];
        let puzzle = Grid::parse(&lines.join("\n"));
        assert!(puzzle.solve().is_some());
    }

    #[test]
    fn solve_simple() {
        // There is only one missing value. It should be completely constrained.
        let lines = [
            "123456789",
            "4567x9123",
            "789123456",
            "234567891",
            "567891234",
            "891234567",
            "345678912",
            "678912345",
            "912345678",
        ];
        let puzzle = Grid::parse(&lines.join("\n"));
        assert!(puzzle.solve().is_some());
    }

    #[test]
    fn solve_multiple_missing() {
        let lines = [
            "x234x6789",
            "x567x9123",
            "789123456",
            "234567891",
            "567891234",
            "891234567",
            "345678912",
            "678912345",
            "912345678",
        ];
        let puzzle = Grid::parse(&lines.join("\n"));
        assert!(puzzle.solve().is_some());
    }

    #[test]
    fn solve_with_guess() {
        // This puzzle reqires that some of the variables are guessed.
        let lines = [
            "x23x56789",
            "x56789x23",
            "789123456",
            "234567891",
            "567891234",
            "891234567",
            "345678912",
            "678912345",
            "912345678",
        ];
        let puzzle = Grid::parse(&lines.join("\n"));
        assert_eq!(puzzle.get(0, 0), &empty_cell());
        assert!(puzzle.solve().is_some());
    }

    #[test]
    fn solve_detects_impossible_puzzles() {
        // This puzzle contains a contradiction. It should be easy to report that there is no solution.
        let lines = [
            "11xxxxxxx",
            "xxxxxxxxx",
            "xxxxxxxxx",
            "xxxxxxxxx",
            "xxxxxxxxx",
            "xxxxxxxxx",
            "xxxxxxxxx",
            "xxxxxxxxx",
            "xxxxxxxxx",
        ];
        let puzzle = Grid::parse(&lines.join("\n"));
        assert!(puzzle.solve().is_none());
    }

    #[test]
    fn constrain_rows() {
        let mut grid = Grid::empty();
        grid.data[0][0] = BTreeSet::new();
        grid.data[0][0].insert(1);

        grid.update_row(0, 0);

        for row in 0..9 {
            if row == 0 {
                continue;
            }
            assert!(
                !grid.data[row][0].contains(&1),
                "cell ({}, 0) still contains the value",
                row
            );
        }
    }

    #[test]
    fn constrain_cols() {
        let mut grid = Grid::empty();
        grid.data[0][0] = BTreeSet::new();
        grid.data[0][0].insert(1);

        grid.update_col(0, 0);

        for col in 0..9 {
            if col == 0 {
                continue;
            }
            assert!(
                !grid.data[0][col].contains(&1),
                "cell (0, {}) still contains the value",
                col
            );
        }
    }

    #[test]
    fn constrain_cell() {
        let mut grid = Grid::empty();
        grid.data[3][3] = BTreeSet::new();
        grid.data[3][3].insert(1);

        grid.update_cell(3, 3);

        for row in 3..6 {
            for col in 3..6 {
                if (row, col) == (3, 3) {
                    continue;
                }
                assert!(
                    !grid.data[row][col].contains(&1),
                    "cell ({}, {}) still contains the value",
                    row,
                    col
                );
            }
        }
    }

    #[test]
    fn most_constrainted() {
        let mut grid = Grid::empty();
        grid.data[0][0] = BTreeSet::new();
        grid.data[0][0].insert(1);
        grid.data[0][0].insert(2);
        grid.data[0][0].insert(3);

        // Should skip variable that are already assigned.
        grid.data[3][3] = BTreeSet::new();
        grid.data[3][3].insert(1);

        // Should find this one
        grid.data[4][4] = BTreeSet::new();
        grid.data[4][4].insert(1);
        grid.data[4][4].insert(2);

        assert!(dbg!(grid.most_constrained_variable()) == Some((4, 4)))
    }

    #[test]
    fn parse() {
        let grid = Grid::parse("123456789\n987654321");
        assert_eq!(grid.data[0][0], [1].into());
        assert_eq!(grid.data[0][8], [9].into());
        assert_eq!(grid.data[1][0], [9].into());
        assert_eq!(grid.data[1][8], [1].into());

        assert_eq!(grid.data[2][0], empty_cell());
    }

    #[test]
    fn parse_with_unfilled() {
        let grid = Grid::parse("x2345678x");
        assert_eq!(grid.data[0][0], empty_cell());
        assert_eq!(grid.data[0][8], empty_cell());
    }
}
