#[derive(Debug)]
enum EngineCC {
    EightHundred,
    TenHundred,
    FifteenHundred,
}
#[derive(Debug)]
struct Vehicle {
    Name :String,
    color :String,
    capacity :EngineCC,
}
impl Vehicle {
    fn vehicle_info(&self) {
        println!("Vehicle Name : {}\nVehicle Color : {}\nVehicle Capacity : {}"
        ,self.Name,self.color
        ,match self.capacity {
                EngineCC::EightHundred => "800cc",
                EngineCC::TenHundred => "1000cc",
                EngineCC::FifteenHundred =>"1500cc"
            }        
        )
    }
}

// #[derive(Debug)]
// enum shapes{
//     Rectangle(l :f64,w :f64),
//     Circle(r :f64),
//     Square(l :f64),
// }

fn main (){

    println!("{:0}",divide(12.0,2.0.0));
    match divide(12.0,0.0) {
        Some(x) => println!("The result is {}",x),
        None => println!("Invalid divisor"),
    }

//     let mehran = EngineCC::EightHundred;
//     let dragonR = EngineCC::TenHundred;
//     let SR = EngineCC::FifteenHundred;

//     match mehran {
//         EngineCC::EightHundred => println!("Mehran"),
//         EngineCC::TenHundred => println!("DragonR"),
//         EngineCC::FifteenHundred => println!("Civic Turbo")
//     }

//     let vehicle_01 = Vehicle{Name: "Civic".to_string(),
//                     color: "Red".to_string(),
//                     capacity: EngineCC::FifteenHundred};

//    vehicle_01.vehicle_info();

}

fn divide(x:f64,y:f64) -> Option<f64>{

    if y == 0.0{
        None
    }
    else {
        Some(x/y)
    }
}