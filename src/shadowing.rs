pub fn shadowing() {
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
