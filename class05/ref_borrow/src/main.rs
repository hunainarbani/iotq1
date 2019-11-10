fn main() {
    let mut _str = String::from("Hello");
    

    change(&mut _str);
    let _len = calc_length(& _str);

    println!("The length of {} is {}", _str,_len);

    let mut s = String :: from("hello");
    //{
    let r1 = &mut s;
    println!("{}",r1);
    //}

    
    let r2 = &mut s;
    println!("{}",r2);
    //println!("{} , {}",r1,r2); 

}

fn change(_str: &mut String) {

    _str.push_str(", World")
}

fn calc_length(_str :&String) -> usize {

    _str.len()
}
