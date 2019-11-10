mod announcment{

    pub fn announcment_students(){
        println!("This announcment is for Student");
    }

    pub fn announcment_teacher(){
        println!("This announcment is for Teacher");
    }
} 

pub mod student_register{

    #[derive(Debug)]
    pub struct Student{
        name: String,
        email: String,
        age: i16,
        single: bool
    }

    impl Student {

        pub fn new(name: String,
        email: String,
        age: i16,
        single: bool) -> Student{
            Student{
                name,
                email,
                age,
                single
            }
        }
        pub fn get_student(&self) -> String {

            super::announcment::announcment_students();
            format!("Name {}\nEmail{}\nAge {}\nSingle {}",self.name,self.email,self.age,self.single)

        }

    }
}

pub mod teacher_register{

    #[derive(Debug)]
    pub struct Teacher{
        name: String,
        email: String,
        age: i16,
        single: bool
    }

    impl Teacher {

        pub fn new(name: String,
        email: String,
        age: i16,
        single: bool) -> Teacher{
            Teacher{
                name,
                email,
                age,
                single
            }
        }
        pub fn get_teacher(&self) -> String {

            super::announcment::announcment_teacher();
            format!("Name {}\nEmail{}\nAge {}\nSingle {}",self.name,self.email,self.age,self.single)

        }

    }
}