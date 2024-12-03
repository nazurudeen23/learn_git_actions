use std::io;

const EMPTY: char = ' ';

/// Represents the Tic Tac Toe board
struct Board {
    cells: [[char; 3]; 3],
}

impl Board {
    fn new() -> Self {
        Self {
            cells: [[EMPTY; 3]; 3],
        }
    }

    fn display(&self) {
        println!("\nCurrent Board:");
        for row in self.cells.iter() {
            println!(" {} | {} | {} ", row[0], row[1], row[2]);
            println!("---+---+---");
        }
    }

    fn is_full(&self) -> bool {
        self.cells.iter().all(|row| row.iter().all(|&cell| cell != EMPTY))
    }

    fn place_mark(&mut self, row: usize, col: usize, mark: char) -> Result<(), String> {
        if row >= 3 || col >= 3 {
            return Err("Invalid position! Choose between 0 and 2.".to_string());
        }
        if self.cells[row][col] != EMPTY {
            return Err("Cell is already occupied.".to_string());
        }
        self.cells[row][col] = mark;
        Ok(())
    }

    fn check_winner(&self) -> Option<char> {
        for i in 0..3 {
            // Check rows
            if self.cells[i][0] == self.cells[i][1]
                && self.cells[i][1] == self.cells[i][2]
                && self.cells[i][0] != EMPTY
            {
                return Some(self.cells[i][0]);
            }
            // Check columns
            if self.cells[0][i] == self.cells[1][i]
                && self.cells[1][i] == self.cells[2][i]
                && self.cells[0][i] != EMPTY
            {
                return Some(self.cells[0][i]);
            }
        }
        // Check diagonals
        if self.cells[0][0] == self.cells[1][1]
            && self.cells[1][1] == self.cells[2][2]
            && self.cells[0][0] != EMPTY
        {
            return Some(self.cells[0][0]);
        }
        if self.cells[0][2] == self.cells[1][1]
            && self.cells[1][1] == self.cells[2][0]
            && self.cells[0][2] != EMPTY
        {
            return Some(self.cells[0][2]);
        }
        None
    }
}

fn main() {
    let mut board = Board::new();
    let mut current_player = 'X';

    println!("Welcome to Tic Tac Toe!");

    loop {
        board.display();

        println!("Player '{}', enter your move (row and column): ", current_player);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input: Vec<&str> = input.trim().split_whitespace().collect();

        if input.len() != 2 {
            println!("Please enter row and column numbers separated by a space.");
            continue;
        }

        let row: usize = match input[0].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid row input. Please enter a number between 0 and 2.");
                continue;
            }
        };

        let col: usize = match input[1].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid column input. Please enter a number between 0 and 2.");
                continue;
            }
        };

        match board.place_mark(row, col, current_player) {
            Ok(_) => {
                if let Some(winner) = board.check_winner() {
                    board.display();
                    println!("Player '{}' wins!", winner);
                    break;
                }
                if board.is_full() {
                    board.display();
                    println!("It's a draw!");
                    break;
                }
                current_player = if current_player == 'X' { 'O' } else { 'X' };
            }
            Err(err) => println!("{}", err),
        }
    }
}
