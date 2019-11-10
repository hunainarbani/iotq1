use std::io;
fn main() {
        println!("Please enter your Days!!");   

        let mut days = String::new();
        io::stdin().read_line(&mut days).expect("Failed to read line");                
        let days: i32 = days.trim().parse().unwrap();


        let year: i32 = days / 365;
        let weeks: i32 = (days % 365) /7;


        println!("Years {}\nWeeks {}",year,weeks);
}
