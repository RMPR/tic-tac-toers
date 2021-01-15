use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    print!("Enter something \n > ");
    stdout.flush();
    println!("{}", stdin.lock().lines().next().unwrap().unwrap());
    /*
    for line in stdin.lock().lines() {
        println!("{}", line.unwrap());
    }
    */
}
