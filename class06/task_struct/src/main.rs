#[derive(Debug)]
struct Car{

    car_name : String,
    color : String,
    cc : i16,
    model : String,
    transmission : String
}

fn main() {
    
    let car_01 :Car = create_car(
        "Alto VXR".to_string(),
        "Blue".to_string(),
        660,
        "2019".to_string(),
        "Automatic".to_string()
    );
}

fn create_car(car_name :String,
    color :String,
    cc :i16,
    model :String,
    transmission :String) -> Car {

        Car = {
            car_name : car_name,
            color : color,
            cc : cc,
            model : model,
            transmission : transmission

        }

    }
