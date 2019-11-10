fn main() {
    
    println!("The sum of two number is {}",addnumbers(15,20));

    println!("The square of 3 is {}",getsq(3));

    gethelloworld();

    printname("Hunain Arbani".to_string());

    println!("The square of given number is {}",getsqbyinput());

    use std::io;
    println!("Please enter first number!!");
    let mut _input01 = String::new();  
    io::stdin().read_line(&mut _input01).expect("Failed to read line"); 
    let _input01 :i32  = _input01.trim().parse().unwrap();

    println!("Please enter second number!!");
    let mut _input02 = String::new();  
    io::stdin().read_line(&mut _input02).expect("Failed to read line"); 
    let _input02 :i32  = _input02.trim().parse().unwrap();

    println!("{}",comaprenumbers(_input01,_input02).to_string());
    println!("Highest Number is {}",comapareandreturn(_input01,_input02));


    let _mix_data = take_full_name();
    println!("{:?}",_mix_data);

    println!("Please enter first number!!");
    let mut _x = String::new();  
    io::stdin().read_line(&mut _x).expect("Failed to read line"); 
    let _x :i32  = _x.trim().parse().unwrap();

    println!("Please enter second number!!");
    let mut _y = String::new();  
    io::stdin().read_line(&mut _y).expect("Failed to read line"); 
    let _y :i32  = _y.trim().parse().unwrap();

    let _arth_data = arthm_operations(_x,_y);


    let mut _n = String::new();  
    io::stdin().read_line(&mut _n).expect("Failed to read line"); 
    let _n :i32  = _n.trim().parse().unwrap();


    let _data = get_sq_cube(_n);

    println!("Sq of {} is {} and cube {}",_n,_data.0,_data.1);


}
fn addnumbers (x:i32,y:i32) -> i32{
    let sum;
    sum = x+y;
    sum
}

fn gethelloworld(){

    println!("Hello World!!");
}

fn getsq (x:i32) -> i32 {

    x * x
}

fn printname(name :String) {

    println!("Name is {}",name);
} 


fn getsqbyinput () -> i32 {

    use std::io;
    println!("Please enter any number!!");
    let mut _input = String::new();
    
    io::stdin().read_line(&mut _input).expect("Failed to read line"); 

    let _input :i32  = _input.trim().parse().unwrap();
    _input * _input
}

fn comaprenumbers(_input01 :i32,_input02 :i32) -> String {

    if _input01 > _input02 {

       "Input 01 is greater".to_string()
    }
    else if _input02 > _input01 {
        "Input 02 is greater".to_string()
    }
    else{
        "Both numbers are equal".to_string()
    }


}

fn comapareandreturn(_input01 :i32,_input02 :i32) -> i32 {

    if _input01 > _input02 {

       _input01
    }
    else if _input02 > _input01 {
        _input02
    }
    else{
        -1
    }

}

fn take_full_name() -> (String,String){

    use std::io;
    println!("Please enter first name!!");
    let mut _input01 = String::new();  
    io::stdin().read_line(&mut _input01).expect("Failed to read line"); 
    

    println!("Please enter last name!!");
    let mut _input02 = String::new();  
    io::stdin().read_line(&mut _input02).expect("Failed to read line"); 
    
    (_input01.trim().to_string(),_input02.trim().to_string())

}

fn arthm_operations(x :i32,y :i32) -> (i32,i32,i32,i32){

    (x+y,x-y,x*y,x/y)
}

fn get_sq_cube(_n :i32) -> (i32,i32){

    (_n*_n,_n*_n*_n)
}