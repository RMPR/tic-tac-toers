use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut _board: [[bool; 3]; 3] = [[false; 3]; 3];

    print!("Enter something \n > ");
    let _ = stdout.flush();
    println!("{}", stdin.lock().lines().next().unwrap().unwrap());
    /*
    for line in stdin.lock().lines() {
        println!("{}", line.unwrap());
    }
    */
}
