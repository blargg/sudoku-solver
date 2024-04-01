use std::collections::BTreeSet;

struct Grid<A> {
    data:[[A;9];9],
}

type CellAssignment = BTreeSet<i32>;

impl Grid<CellAssignment> {
    fn solve(&self) -> Self {
        todo!()
    }

    fn get_assigned(&self, x: usize, y: usize) -> i32 {
        assert!(self.data[x][y].len() == 1, "should have exactly one value");
        *self.data[x][y].first().expect("There should be at least one value possible for this cell")
    }

    fn update_row(&mut self, x: usize, y: usize) {
        let assigned = self.get_assigned(x,y);
        for row in 0..9 {
            // Skip the row where the value is assigned.
            if row == x { continue; }
            self.data[row][y].remove(&assigned);
        }
    }

    fn update_col(&mut self, x: usize, y: usize) {
        let assigned = self.get_assigned(x,y);
        for col in 0..9 {
            // Skip the row where the value is assigned.
            if col == y { continue; }
            self.data[x][col].remove(&assigned);
        }
    }

    fn update_cell(&mut self, x: usize, y: usize) {
        let assigned = self.get_assigned(x,y);
        for (row, col) in all_in_large_cell(x, y) {
            if (row, col) == (x,y) { continue;}
            self.data[row][col].remove(&assigned);
        }
    }

}

fn most_constrainted_variable() { todo!() }

fn most_constraining_fn() { todo!() }

fn all_in_large_cell(x: usize, y: usize) -> impl Iterator<Item=(usize, usize)> { 
    // The initial starting cell for the 3x3 cell block.
    let row = x / 3;
    let col = y / 3;
    (0..3).flat_map(move |row_offset| { 
        (0..3).map(move |col_offset| {
        (row + row_offset, col + col_offset)
        })
    })
}

fn main() {
    println!("Hello, world!");
}
