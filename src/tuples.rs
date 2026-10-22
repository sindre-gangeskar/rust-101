pub fn tuples() {
    let tup: (f32, &str, bool) = (2.0, "Coming from tuple!", true);
    println!("Tuple variable: {:#?}", tup);

    let (float_var, string_var, bool_var) = tup;

    println!("Destructured from tuple:");
    println!("{float_var} {string_var} {bool_var}")
}
