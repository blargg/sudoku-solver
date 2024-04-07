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

impl Grid<CellAssignment> {
    fn solve(&self) -> Self {
        if self.complete() {
            return self.clone();
        }

        let mut puzzle = self.clone();
        puzzle.apply_constraints_all_cells();

        let (row, col) = puzzle.most_constrained_variable().unwrap();
        for possible in puzzle.data[row][col].iter() {
        }
        todo!("finish the solver");
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
                if !cell.is_empty() {
                    return false;
                }
            }
        }

        return true;
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
        self.update_row(x, y);
        self.update_col(x, y);
        self.update_cell(x, y);
    }

    /// Applys the contraints of all assigned cells.
    fn apply_constraints_all_cells(&mut self) {
        for row in 1..=9 {
            for col in 1..=9 {
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
                if is_assigned(cur_cell) {
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
    let puzzle = Grid::empty();
    // TODO, this should probably be checked in the initial solve method.
    assert!(puzzle.is_valid(), "The puzzle must be valid at the start");
    let solved = puzzle.solve();
    println!("{:?}", solved.data);
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
