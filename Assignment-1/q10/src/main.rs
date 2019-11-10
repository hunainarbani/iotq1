fn main() {
 let a_binding;
 {
    let x = 2;
    // Initialize the binding
    a_binding = x * x;
 }
 println!("a binding: {}", a_binding);
}