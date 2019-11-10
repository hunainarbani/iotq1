//extern crate ha_lib;
//use iot_io::read_input as io;
extern crate student;
use student::student_register as stud;
use student::teacher_register as techr;
fn main (){

    /*
    ha_lib::print_text();
    println!("Please enter your Name");
    let text = io::read();
    println!("Welcome {}!!",text.trim());
    */
    let student_01 = stud::Student::new("Hunain Arbani".to_string(),
        "ha@yahoo.com".to_string(),
        32,
        false
    );

    println!("{}",student_01.get_student());    


    let teacher_01 = techr::Teacher::new("Hunain Arbani".to_string(),
        "ha@yahoo.com".to_string(),
        32,
        false
    );

    println!("{}",teacher_01.get_teacher());    
}