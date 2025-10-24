#![deny(missing_docs)]
#![deny(clippy::missing_docs_in_private_items)]
#![allow(clippy::too_many_arguments)]

//! This crate is housing various different methods for solving Sudoku boards.

/// Imports for the board.
use std::{collections::BTreeSet, fs, process::exit, time::Instant};

/// The enumeration for specifying the algorithm solution type.
pub enum SolvingMethod {

    /// The naive method, a recursive solution with O(n^2) complexity.
    Naive,

    /// A custom method, that revolves around eliminating possibilities before assigning.
    BaxStrat,
}

/// Data structure containing the board state and memory.
#[derive(Default)]
pub struct Board {

    /// The cells of the board containing values.
    cells: Vec<Vec<u8>>,

    /// The NxN dimensions of the board.
    size: usize,

    /// The local block size within the board.
    segment_size: usize,

    /// The memory structure of all potential values that could be put into the cell.
    memory: Vec<Vec<BTreeSet<u8>>>,

    /// The maximum value that can be emplaced into the board.
    max_val: u8,
}

impl Board {

    /// Initializes the board given its current size.
    fn init(&mut self) {
        self.cells = vec![vec![0; self.size]; self.size];
        self.memory = vec![vec![BTreeSet::new(); self.size]; self.size];
        self.max_val = (self.size + 1) as u8;
        self.segment_size = (self.size as f32).sqrt() as usize;
    }

