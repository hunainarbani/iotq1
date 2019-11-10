use std::io;
fn main() {
    
    println!("Please input your marks obtained..");
    let mut omarks = String::new();
    io::stdin().read_line(&mut omarks).expect("Failed to read line");
    println!("Please input your total marks..");
    let mut tmarks = String::new();
    io::stdin().read_line(&mut tmarks).expect("Failed to read line");

    let x: f32 =  omarks as f32; 
    let n: f32 = tmarks as f32;
    let p: f32 = omarks / tmarks * 100.0;

    println!("Your have secured {}%",p);

}