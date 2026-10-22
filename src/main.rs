use std::{cmp::Ordering, io};
fn main() -> io::Result<()> {
    println!("Guess the number between 1-10");

    loop {
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
    Ok(())
}
