#[macro_use] extern crate text_io;
pub mod calculator{

    pub fn calculate(){

        
        loop {
            println!("Enter the function you wish to perform\n1)addition\n2)subtraction\n3)multiplication\n4)division\n5)quit");
            let opt :i8 = read!();
            if opt == 5 {
                break;
            } 
            match opt{
                1 => addition(),
                2 => subtraction(),
                3 => multiplication(),
                4 => division(),                
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

        println!("Please enter numbers seprated by space");
        // let sometext :String = read!();
        use std::{io};
        let mut sometext = String::new();
        io::stdin().read_line(&mut sometext)
        .expect("Failed to read your input");

        let mut ctype :u32 = 0;
        let mut result :f64 = 0.0;
        let mut sstr :String = "".to_string();
        let mut digit :f64 = 0.0;
        for vchar in sometext.trim().chars(){
            ctype = vchar as u32;            

            if ctype == 32 {
                
            }
            else if is_valid_number(ctype){
                digit = vchar.to_string().trim().parse().unwrap();            
                sstr = sstr + &vchar.to_string() + &"+".to_string();
                result += digit;
            }
            else{
                println!("Invalid Input");
            }

        }

        println!("{} = {}",&sstr[0..sstr.len()-1],result);

        loop {
            println!("Are you want to try again?\n1)Yes\n2)No");
            let again :i8= read!();
            
            if again == 1{
                addition();
                break;
            }
            else{
                break;
            }

        }

    }

    fn subtraction(){

        println!("Please enter numbers seprated by space");
        // let sometext :String = read!();
        use std::{io};
        let mut sometext = String::new();
        io::stdin().read_line(&mut sometext)
        .expect("Failed to read your input");

        let mut ctype :u32 = 0;
        let mut result :f64 = 0.0;
        let mut sstr :String = "".to_string();
        let mut digit :f64 = 0.0;
        for vchar in sometext.trim().chars(){
            ctype = vchar as u32;            

            if ctype == 32 {
                
            }
            else if is_valid_number(ctype){
                digit = vchar.to_string().trim().parse().unwrap();            
                sstr = sstr + &vchar.to_string() + &"-".to_string();
                if result == 0.0 {
                    result = digit;
                }
                else {
                    result -= digit;
                }
            }
            else{
                println!("Invalid Input");
            }

        }

        println!("{} = {}",&sstr[0..sstr.len()-1],result);

        loop {
            println!("Are you want to try again?\n1)Yes\n2)No");
            let again :i8= read!();
            
            if again == 1{
                subtraction();
                break;
            }
            else{
                break;
            }

        }

    }

    fn multiplication(){

        println!("Please enter numbers seprated by space");
        // let sometext :String = read!();
        use std::{io};
        let mut sometext = String::new();
        io::stdin().read_line(&mut sometext)
        .expect("Failed to read your input");

        let mut ctype :u32 = 0;
        let mut result :f64 = 1.0;
        let mut sstr :String = "".to_string();
        let mut digit :f64 = 0.0;
        for vchar in sometext.trim().chars(){
            ctype = vchar as u32;            

            if ctype == 32 {
                
            }
            else if is_valid_number(ctype){
                digit = vchar.to_string().trim().parse().unwrap();            
                sstr = sstr + &vchar.to_string() + &"x".to_string();
                result *= digit;
            }
            else{
                println!("Invalid Input");
            }

        }

        println!("{} = {}",&sstr[0..sstr.len()-1],result);

        loop {
            println!("Are you want to try again?\n1)Yes\n2)No");
            let again :i8= read!();
            
            if again == 1{
                multiplication();
                break;
            }
            else{
                break;
            }

        }

    }

    fn division(){

        println!("Please enter numbers seprated by space");
        // let sometext :String = read!();
        use std::{io};
        let mut sometext = String::new();
        io::stdin().read_line(&mut sometext)
        .expect("Failed to read your input");

        let mut ctype :u32 = 0;
        let mut result :f64 = 0.0;
        let mut sstr :String = "".to_string();
        let mut digit :f64 = 0.0;
        for vchar in sometext.trim().chars(){
            ctype = vchar as u32;            

            if ctype == 32 {
                
            }
            else if is_valid_number(ctype){
                digit = vchar.to_string().trim().parse().unwrap();            
                sstr = sstr + &vchar.to_string() + &"/".to_string();
                if result == 0.0 {
                    result = digit;
                }
                else {
                    result /= digit
                };
            }
            else{
                println!("Invalid Input");
            }

        }

        println!("{} = {}",&sstr[0..sstr.len()-1],result);

        loop {
            println!("Are you want to try again?\n1)Yes\n2)No");
            let again :i8= read!();
            
            if again == 1{
                division();
                break;
            }
            else{
                break;
            }

        }

    }

}