    /// Loads in board data from a file.
    ///
    /// Arguments:
    /// - `filename`: [`String`] &rarr; The location of the file to read data from.
    pub fn load(&mut self, filename: String) {

        // Ensure the file exists.
        if !fs::exists(&filename).expect("Can check for file") {
            println!("Given file `{filename}`, does not exist.");
            exit(0);
        }

        // Read in the file data.
        let data = match fs::read_to_string(&filename) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Cannot read from {filename}, ({e})");
                exit(1);
            }
        };

        let mut iterator = data.split_whitespace();

        // Grab the first number, which indicates the size of the board.
        self.size = match iterator.next() {
            Some(v) => match v.parse::<usize>() {
                Ok(s) => s,
                Err(_) => {
                    eprintln!("Board size is not a number");
                    exit(1);
                }
            },
            None => {
                eprintln!("Cannot read board size from file.");
                exit(1);
            }
        };

        // Initialize the board give the size information.
        self.init();

        let mut values = vec![];

        // If file data cannot be consumed, report an error and exit.
        for val in iterator {
            match val.parse::<u8>() {
                Ok(v) => values.push(v),
                Err(_) => {
                    eprintln!("Cannot parse value `{val}` into an integer");
                    exit(1);
                }
            }
        }

        if values.len() != self.size*self.size {
            println!("Loaded data did not match the expected value count of `{}`.", self.size*self.size);
            exit(1);
        }

        // Assign values.
        self.loads(values);
    }

    /// Assigns given values to the board, used for initializing data.
    ///
    /// Arguments:
    /// - `values`: [`Vec<u8>`] &rarr; The values to assign.
    pub fn loads(&mut self, values: Vec<u8>) {
        let mut index = 0;
        for row in 0..self.size {
            for col in 0..self.size {
                self.cells[row][col] = values[index];
                index += 1;
            }
        }
    }

    /// The function called into by `main` in order to solve the given board.
    ///
    /// Arguments:
    /// - `method`: [`SolvingMethod`] &rarr; The algorithm selection to solve the given board.
    pub fn solve(&mut self, method: SolvingMethod) {

        // Get the current time so as to record the solving duration.
        let now = Instant::now();

        // Run the selected algorith.
        let solution = match method {
            SolvingMethod::Naive => self.solve_naive(0, 0),
            SolvingMethod::BaxStrat => self.solve_baxstrat(),
        };

        // Get the duration.
        let elapsed = now.elapsed();

        // If a solution was found, indicate success, failure otherwise.
        if solution {
            println!("A solution was found in {elapsed:?}!");
            self.print_board();
        } else {
            println!("No solution could be found with the given board");
        }
    }

    /// Neatly prints the current board to `stdout`.
    fn print_board(&self) {

        // Get the number of decimal digits the values are.
        let mult = ((self.size as f64).log10().floor() as usize) + 1;

        // Get the overall length of the print board.
        let overall_length = 
            (self.size * mult)        // There are `n` values of `m` digits.
            + (2 * self.segment_size) // Accounts for the `|` between blocks.
            + 3                       // Accounts for board edges.
            + (self.size - 1);        // Accounts for the spaces between values.

        // Extra newline.
        println!();

        // For each row in the board..
        for (ind, row) in self.cells.iter().enumerate() {

            // If there is a new block, print a line.
            if ind.is_multiple_of(self.segment_size) {
                print!(" ");
                for _ in 1..overall_length {
                    print!("-");
                }
                println!();
            }

            // For each column in the board..
            for (col_ind, val) in row.iter().enumerate() {

                // If there is a new block, print a vertical bar.
                if col_ind.is_multiple_of(self.segment_size) {
                    print!(" |");
                }

                // Get the decimal digit size of the current number and determine the number of
                // spaces to print before it.
                let local_size = mult - (*val as f64).log10().floor() as usize;

                // Print those spaces.
                for _ in 1..local_size {
                    print!(" ");
                }

                // Print the value.
                print!(" {val}");
            }
            // Print the edge of the board.
            println!(" |");
        }

        // Print the bottom line of the board.
        print!(" ");
        for _ in 1..overall_length {
            print!("-");
        }
        println!();

    }

    /// Checks the current row if the given value does not exist within that row.
    ///
    /// Arguments:
    /// - `val`: [`u8`] &rarr; The value to check against.
    /// - `pos_y`: [`usize`] &rarr; The row index to iterate through.
    ///
    /// Returns: [`bool`]
    /// - `true` &rarr; The `val` is *not* present within the row.
    /// - `false` &rarr; The `val` *is present* within the row.
    fn check_row(&self, val: u8, pos_y: usize) -> bool {
        for col in 0..self.size {
            if self.cells[pos_y][col] == val {
                return false;
            }
        }

        true
    }

    /// Checks the current column if the given value does not exist within that column.
    ///
    /// Arguments:
    /// - `val`: [`u8`] &rarr; The value to check against.
    /// - `pos_x`: [`usize`] &rarr; The column index to iterate through.
    ///
    /// Returns: [`bool`]
    /// - `true` &rarr; The `val` is *not* present within the column.
    /// - `false` &rarr; The `val` *is present* within the column.
    fn check_col(&self, val: u8, pos_x: usize) -> bool {
        for row in 0..self.size {
            if self.cells[row][pos_x] == val {
                return false;
            }
        }

        true
    }

    /// Checks the local block if the given value does not exist within that block;
    ///
    /// Arguments:
    /// - `val`: [`u8`] &rarr; The value to check against.
    /// - `row`: [`usize`] &rarr; The row index of the current cell.
    /// - `col`: [`usize`] &rarr; The column index of the current cell.
    ///
    /// Returns: [`bool`]
    /// - `true` &rarr; The `val` is *not* present within the local block.
    /// - `false` &rarr; The `val` *is present* within the local block.
    fn check_block(&self, val: u8, col: usize, row: usize) -> bool {

        // Get the 'block' position index of the board.
        let block_start_row = row / self.segment_size;
        let block_start_col = col / self.segment_size;

        // Get the top left position of the block.
        let block_start_row_index = block_start_row * self.segment_size;
        let block_start_col_index = block_start_col * self.segment_size;

        // Search through the block.
        for row_prime in block_start_row_index..block_start_row_index + self.segment_size {
            for col_prime in block_start_col_index..block_start_col_index + self.segment_size {

                // Skip ourselves
                if col_prime == col && row_prime == row {
                    continue;
                }

                if self.cells[row_prime][col_prime] == val {
                    return false;
                }
            }
        }

        true
    }

    /// Runs the functions [`Self::check_row`], [`Self::check_col`], and [`Self::check_block`] in
    /// sequence.
    ///
    /// Arguments:
    /// - `val`: [`u8`] &rarr; The value to check against.
    /// - `row`: [`usize`] &rarr; The row index of the current cell.
    /// - `col`: [`usize`] &rarr; The column index of the current cell.
    ///
    /// Returns: [`bool`]
    /// - `true` &rarr; The `val` was *not* found.
    /// - `false` &rarr; The `val` *was found*.
    fn check(&self, val: u8, col: usize, row: usize) -> bool {
        self.check_row(val, row) && self.check_col(val, col) && self.check_block(val, col, row)
    }

    /// Runs a recursive algorithm to solve the given board. If a cell is already populated, it is
    /// skipped.
    ///
    /// The base case is if the row size is greater than or equal to the board size, indicating
    /// that end of the board has been surpassed.
    ///
    /// Arguments:
    /// - `row`:[`usize`] &rarr; The row index for the given cell.
    /// - `col`:[`usize`] &rarr; The col index for the given cell.
    ///
    /// Returns: [`bool`]
    /// - `true` &rarr; If assigning a cell was successful.
    /// - `false` &rarr; If no assignment of a cell was possible.
    fn solve_naive(&mut self, row: usize, col: usize) -> bool {

        // If passed the board size, return.
        if row >= self.size {
            return true;
        }

        // If the cell is already assigned, return.
        if self.cells[row][col] != 0 {

            // If at the end of a row, increase the row count, not col.
            if col + 1 >= self.size {
                return self.solve_naive(row + 1, 0);
            } else {
                return self.solve_naive(row, col + 1);
            }
        }

        // For each potential value that could be assigned..
        for val in 1..self.max_val {

            // Check via [`Self::check`] to determine if assignment is valid.
            if self.check(val, col, row) {

                // Assign the value as `val` is currently valid.
                self.cells[row][col] = val;

                // Recursively run algorithm on the next cell.
                if col + 1 >= self.size {

                    // Check for a solution in the next row.
                    if !self.solve_naive(row + 1, 0) {
                        // A solution was not found, reassign to `0`.
                        self.cells[row][col] = 0;
                    } else {
                        // A solution was found.
                        break;
                    }
                } else if !self.solve_naive(row, col + 1) {
                    // A solution was not found, reassign to `0`.
                    self.cells[row][col] = 0;
                } else {
                    // A solution was found.
                    break;
                }
            }
        }

        // Return if the current cell has been assigned.
        self.cells[row][col] != 0
    }

    /// Removes a given value from board memory in the specified row.
    ///
    /// Arguments:
    /// - `val`: [`u8`] &rarr; The value to remove from row memory.
    /// - `row`: [`u8`] &rarr; The row to to iterate across.
    fn mem_row_remove(&mut self, val: u8, row: usize) {
        for i in 0..self.size {
            self.memory[row][i].remove(&val);
        }
    }

    /// Removes a given value from board memory in the specified col.
    ///
    /// Arguments:
    /// - `val`: [`u8`] &rarr; The value to remove from column memory.
    /// - `col`: [`u8`] &rarr; The col to to iterate across.
    fn mem_col_remove(&mut self, val: u8, col: usize) {
        for i in 0..self.size {
            self.memory[i][col].remove(&val);
        }
    }

    /// Removes a given value from board memory in the current local block.
    ///
    /// Arguments:
    /// - `val`: [`u8`] &rarr; The value to remove from memory.
    /// - `block_start_row_index`: [`usize`] &rarr; The left most index of the local block.
    /// - `block_end_row_index`: [`usize`] &rarr; The right most index of the local block.
    /// - `block_start_col_index`: [`usize`] &rarr; The top most index of the local block.
    /// - `block_end_col_index`: [`usize`] &rarr; The bottom most index of the local block.
    fn mem_block_remove(
        &mut self,
        val: u8,
        block_start_row_index: usize,
        block_end_row_index: usize,
        block_start_col_index: usize,
        block_end_col_index: usize,
    ) {
        for row_ind in block_start_row_index..block_end_row_index {
            for col_ind in block_start_col_index..block_end_col_index {
                self.memory[row_ind][col_ind].remove(&val);
            }
        }
    }

    /// Removes a given value from board memory in accordance to the current position. To be used
    /// after assigning a cell a particular value, i.e. removing potential values from other cells.
    ///
    /// Runs the [`Self::mem_row_remove`], [`Self::mem_col_remove`], and
    /// [`Self::mem_block_remove`] functions in sequence.
    ///
    /// Arguments:
    /// - `val`: [`u8`] &rarr; The value to remove from memory.
    /// - `row`: [`u8`] &rarr; The row to to iterate across.
    /// - `col`: [`u8`] &rarr; The col to to iterate across.
    /// - `block_start_row_index`: [`usize`] &rarr; The left most index of the local block.
    /// - `block_end_row_index`: [`usize`] &rarr; The right most index of the local block.
    /// - `block_start_col_index`: [`usize`] &rarr; The top most index of the local block.
    /// - `block_end_col_index`: [`usize`] &rarr; The bottom most index of the local block.
    fn mem_remove(
        &mut self,
        val: u8,
        row: usize,
        col: usize,
        block_start_row_index: usize,
        block_end_row_index: usize,
        block_start_col_index: usize,
        block_end_col_index: usize,
    ) {
        self.mem_row_remove(val, row);
        self.mem_col_remove(val, col);
        self.mem_block_remove(
            val,
            block_start_row_index,
            block_end_row_index,
            block_start_col_index,
            block_end_col_index,
        );
    }

    /// Verifies that a given value is the only existing instance of that value within memory for
    /// the given row, column and local block.
    ///
    /// Arguments:
    /// - `val`: [`u8`] &rarr; The value to check against from memory.
    /// - `row`: [`u8`] &rarr; The row to to iterate across, and the current memory cell.
    /// - `col`: [`u8`] &rarr; The col to to iterate across, and the current memory cell.
    /// - `block_start_row_index`: [`usize`] &rarr; The left most index of the local block.
    /// - `block_end_row_index`: [`usize`] &rarr; The right most index of the local block.
    /// - `block_start_col_index`: [`usize`] &rarr; The top most index of the local block.
    /// - `block_end_col_index`: [`usize`] &rarr; The bottom most index of the local block.
    ///
    /// Returns: [`bool`]
    /// - `true` &rarr; That `val` is the only instance within that memory cell.
    /// - `false` &rarr; That `val` was found elsewhere in conflicting memory cells.
    fn only_val_in_block(
        &mut self,
        val: u8,
        row: usize,
        col: usize,
        block_start_row_index: usize,
        block_end_row_index: usize,
        block_start_col_index: usize,
        block_end_col_index: usize,
    ) -> bool {

        // Return immediately if current memory cell doesn't even contain the value in question.
        if !self.memory[row][col].contains(&val) {
            return false;
        }

        // Check row indices 
        for row_ind in block_start_row_index..block_end_row_index {

            // Check column indices
            for col_ind in block_start_col_index..block_end_col_index {

                // Ignore self.
                if col_ind == col && row_ind == row {
                    continue;
                }

                // A different memory cell has the value in question, return immediately.
                if self.memory[row_ind][col_ind].contains(&val) {
                    return false;
                }
            }
        }

        // No instances of `val` were found in the local block.
        // Assign value to cell, clear cell memory, and remove value from row and column memory cells.
        self.cells[row][col] = val;
        self.memory[row][col].clear();
        self.mem_remove(
            val,
            row,
            col,
            block_start_row_index,
            block_end_row_index,
            block_start_col_index,
            block_end_col_index,
        );

        true
    }

    /// Searches through the row, column, and local block given a row index and column index to
    /// search for the pressence of a given value within cell memory. If a specific pattern of
    /// value appearances are present, clear cell memory to eliminate possibilities for cells.
    ///
    /// Arguments:
    /// - `val`: [`u8`] &rarr; The value to check against from memory.
    /// - `row`: [`u8`] &rarr; The row to to iterate across, and the current memory cell.
    /// - `col`: [`u8`] &rarr; The col to to iterate across, and the current memory cell.
    /// - `block_start_row_index`: [`usize`] &rarr; The left most index of the local block.
    /// - `block_end_row_index`: [`usize`] &rarr; The right most index of the local block.
    /// - `block_start_col_index`: [`usize`] &rarr; The top most index of the local block.
    /// - `block_end_col_index`: [`usize`] &rarr; The bottom most index of the local block.
    ///
    /// Returns: [`bool`]
    /// - `true` &rarr; A change was made to memory cells.
    /// - `false` &rarr; No changes were made to memory cells.
    fn check_mem(
        &mut self,
        val: u8,
        row: usize,
        col: usize,
        block_start_row_index: usize,
        block_end_row_index: usize,
        block_start_col_index: usize,
        block_end_col_index: usize,
    ) -> bool {

        // Return immediately if current memory cell doesn't even contain the value in question.
        if !self.memory[row][col].contains(&val) {
            return false;
        }

        // Boolean to keep track of row instances in memory cells.
        let mut found_in_row = false;

        // Boolean to keep track of column instances in memory cells.
        let mut found_in_col = false;

        // Boolean to keep track of local block instances in memory cells, not in the same row and
        // column.
        let mut found_else = false;

        // Iterate across all the cells in the local block.
        for row_ind in block_start_row_index..block_end_row_index {
            for col_ind in block_start_col_index..block_end_col_index {

                // Ignore self.
                if row_ind == row && col_ind == col {
                    continue;
                }

                // If same value is within other memory cells in local block..
                if self.memory[row_ind][col_ind].contains(&val) {

                    // In the same row?
                    if row_ind == row {
                        found_in_col = true;

                    // In the same column?
                    } else if col_ind == col {
                        found_in_row = true;

                    // Otherwise?
                    } else {
                        found_else = true;
                    }
                }
            }
        }

        // Values in memory cells not in the same row and column but in local block the same
        // cannot neatly be decidable according to the elimination method, therefore just return
        // false.
        if found_else {
            return false;
        }

        // Values in memory cells that are in the same row and in the same column as the indexing
        // memory cell cannot neatly be decidable according to the elimination method, therefore
        // just return false.
        if found_in_row && found_in_col {
            return false;
        }

        // Were any updates made?
        let mut changed = false;

        if found_in_row {

            for row_ind in 0..self.size {
                if row_ind >= block_start_row_index
                    && row_ind < block_start_row_index + self.segment_size
                {
                    // Ignore the current local block.
                    continue;
                }

                // Remove memory cell instances of `val` from the row.
                if self.memory[row_ind][col].contains(&val) {
                    self.memory[row_ind][col].remove(&val);
                    changed = true;
                }
            }

        } else if found_in_col {

            for col_ind in 0..self.size {
                if col_ind >= block_start_col_index
                    && col_ind < block_start_col_index + self.segment_size
                {
                    // Ignore the current local block.
                    continue;
                }

                // Remove memory cell instances of `val` from the column.
                if self.memory[row][col_ind].contains(&val) {
                    self.memory[row][col_ind].remove(&val);
                    changed = true;
                }
            }

        }

        changed
    }


    /// Runs an algorithm to eliminate possibilities from the board. If a cell has only one
    /// possibility of being some particular value, that cell is automatically populated as that
    /// value. When the board can no longer be updated or eliminate possibilities, the board is
    /// passed to the naive method, [`Self::solve_naive`], to fill the remaining unassigned cells.
    ///
    /// Returns: [`bool`]
    /// - `true` &rarr; A board solution was findable.
    /// - `false` &rarr; A board solution was not findable.
    fn solve_baxstrat(&mut self) -> bool {

        // Populate the memory cells with all potential values.
        for row in 0..self.size {
            for col in 0..self.size {
                if self.cells[row][col] != 0 {
                    continue;
                }

                for val in 1..self.max_val {

                    // Make simple checks against values that are in the board, not memory cells.
                    if self.check(val, col, row) {
                        self.memory[row][col].insert(val);
                    }
                }
            }
        }

        let mut updated = true;

        while updated {

            // Loop will terminate if no board eliminations could be done.
            updated = false;

            // Values are the starting loop because this allows subsequent checks to run faster
            // than iterating over each cell first. This allows for whole assignable values to
            // essentially be skipped which helps with performance.

            // For every assignable value..
            for i in 1..self.max_val {

                // For each row..
                for row in 0..self.size {

                    // Calculate the local block row indices
                    let block_start_row = row / self.segment_size;
                    let block_start_row_index = block_start_row * self.segment_size;
                    let block_end_row_index = block_start_row_index + self.segment_size;

                    // For each column..
                    for col in 0..self.size {

                        // If the cell has a value, skip.
                        if self.cells[row][col] != 0 {
                            continue;
                        }

                        // Calculate the local block column indices.
                        let block_start_col = col / self.segment_size;
                        let block_start_col_index = block_start_col * self.segment_size;
                        let block_end_col_index = block_start_col_index + self.segment_size;

                        // Identify if the memory cell has only one entry, i.e. it *must* be this
                        // value.
                        if self.memory[row][col].len() == 1 {

                            // Pop the value.
                            let v = self.memory[row][col].pop_first().unwrap();

                            // Assign the value.
                            self.cells[row][col] = v;

                            // remove from all directional memory cells
                            self.mem_remove(
                                v,
                                row,
                                col,
                                block_start_row_index,
                                block_end_row_index,
                                block_start_col_index,
                                block_end_col_index,
                            );

                            // Updated and skip to next cell.
                            updated = true;
                            continue;
                        }

                        // Identify if the value is the only instance within the local block.
                        if self.only_val_in_block(
                            i,
                            row,
                            col,
                            block_start_row_index,
                            block_end_row_index,
                            block_start_col_index,
                            block_end_col_index,
                        ) {
                            // Updated and skip to next cell.
                            updated = true;
                            continue;
                        }

                        // Identify if the value only exists in a particular row or column and
                        // therefore can eliminate that value from memory cells in other local
                        // blocks on the board.
                        if self.check_mem(
                            i,
                            row,
                            col,
                            block_start_row_index,
                            block_end_row_index,
                            block_start_col_index,
                            block_end_col_index,
                        ) {
                            // Updated and go to next cell.
                            updated = true;
                        }
                    }
                }
            }
        }

        // All reductions have been made, brute force any remaining unassigned cells.
        self.solve_naive(0, 0)
    }
}
