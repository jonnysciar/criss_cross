use std::io;

use crate::board::CrissCross;
mod board;

fn main() {
    println!("Welcome to Rust CrissCross game!");
    let mut board: board::Board = Default::default();
    println!("{}", board);
    board.add_o(0, 0);
    board.add_x(1, 1);
    board.add_o(2, 2);
    println!("{}", board);
    
    let mut buffer = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.

    loop {
        let res = stdin.read_line(&mut buffer);
        if let Ok(size) = res {
            println!("Echo: {}, Byte: {}", buffer.trim_end(), size);
            buffer.clear();
        }

    }
}
