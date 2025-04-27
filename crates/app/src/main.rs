//! docs

use ttt::BoardIndex;
use ttt::Game;

fn main() {
    let mut game = Game::default();
    loop {
        println!("{game}");
        if let Some(winner) = game.has_winner() {
            println!("Game over, {winner} wins!");
            return;
        }
        println!("Pick a place to play");
        let stdin = std::io::stdin();
        let mut line = String::new();
        match stdin.read_line(&mut line) {
            Ok(_) => (),
            Err(e) => println!("Error reading from stdin: {e}"),
        }
        match line.trim().parse::<u8>() {
            Ok(num) => {
                if num == 0 {
                    println!("Input error: provide an positive number (1-9)");
                    continue;
                }
                match BoardIndex::try_from(num - 1) {
                    Ok(idx) => match game.play_turn(idx) {
                        Ok(()) => println!("Played at position {num}"),
                        Err(_) => println!(
                            "Input error: invalid index {idx}. Provide a positive number (1-9)"
                        ),
                    },
                    Err(e) => println!("{e}"),
                }
            }
            Err(_) => println!("Input error: provide an positive number (1-9)"),
        }
    }
}
