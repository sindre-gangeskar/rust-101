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
    shadowing();
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
    // De-reference and return its value. Return a copy of y
    *ref_y
}

fn shadowing() {
    // Set the initial variable
    let x = 5;
    println!("The initial x variable is: {x}");
    // Shadow x by defining another x variable with its own assignment that includes the original x
    let x = x + 5;
    println!("The shadowed x variable is: {x}");

    // Shadow x inside a scope for edge-cases and lasts only until the block ends
    {
        let x = x * 2;
        println!("The shadowed x variable of the inner scope is: {x}");
    }
    // Shadow x as a different type all together inside its own scope.
    {
        let x = "this string right here inside its own scope";
        println!("x shadowed as a string value: {x}");
    }
    // Prints the very first shadow of the x variable.
    println!("x variable at the end of the shadowing function: {x}");
}
