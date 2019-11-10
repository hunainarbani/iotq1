mod vehicles;
use vehicles::vehicle_main;
fn main() {
    
    let vech_01: vehicle_main::Vehicle = vehicle_main::input_vehicle {
         Name: "Suzuki".to_string(),
         Model: "Vitz".to_string(),
         Capacity: 1500,
         Color: "Red".to_string(),
         Transmission: "Automatic".to_string()
    };

    //println!("{}",vech_01.)

}
