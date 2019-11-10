// #[macro_use]
// extern crate text_io;
fn main() {
    // let i: i32 = read!();

    let _at :(i32,i32,i32) = (23,43,54);

    let s = (_at.0 + _at.1 + _at.2)/2;
    let r = (s *(s-_at.0)*(s - _at.1)*(s - _at.2)) as f64;
    let area = r.sqrt();
    println!("Area of triangle is {}",area);


    let sides : AreaTriangle = AreaTriangle {

        a: 23,
        b: 43,
        c: 54
    };

    let ss = (sides.a + sides.b + sides.c)/2;
    let rs = (ss *(ss- sides.a)*(ss - sides.b)*(ss - sides.c)) as f64;
    let areas = rs.sqrt();
    println!("Area of triangle is {}",areas);    

    let sides = AreaTriangle{a:23,b:43,c:54};
    println!("Area of triangle is {}",sides.calculate_area());    

    let circle_01: Circle = Circle { radius: 23.0};
    println!("Area of Circle is {}",circle_01.calculate_area());
    println!("Circumfarance of Circle is {}",circle_01.calculate_circumfarance());
    println!("Diameter of Circle is {}",circle_01.calculate_diameter());


    let student_01 = Student {
        first_name: "Hunain".to_string(),
        last_name: "Arbani".to_string(),
        age: 22,
        height: 5.11,
        maritalstatus: "Married".to_string()
    };

    println!("Student Info {}",student_01.get_student_info());

    let rect_01 = Rectangle {
        l: 4,
        w:6
    };

    println!("Area of rectangle {}", rect_01.calculate_area());


}
#[derive(Debug)]
struct AreaTriangle {
    a:i32,
    b:i32,
    c:i32
}

impl AreaTriangle{

    fn calculate_area(self) -> f64 {

        let s = (self.a + self.b + self.c)/2;        
        let r = (s *(s-self.a)*(s - self.b)*(s - self.c)) as f64;
        r.sqrt()
    }
}

#[derive(Debug)]
struct Circle {
    radius: f64
}

impl Circle{

    fn calculate_diameter(&self) -> f64 {

        2.0 * self.radius
    }

    fn calculate_circumfarance(&self) -> f64 {

        2.0 * self.radius * std::f64::consts::PI
    }

    fn calculate_area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powf(2.0)
    }
}

#[derive(Debug)]
struct Student {
    first_name: String,
    last_name: String,
    age: i8,
    height: f32,
    maritalstatus: String
}

impl Student {

    fn get_student_info(&self) -> String {

        format!("First Name : {}\nLastName : {}\nAge : {}\nHeight : {}\nMaritalStatus : {}",
        self.first_name,self.last_name,self.age,self.height,self.maritalstatus)        
    }


}

#[derive(Debug)]
struct Rectangle{
    l: i32,
    w: i32
}

impl Rectangle {

    fn calculate_area(self) -> i32 {
        self.l * self.w
    }
}