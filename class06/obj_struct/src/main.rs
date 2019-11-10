#[derive(Debug)]
struct Student{

    first_name : String,
    last_name : String,
    age : i8,
    single : bool,
    height : f32
}

fn main() {
    println!("Hello, world!");

    // let mut student_01 = Student{
    //     first_name : "Muhammad".to_string(),
    //     last_name : "Mujtaba".to_string(),
    //     age : 3,
    //     single : true,
    //     height : 3.0
    // };

    // let student_02 = Student{
    //     first_name : "Muhammad".to_string(),
    //     last_name : "Ahmded".to_string(),
    //     age : 2,
    //     single : true,
    //     height : 3.2
    // };
    let student_01 = create_student("Hunain".to_string(),"Arbani".to_string()
                                     ,32,false,5.11   );
    println!("{:?}",student_01);
    //println!("{:?}",student_02);
}

fn create_student(first_name:String,last_name:String,age:i8,single:bool,height:f32) -> Student{

    Student{
        first_name,
        last_name,
        age,
        single,
        height
    }

    // Student{
    //     first_name : fname,
    //     last_name : lname,
    //     age : age,
    //     single : single,
    //     height : height
    // }
}