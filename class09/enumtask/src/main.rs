extern crate reqwest;
use reqwest;
use std::collections::HashMap;
enum DaysofWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

// fn is_weekend(dow : DaysofWeek){

//     match (dow=)
// }

fn main() {
    let resp: HashMap<String, String> = reqwest::get("https://httpbin.org/ip")?
        .json()?;
    println!("{:#?}", resp);
    Ok(())
}
