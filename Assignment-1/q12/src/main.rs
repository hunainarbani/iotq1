use std::io;
fn main() {
        println!("Please enter your name!!");

        let mut uname = String::new();
        io::stdin().read_line(&mut uname).expect("Failed to read line");
        println!("Welcome Mr. {}", uname);
}
