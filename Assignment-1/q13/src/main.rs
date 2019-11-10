use std::io;
fn main() {
        println!("Please enter your name!!");
        let mut uname = String::new();
        io::stdin().read_line(&mut uname).expect("Failed to read line");
        let uname = uname.trim();

        println!("Please enter your Age!!");
        let mut uage = String::new();
        io::stdin().read_line(&mut uage).expect("Failed to read line");                
        let uage: i32 = uage.trim().parse().unwrap();
        
        println!("Please enter your Mobile Number!!");
        let mut umobile = String::new();
        io::stdin().read_line(&mut umobile).expect("Failed to read line");

        //let umobile: i64 = umobile.trim().parse().unwrap();

        println!("Welcome {}, your Age is {} and Mobile Number is {}", uname,uage,umobile);
}
