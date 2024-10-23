use std::collections::HashSet;

pub struct Sudoku<'a> {
    board: &'a mut [[i32; 9]; 9]
}

impl<'a> Sudoku<'a> {
    pub fn new(board: &'a mut [[i32; 9]; 9]) -> Self {
        Self { board }
    }

    pub fn print(&self) {
        for row in self.board.iter() {
            for val in row.iter() {
                print!("{} ", val);
            }
            println!();
        }
    }

    pub fn get_row(&self, row: usize) -> Vec<i32> {
        let mut row_vals: Vec<i32> = Vec::new();
        for col in 0..9 {
            row_vals.push(self.board[row][col]);
        }
        row_vals
    }

    pub fn get_column(&self, col: usize) -> Vec<i32> {
        let mut column: Vec<i32> = Vec::new();
        for row in self.board.iter() {
            column.push(row[col]);
        }
        column
    }

    pub fn get_box_vals(&self, box_num: usize) -> Vec<i32> {
        let mut box_vals: Vec<i32> = Vec::new();
        let row_start = (box_num / 3) * 3;
        let col_start = (box_num % 3) * 3;

        for row in row_start..row_start + 3 {
            for col in col_start..col_start + 3 {
                box_vals.push(self.board[row][col]);
            }
        }

        box_vals
    }

    pub fn get_box_of_cell(&self, row: usize, col: usize) -> Vec<i32> {
        let box_num = (row / 3) * 3 + col / 3;
        self.get_box_vals(box_num)
    }

    pub fn is_one_to_nine(&self, vec: Vec<i32>) -> bool {
        if vec.len() != 9 {
            return false;
        }
    
        let unique_elements: HashSet<i32> = vec.iter().cloned().collect();
        let one_to_nine: HashSet<i32> = (1..=9).collect();
    
        unique_elements == one_to_nine
    }

    pub fn is_fully_solved(&self) -> bool {
        for i in 0..9 {
            let row_vals = self.get_row(i);
            let col_vals = self.get_column(i);
            let box_vals = self.get_box_vals(i);
            if !self.is_one_to_nine(row_vals) ||
                !self.is_one_to_nine(col_vals) ||
                !self.is_one_to_nine(box_vals)    
            {
                return false;
            }
        }
        true
    }

    pub fn find_next_free_cell(&self) -> Option<(usize, usize)> {
        for row in 0..9 {
            for col in 0..9 {
                if self.board[row][col] == 0 {
                    return Some((row, col));
                }
            }
        }
        None
    }

    pub fn get_possible_values(&self, row: usize, col: usize) -> HashSet<i32> {
        let mut possible_values: HashSet<i32> = (1..=9).collect();
        for val in self.get_row(row) {
            possible_values.remove(&val);
        }
        for val in self.get_column(col) {
            possible_values.remove(&val);
        }
        for val in self.get_box_of_cell(row, col) {
            possible_values.remove(&val);
        }
        possible_values
    }

    pub fn solve(&mut self) -> bool { // need &mut to allow mutating in place
        if self.is_fully_solved() {
            return true;
        } else {
            match self.find_next_free_cell() {
                Some((row, col)) => {
                    for val in self.get_possible_values(row, col) {
                        self.board[row][col] = val;
                        if self.solve() {
                            return true;
                        }
                        self.board[row][col] = 0;
                    }
                    return false;
                },
                None => false,
            }
        }
    }
}
