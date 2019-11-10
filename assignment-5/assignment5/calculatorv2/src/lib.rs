#[macro_use] extern crate text_io;
pub mod calculator{

pub fn calculate(){

        
        loop {
            println!("Enter the function you wish to perform\n1)addition\n2)subtraction\n3)multiplication\n4)division\n5)division\n6)quit");
            let opt :i8 = read!();
            if opt == 6 {
                break;
            } 
            match opt{
                1 => addition(),
                //2 => subtraction(),
                //3 => multiplication(),
                //4 => division(),                
                //5 => exponential(), 
                _ => println!("Bye..")
            }           
        }
    }

    fn is_valid_number(digit :u32) -> bool{
        
        let mut result :bool = false;

        if digit >= 48 && digit <= 57 {
            result = true;
        }
        result
    }

    fn addition(){

        println!("Please enter expression..");
        // let sometext :String = read!();
        use std::{io};
        let mut s = String::new();
        io::stdin().read_line(&mut s)
        .expect("Failed to read your input");

        let lbytes = s.as_bytes();
        let vexpression = s.as_bytes();
        let mut result :f64 = 0.0;
        let mut sstr :String = "".to_string();
        let mut digit :f64 = 0.0;
        for (i, &item) in lbytes.iter().enumerate() {
                // let ctype :u32 = &s[0..i].trim().parse().unwrap();      
                // if is_valid_number(ctype){
                // digit = &s[0..i].to_string().trim().parse().unwrap();            
                // sstr = sstr + &vchar.to_string() + &"+".to_string();
                // result += digit;                
                //}
                println!("{}",&item);
            
        }
    }
}