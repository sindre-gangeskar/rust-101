pub fn variables() -> i32 {
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
