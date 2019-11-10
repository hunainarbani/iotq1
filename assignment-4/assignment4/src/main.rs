use std::io;

fn main(){
    let mut int = String::new();
    io::stdin().read_line(&mut int)
        .expect("Failed to read your input");

    let int: u8 = match int.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };
  
// Enter your code here. 

    //println!("{:?}",);

    for i in &reverse_vector(read_vector_elements(),int) {
        print!("{} ", i);
    }
}

fn reverse_vector(_vect :Vec<u8>,no_element :u8)->Vec<u8>{

 // Enter your code here.
    
    let mut r_array: Vec<u8> = Vec::new();
    
    for i in (0..no_element).rev() {       
       let _x :u8 = _vect[i as usize];
       r_array.push(_x);

    }


    r_array
}

fn read_vector_elements() -> Vec<u8> {
    // Enter your code here.
    
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("input");
    let v_array :Vec<u8> = line.trim().split(' ').flat_map(str::parse::<u8>).collect::<Vec<_>>();
    
    v_array
    
}