//use std::io;
fn main() {
    // let avail = false;

    // if avail {    
    //     println!("I'm available here!!")
    // }
    // else if !avail{
    //     println!("I'm at Home!!")
    // }
    // else {
    //     println!("I'm not available here!!")
    // }


    // let num = 14;

    // if num % 2 == 0 {
    //     println!("The number is Even..");
    // }
    // else {
    //     println!("The number is Odd..");
    // }

    // let num = num - 1;

    // if num % 2 != 0 {
    //     println!("The number is Odd..");
    // }
    // else {
    //     println!("The number is Even..");
    // }

    // number max divided by 12,6,3,2

    let modnum = 27;
    if modnum % 12 == 0 {
        println!("HCM of {} is 12",modnum);
    }
    else if modnum % 6 == 0 {
        println!("HCM of {} is 6",modnum);
    }
    else if modnum % 3 == 0 {
        println!("HCM of {} is 3",modnum);
    }
    else if modnum % 2 == 0 {
        println!("HCM of {} is 2",modnum);
    }
    else {
        println!("HCM of {} is neither 12 nor 6 nor 3 nor 2",modnum);
    }

    // let mut avail = String::new();
    // println!("Where are you? For Baharia press B, for Home press H else O.");
    // io::stdin().read_line(&mut avail).expect("Failed to read line");
    // println!("{}",avail);
    
    // if avail == "B" {
    //     println!("Your are at Bahria Auditorium..");
    // }
    // else if avail == "H"{
    //     println!("Your are at Home.");
    // }
    // else {
    //     println!("Your are not at Bahria Auditorium or Home");
    // }



}
