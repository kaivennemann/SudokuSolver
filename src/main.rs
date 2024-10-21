use std::collections::HashSet;


fn print_matrix<T: std::fmt::Display, const ROWS: usize, const COLS: usize>(
    matrix: &[[T; COLS]; ROWS]
) {
    for row in matrix.iter() {
        for val in row.iter() {
            print!("{} ", val);
        }
        println!();
    }
}

fn is_valid_sudoku(matrix: &[[i32; 9]; 9]) -> bool {
    for i in 0..9 {
        let row_vals = get_row(matrix, i);
        let col_vals = get_column(matrix, i);
        let box_vals = get_box(matrix, i);
        if !is_valid_sudoku_collection(row_vals) ||
            !is_valid_sudoku_collection(col_vals) ||
            !is_valid_sudoku_collection(box_vals)
        {
            return false;
        }
    }
    true
}

fn is_fully_solved(sudoku: &[[i32; 9]; 9]) -> bool {
    for i in 0..9 {
        let row_vals = get_row(sudoku, i);
        let col_vals = get_column(sudoku, i);
        let box_vals = get_box(sudoku, i);
        if !is_one_to_nine(row_vals) ||
            !is_one_to_nine(col_vals) ||
            !is_one_to_nine(box_vals)    
        {
            return false;
        }
    }
    true
}

fn is_valid_sudoku_collection(vec: Vec<i32>) -> bool {
    let mut elements: HashSet<i32> = HashSet::with_capacity(9);
    for val in vec {
        let result = elements.insert(val);
        if !result || val < 0 || val > 9 {  // we allow 0 as a placeholder
            return false;
        }
    }
    true
}

fn is_one_to_nine(vec: Vec<i32>) -> bool {
    if vec.len() != 9 {
        return false;
    }

    let unique_elements: HashSet<i32> = vec.iter().cloned().collect();
    let one_to_nine: HashSet<i32> = (1..=9).collect();

    unique_elements == one_to_nine
}

fn get_row<T: Copy, const ROWS: usize, const COLS: usize>(
    matrix: &[[T; COLS]; ROWS],
    row: usize
) -> Vec<T> {
    let mut vals: Vec<T> = Vec::with_capacity(COLS);
    for col in 0..COLS {
        vals.push(matrix[row][col]);
    }
    vals
}

fn get_column<T: Copy, const ROWS: usize, const COLS: usize>( // trait bound Copy to ensure we can copy
    matrix: &[[T; COLS]; ROWS],
    col: usize
) -> Vec<T> {
    let mut vals: Vec<T> = Vec::with_capacity(ROWS);
    for row in 0..ROWS {
        vals.push(matrix[row][col]);
    }
    vals
}

fn get_submatrix<T: Copy, const ROWS: usize, const COLS: usize>(
    matrix: &[[T; COLS]; ROWS],
    col_start: usize,
    col_end: usize,
    row_start: usize,
    row_end: usize
) -> Vec<T> { 
    let size = (col_end - col_start) * (row_end - row_start);
    let mut vals: Vec<T> = Vec::with_capacity(size);

    for row in row_start..row_end {
        for col in col_start..col_end {
            vals.push(matrix[row][col]);
        }
    }

    vals
}

fn get_box<T: Copy + std::fmt::Display>(
    matrix: &[[T; 9]; 9], box_num: usize
) -> Vec<T> {
    let row_start = (box_num / 3) * 3;
    let row_end = row_start + 3;
    let col_start = (box_num % 3) * 3;
    let col_end = col_start + 3;
    
    get_submatrix(matrix, col_start, col_end, row_start, row_end)
}

fn get_box_of_cell(sudoku: &[[i32; 9]; 9], row: usize, col: usize) -> Vec<i32> {
    let box_col_index = col / 3;
    let box_row_index = row / 3;
    let box_num = box_row_index * 3 + box_col_index;
    get_box(sudoku, box_num)
}

fn flatten<T: Copy + std::fmt::Display, const ROWS: usize, const COLS: usize>(
    matrix: &[[T; COLS]; ROWS]
) -> Vec<T> {
    let mut vals: Vec<T> = Vec::with_capacity(ROWS * COLS);
    for row in 0..ROWS {
        for col in 0..COLS {
            vals.push(matrix[row][col]);
        }
    }
    vals
}

fn solve(sudoku: &mut [[i32; 9]; 9]) -> bool { // need &mut to allow mutating in place
    if is_fully_solved(&sudoku) {
        return true;
    } else {
        match find_next_free_cell(&sudoku) {
            Some((row, col)) => {
                for val in get_possible_values(&sudoku, row, col) {
                    sudoku[row][col] = val;
                    if solve(sudoku) {
                        return true;
                    }
                    sudoku[row][col] = 0;
                }
                return false;
            },
            None => false,
        }
    }
}

fn find_next_free_cell(sudoku: &[[i32; 9]; 9]) -> Option<(usize, usize)> {
    for row in 0..9 {
        for col in 0..9 {
            if sudoku[row][col] == 0 {
                return Some((row, col));
            }
        }
    }
    None
}

fn get_possible_values(sudoku: &[[i32; 9]; 9], row: usize, col: usize) -> HashSet<i32> {
    let mut possible_values: HashSet<i32> = (1..=9).collect();
    for val in get_row(sudoku, row) {
        possible_values.remove(&val);
    }
    for val in get_column(sudoku, col) {
        possible_values.remove(&val);
    }
    for val in get_box_of_cell(sudoku, row, col) {
        possible_values.remove(&val);
    }
    possible_values
}

fn main() {
    let mut sudoku = [
        [8, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 3, 6, 0, 0, 0, 0, 0],
        [0, 7, 0, 0, 9, 0, 2, 0, 0],
        [0, 5, 0, 0, 0, 7, 0, 0, 0],
        [0, 0, 0, 0, 4, 5, 7, 0, 0],
        [0, 0, 0, 1, 0, 0, 0, 3, 0],
        [0, 0, 1, 0, 0, 0, 0, 6, 8],
        [0, 0, 8, 5, 0, 0, 0, 1, 0],
        [0, 9, 0, 0, 0, 0, 4, 0, 0],
    ];
    
    let is_solved = solve(&mut sudoku);

    println!("Solved: {}", is_solved);
    print_matrix(&sudoku);
}
