#[macro_use] extern crate mysql;
pub mod db {
    use mysql as sql;

    const CON_STRING: &str = "mysql://root:@localhost:3306/test";

    #[derive(Debug, PartialEq, Eq)]
    pub struct Students{
        pub id :i32,
        pub student_name :Option<String>,
        pub email :Option<String>,
        pub age :Option<i8>,
        pub marital_status :Option<String>
    }

    impl Students{

        pub fn getStruct (id :i32,
                student_name :String,
                email :String,
                age :i8,
                marital_status :String) -> Students{

            
            let student_data :Students= Students {
                    id : id,
                    student_name: Some(student_name),
                    email:Some(email),
                    age:Some(age),
                    marital_status:Some(marital_status)
                };
           

        student_data
    }
        pub fn new (id :i32,
                student_name :String,
                email :String,
                age :i8,
                marital_status :String){

            let _pool = sql::Pool::new(CON_STRING).unwrap();


            let student_data = vec![
                Students {
                    id : id,
                    student_name: Some(student_name),
                    email:Some(email),
                    age:Some(age),
                    marital_status:Some(marital_status)
                }
        ];    

        let query :String = r"insert into student_master (student_name,email,age,marital_status) values
                                        (:student_name,:email,:age,:marital_status)".to_string();
        for mut stmt in _pool.prepare(query
                                    ).into_iter(){

            for s in student_data.iter(){

                stmt.execute(params!{
                    //"id" => s.id,
                    "student_name" => &s.student_name,
                    "email" => &s.email,
                    "age" => s.age,
                    "marital_status" => &s.marital_status

                }).unwrap();
            }

            };
        }

        pub fn select(id :i32) -> Vec<Students>{//1 by Id,2 by Name
            //let mut selected_students: Vec<Students>;
            let mut query = "".to_string();
            if id == 0 {                
                query = "SELECT id,student_name,email,age,marital_status from student_master".to_string();
               // println!("{}",query);
            }
            else {
                query = "SELECT id,student_name,email,age,marital_status from student_master where id = ".to_string() + &id.to_string();
                //println!("{}",query);
            }
            let _pool = sql::Pool::new(CON_STRING).unwrap();
            //if searchby == 1 {
                let selected_students: Vec<Students> =
                _pool.prep_exec(query, ())
                .map(|result| {
                    result.map(|x| x.unwrap()).map(|row| {
                        // ⚠️ Note that from_row will panic if you don't follow your schema
                        let (id,student_name,email,age,marital_status) = sql::from_row(row);
                        Students {
                            id: id,
                            student_name: student_name,
                            email: email,
                            age: age,
                            marital_status :marital_status
                        }
                    }).collect()
                }).unwrap();
           // }
            // else {

            // }

            selected_students

        }

        pub fn update(s :Students){

            let _pool = sql::Pool::new(CON_STRING).unwrap();


            let query :String = "UPDATE student_master set student_name = :student_name,
                                email = :email,
                                age = :age,
                                marital_status = :marital_status
                                where id = :id".to_string();
            for mut stmt in _pool.prepare(query
                                    ).into_iter(){
                stmt.execute(params!{
                    "id" => s.id,
                    "student_name" => &s.student_name,
                    "email" => &s.email,
                    "age" => s.age,
                    "marital_status" => &s.marital_status

                }).unwrap();            
            };

        }

        pub fn delete(id :i32){
                let _pool = sql::Pool::new(CON_STRING).unwrap();


            let query :String = "DELETE FROM student_master where id = :id".to_string();
            println!("{}",query);
            for mut stmt in _pool.prepare(query
                                    ).into_iter(){
                stmt.execute(params!{
                    "id" => id//,
                    // "student_name" => &s.student_name,
                    // "email" => &s.email,
                    // "age" => s.age,
                    // "marital_status" => &s.marital_status
                }).unwrap();            
            };

        }
    }
}