use std::{cmp::Ordering, io};
fn main() -> io::Result<()> {
    let mut skip_guessing_game = String::new();

    println!("Skip guessing game?");
    io::stdin().read_line(&mut skip_guessing_game)?;
    let will_skip: bool = match skip_guessing_game.trim().parse() {
        Ok(value) => value,
        Err(_) => false,
    };

    println!("Guess the number between 1-10");
    loop {
        if will_skip {
            println!("You chose to skip the guessing game!");
            break;
        };
        let correct_number: u32 = rand::random_range(0..=10);
        let mut guess = String::new();
        println!("The actual answer is: {:?}", correct_number);

        io::stdin().read_line(&mut guess)?;

        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Please use numbers only!");
                continue;
            }
        };
        match guess.cmp(&correct_number) {
            Ordering::Less => println!("Guess higher!"),
            Ordering::Equal => {
                println!("You got it!");
                break;
            }
            Ordering::Greater => println!("You're guessing too high!"),
        };
    }
    variables();
    Ok(())
}

fn variables() -> i32 {
    // Mutables
    let mut x = 5;
    println!("The x variable value is: {x}");
    x = 3;
    println!("The new x variable value is: {x}");

    // Reference on immutable by borrow
    let y: i32 = 2;
    let ref_y: &i32 = &y;
    // De-reference and return its value. Essentially return a copy of y
    *ref_y
}
