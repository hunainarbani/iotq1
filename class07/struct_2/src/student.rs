#[derive(Debug)]
struct Student {
    first_name: String,
    last_name: String,
    age: i8,
    height: f32,
    maritalstatus: String
}

impl Student {

    pub mod get_student_info(&self) -> String {

        format!("First Name : {}\nLastName : {}\nAge : {}\nHeight : {}\nMaritalStatus : {}",
        self.first_name,self.last_name,self.age,self.height,self.maritalstatus)        
    }


}
