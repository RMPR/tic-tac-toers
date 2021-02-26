use std::io::{self, BufRead, Write};

#[derive(Copy, Clone)]
enum Mark {
    /*
     * States of a square
     */
    Empty,
    Cross,
    Circle,
}

fn print_board(board: [[Mark; 3]; 3]) {
    /*
     * Prints the current state of the board
     */
    for line in board.iter() {
        for cell in line.iter() {
            match cell {
                Mark::Empty => print!("|   |"),
                Mark::Cross => print!("| X |"),
                Mark::Circle => print!("| ◯ |"),
            }
        }
        println!();
    }
}

fn verify_board(board: [[Mark; 3]; 3]) -> bool {
    /*
     * Verify if there's a winner
     */
    true
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut board: [[Mark; 3]; 3] = [[Mark::Empty; 3]; 3];

    print_board(board);

    print!("Enter something \n > ");
    let _ = stdout.flush();
    println!("{}", stdin.lock().lines().next().unwrap().unwrap());
    /*
    for line in stdin.lock().lines() {
        println!("{}", line.unwrap());
    }
    */
}
