fn main() {
    let long_lived_binding = 1;
    // This is a block, and has a smaller scope than the main function
    {
        // This binding only exists in this block
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);
        println!("inner short: {}", long_lived_binding);
    }
    // Error! `short_lived_binding` doesn't exist in this scope
    //println!("outer short: {}", short_lived_binding);cannot find value `short_lived_binding` in this scope
    // FIXME ^ Comment out this line
}
