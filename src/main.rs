use std::io;

mod arrays;
mod guessing_game;
mod shadowing;
mod tuples;
mod variables;
fn main() -> io::Result<()> {
    let mut skip_guessing_game = String::new();

    println!("Skip guessing game?");
    io::stdin().read_line(&mut skip_guessing_game)?;

    let will_skip: bool = match skip_guessing_game.trim().parse() {
        Ok(value) => value,
        Err(_) => false,
    };

    if !will_skip {
        guessing_game::game()?;
    }
    variables::variables();
    shadowing::shadowing();
    tuples::tuples();
    arrays::arrays();
    Ok(())
}
