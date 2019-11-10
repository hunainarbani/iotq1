#[macro_use] extern crate text_io;

pub mod strings{
    
    pub fn text_repeat(){

        println!("Please enter String!!");
        let text :String = read!();

        println!("How many copies of String you need?");
        let n :i8 = read!();
        let mut rtext = "".to_string();
        for x in 0..n{
            rtext = rtext.to_string() + &text;
        }
        println!("{}",rtext);
    }
    
    fn is_vowel(vchar :char) -> bool{


        match vchar{
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false
        }

    }

    pub fn check_vowel(){

        println!("Please enter a character!!");
        let vchar :char = read!();

        if is_vowel(vchar){
            println!("Letter {} is vowel",vchar);
        }
        else{
            println!("Letter {} is not vowel",vchar);
        }

    }

    pub fn count_vow_const(){

        println!("Please enter some text");
        let sometext :String = read!();

        let mut vcount = 0;
        let mut ccount = 0;

        for vchar in sometext.chars(){
            
            if is_vowel(vchar){
               vcount += 1;
            }
            else{
                ccount += 1;
            }            
        }

        println!("Vowels {}",vcount);
        println!("Constans {}",ccount);

    }

    pub fn is_palindrome(){

        println!("Please enter some text");
        let sometext :String = read!();
        let mut ptext :String = "".to_string();

        for txt in sometext.chars(){
            ptext = txt.to_string() + &ptext;
        }

        if sometext == ptext {
            println!("Entered text {} is Palindrome",sometext);
        }
        else{
            println!("Entered text {} is not Palindrome",sometext);
        }        
    }

    pub fn calc_char_type(){

        println!("Please enter some text");
        // let sometext :String = read!();
        use std::{io};
        let mut sometext = String::new();
        io::stdin().read_line(&mut sometext)
        .expect("Failed to read your input");

        let mut ctype :u32 = 0;
        let mut nums = 0;
        let mut alpha = 0;
        let mut space = 0;
        let mut spec = 0;
        for vchar in sometext.trim().chars(){
            ctype = vchar as u32;            
            if (ctype == 32){
                space += 1;
            }
            else if (ctype >= 48 && ctype <= 57){
                nums += 1;
            }
            else if (ctype >= 65 && ctype <= 90 ) || (ctype >= 97 && ctype <= 122 ) {
                alpha +=1;
            }
            else{
                spec += 1;
            }
        }

        println!("Numbers = {}",nums);
        println!("Alphabets = {}" ,alpha);
        println!("Special Characters = {}",spec);
        println!("Spaces = {}",space);

    }
}