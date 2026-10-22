pub fn arrays() {
    // Initialize an array that contains 5 elements with the same initial value
    let default_value_arr = [3; 5];
    println!("Printing from array that has 5 elements with initial value: {default_value_arr:?}");
    // Set the type that is stored in the array, and right-hand is the fixed length of the array
    let arr: [i32; 5] = [0, 1, 2, 3, 4];
    println!("Printing from array that contains custom data: {arr:?}");

    for n in arr {
        println!("{n}");
    }
    // Index out of bounds - .get() returns None instead of panicking
    let i = 10;

    // If index placement is not out of bounds
    if let Some(value) = arr.get(i) {
        println!("{value}");
    } else {
        println!("Index {i} is out of bounds (length is {})", arr.len());
    }
}
