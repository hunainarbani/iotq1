#[macro_use] extern crate text_io;
extern crate shapes;
extern crate numbers;
extern crate stringtype;
extern crate calculator;
extern crate calculatorv2;
//use q1::circle as cc;
fn main() {
    
    println!("Please enter Question Number you want to test!!");
    let ques: i8 = read!();
    match ques{
        1 => shapes::circle::cricle_area(),
        2 => numbers::numbers::check_number_sign(),
        3 => numbers::numbers::check_divisble(),
        4 => shapes::sphere::volume_sphere(),
        5 => stringtype::strings::text_repeat(),
        6 => numbers::numbers::check_even_odd(),
        7 => stringtype::strings::check_vowel(),
        8 => shapes::triangle::area_of_triangle(),
        9 => numbers::calculations::calc_interest(),
        10 => numbers::calculations::eucl_dist(),
        11 => numbers::calculations::feet_to_cm(),
        12 => numbers::calculations::calc_bmi(),
        13 => numbers::calculations::sum_pos_int(),
        14 => numbers::calculations::sum_digits(),
        15 => numbers::calculations::decimal_to_binary(),
        16 => numbers::calculations::binary_to_decimal(),
        17 => stringtype::strings::count_vow_const(),
        18 => stringtype::strings::is_palindrome(),
        19 => stringtype::strings::calc_char_type(),
        20 => calculatorv2::calculator::calculate(),
        _ => println!("Bye..")
    }
    
}

