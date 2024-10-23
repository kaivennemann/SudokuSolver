use std::collections::HashSet;

pub struct Sudoku<'a> {
    board: &'a mut [[i32; 9]; 9],
    empty_cells: usize
}

impl<'a> Sudoku<'a> {
    pub fn new(board: &'a mut [[i32; 9]; 9]) -> Self {
        let empty_cells = Self::count_empty_cells(board);
        let this = Self {
            board,
            empty_cells
        };

        if !this.is_valid_sudoku() {
            panic!("Invalid Sudoku board");
        }

        this
    }

    fn count_empty_cells(board: &[[i32; 9]; 9]) -> usize {
        let mut empty_cells = 0;
        for row in board.iter() {
            for val in row.iter() {
                if *val == 0 { empty_cells += 1; }
            }
        }
        empty_cells
    }

    pub fn print(&self) {
        for row in self.board.iter() {
            for val in row.iter() {
                print!("{} ", val);
            }
            println!();
        }
    }

    fn get_row(&self, row: usize) -> Vec<i32> {
        let mut row_vals: Vec<i32> = Vec::new();
        for col in 0..9 {
            row_vals.push(self.board[row][col]);
        }
        row_vals
    }

    fn get_column(&self, col: usize) -> Vec<i32> {
        let mut column: Vec<i32> = Vec::new();
        for row in self.board.iter() {
            column.push(row[col]);
        }
        column
    }

    fn get_box_vals(&self, box_num: usize) -> Vec<i32> {
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

    fn get_box_of_cell(&self, row: usize, col: usize) -> Vec<i32> {
        let box_num = (row / 3) * 3 + col / 3;
        self.get_box_vals(box_num)
    }

    fn is_one_to_nine(&self, vec: Vec<i32>) -> bool {
        if vec.len() != 9 {
            return false;
        }
    
        let unique_elements: HashSet<i32> = vec.iter().cloned().collect();
        let one_to_nine: HashSet<i32> = (1..=9).collect();
    
        unique_elements == one_to_nine
    }

    fn is_valid_vector(&self, vec: Vec<i32>) -> bool {
        let mut elements: HashSet<i32> = HashSet::with_capacity(9);
        for val in vec.iter() {
            let result = elements.insert(*val);
            if *val != 0 && (!result || *val > 9 || *val < 1) {
                return false;
            }
        }
        true
    }

    fn is_valid_sudoku(&self) -> bool {
        for i in 0..9 {
            let row_vals = self.get_row(i);
            let col_vals = self.get_column(i);
            let box_vals = self.get_box_vals(i);
            if !self.is_valid_vector(row_vals) ||
                !self.is_valid_vector(col_vals) ||
                !self.is_valid_vector(box_vals)
            {
                return false;
            }
        }
        true
    }

    fn is_fully_solved(&self) -> bool {
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

    fn find_next_free_cell(&self) -> Option<(usize, usize)> {
        for row in 0..9 {
            for col in 0..9 {
                if self.board[row][col] == 0 {
                    return Some((row, col));
                }
            }
        }
        None
    }

    fn get_possible_values(&self, row: usize, col: usize) -> HashSet<i32> {
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
        if self.empty_cells == 0 {
            return self.is_fully_solved();
        } else {
            match self.find_next_free_cell() {
                Some((row, col)) => {
                    self.empty_cells -= 1;
                    for val in self.get_possible_values(row, col) {
                        self.board[row][col] = val;
                        if self.solve() {
                            return true;
                        }
                    }
                    self.board[row][col] = 0;
                    self.empty_cells += 1;
                    return false;
                },
                None => false,
            }
        }
    }
}
