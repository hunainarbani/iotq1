#[macro_use] extern crate text_io;
pub mod circle {

    pub fn cricle_area() {

        println!("Please enter radius of Circle!!");
        let radius :f32 = read!();
        let area :f32 = std::f64::consts::PI as f32 * radius * radius;
        println!("Area of Circle with radius {} is {}",radius,area);

    }
    
}
pub mod sphere {

    pub fn volume_sphere() {

        println!("Please enter radius of Sphere!!");
        let radius :f32 = read!();
        let volume :f32 = (4.0/3.0) * std::f64::consts::PI as f32 * radius * radius * radius;
        println!("Volume of the Sphere with Radius {} is {}",radius,volume);

    }
}

pub mod triangle{

    pub fn area_of_triangle(){

        println!("Enter magnitude of Triangle base");
        let b :f32 = read!();
        println!("Enter Magnitude of Triangle Height");
        let h :f32 = read!();

        let area = b*h/2.0;

        println!("Area of a Triangle with Height {} and Base {} is {}",h,b,area);
    }
}