fn main() {
    let a : i32 =125;
    let b : i32 = 12345;
    let ax : i64 = 1234567890;
    let s : i16 =4043;
    let x : f32 = 2.13459;
    let dx : f64 = 1.1415927;
    let c : char = 'W';
    let ux : u64 = 2541567890;
    println!("a + c is {}",  a + c as i32); 
    println!("x + c is {}",  x as i32 + c as i32); 
    println!("dx + x is {}",  dx + x as f64);
    println!("a + x is {}",  a + x as i32 );
    println!("s + b is {}",  s + b as i16);
    println!("ax + b is {}",  ax + b as i64);
    println!("s + c is {}",  s as i32 + c as i32); 
    println!("ax + c is {}",  ax as i32 + c as i32); 
    println!("ax + ux is {}",  ax + ux as i64);
}
