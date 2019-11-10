use std::io;
fn main() {
    /*********PART-1*************/
    
    //Question-1
    // println!("Press any key to continue..");
    // let mut linput = String::new();
    // io::stdin().read_line(&mut linput).expect("Failed to read line"); ; 
    
    let x = 3;
    match x {
            3 => println!("Number is {}",x), //";" Wrong
            //Missing
            _ => println!("Number is other than 3"),
    }

    //Question-2
    let mut _n = 0;
    loop{
            _n += 1;
            println!("Count {}", _n);
            if _n > 99 
            {
                break;
            }
    }

    //Question-3
    let _q3 = 3;
    loop {
        println!("I love Stranger things");
        if _q3 ==3 {
            println!("I don’t love it anymore!");
            break;
        }
    }

    //Question-4
    
    println!("Press any 1st Number");
    let mut _n1 = String::new();
    io::stdin().read_line(&mut _n1).expect("Failed to read line"); ;     
    let _n1 : i32 = _n1.trim().parse().unwrap();
    println!("Press any 2nd Number");
    let mut _n2 = String::new();
    io::stdin().read_line(&mut _n2).expect("Failed to read line"); ;     
    let _n2 : i32 = _n2.trim().parse().unwrap();

    
    if _n1 == _n2 {
        println!("{} and {} are equal\n",_n1,_n2);
    } else{
        println!("{} and {} are not equal\n",_n1,_n2);
    }

    //Question-5

    let mut _x1 = 5;
    loop {
        _x1 += _x1 -- 3;
        println!("{}", _x1);
        if _x1 % 5 != 0 { break; }
    }   

    //Question-6

    let mut sum=0;
    for i in 0..10 {
        let mut data = String::new();
        println!("Enter integer {}",i+1);
        io::stdin().read_line(&mut data).expect("Failed to read line"); ;
        let mut data : i64 = data.trim().parse().unwrap();
        sum = sum + data;
    }
    println!("The sum is: {}",sum);
    println!("The average is: {}",sum/10);

    //Question-7

    
    let mut _cube = String::new();
    println!("Enter number for Loop for Cube..");
    io::stdin().read_line(&mut _cube).expect("Failed to read line"); ;
    let mut _cube : i32 = _cube.trim().parse().unwrap();
    for i in 0.._cube{
        println!("Number is :{} and cube is :{}",i,(i as i32).pow(3) ); 
    }

    //Question-8

    let names = ["Ahmed", "Mujtaba","Hunain"];
    for name in names.iter() {
    match name {
        &"Ahmed" => println!("{} is elder Brother!",name),
        &"Mujtaba" => println!("{} is younger Brother!",name),
        _ => println!("Hello {}", name),
        }

    }

    //Question-9

    let _array: [i32; 5] = [2,4,8,16,32];
    let mut sum = 0;
    let _arraylen = _array.len();
    println!("Find sum of all elements of array:");
    println!("----------------------------------");
    for n in 0.._arraylen{
        sum += _array[n];
    }
    println!("Sum of all elements stored in the array is : {}", sum);

    //Question-10
    
    let _array: [i32; 8] = [10,20,30,40,50,60,70,80];
    let mut sum : i32 = 0;
    let _arraylen = _array.len();
    println!("Find sum/average of all elements of array:");
    println!("----------------------------------");
    for n in 0.._arraylen{
        sum += _array[n];
    }
    println!("Sum of all elements stored in the array is : {}", sum);

    let avg = sum / _arraylen as i32;

    println!("Average of all elements in the array is : {}",avg);

    //Question-11

    let cost_price = 12;
    let sale_price = 13;
    let mut profit_lost = 0;
    if sale_price>cost_price //calculate profit
    {
        profit_lost = sale_price-cost_price;
        println!("You can booked your profit amount : {}", profit_lost);
    }
    else if (cost_price>sale_price) //calculate loss
    {
        profit_lost = cost_price-sale_price;
        println!("You got a loss of amount : {}", profit_lost);
    }
    else //No Profit No Loss
    {
        println!("You are running in no profit no loss condition.");
    }


    /*********PART-2*************/
    //Question-1
    let _prime = 10;
    let mut _zerocount = 0;
    for n in 1.._prime {
        
        if _prime % n == 0 {
            _zerocount += 1;
        }

    }

    if _zerocount == 1 {
        println!("{} is Prime Number",_prime)
    }
    else {
        println!("{} is not Prime Number",_prime)
    }

    //Question-2

    let mut _y = 1;

    loop{
        if _y <= 3 {
            println!("I love my Mother!!");
        }
        else if _y == 4{
            println!("I love my Father!!");
        }
        else{
            break;
        }
        _y += 1;
    }

    _y = 1;
    while _y <= 4{
        if _y <= 3 {
            println!("I love my Mother!!");
        }
        else if _y == 4{
            println!("I love my Father!!");
        }
        _y += 1;
    }

    for _y in 1..5 {
        if _y <= 3 {
            println!("I love my Mother!!");
        }
        else if _y == 4{
            println!("I love my Father!!");
        }
    }

    //Question-3

    println!("Please enter 10,100 or 1000");
    let mut _num = String::new();
    io::stdin().read_line(&mut _num).expect("Failed to read line");                
    let _num: i32 = _num.trim().parse().unwrap();

    match _num{
        10 => println!("Decade.."),
        100 => println!("Century.."),
        1000 => println!("Millinium.."),    
        _ => println!("Please enter 10,100 or 1000"),
    }

    //Question-4

   
    let mut _nsum = 0;

    for i in 1..11 {
            println!("{}",i);
            _nsum += i;
    }

    println!("Sum of first 10 Natural Number {}",_nsum);

    //Question-5

    println!("Please enter Physics Marks");
    let mut _phy = String::new();
    io::stdin().read_line(&mut _phy).expect("Failed to read line");                
    let _phy: i32 = _phy.trim().parse().unwrap();
    
    println!("Please enter Chemistry Marks");
    let mut _chem = String::new();
    io::stdin().read_line(&mut _chem).expect("Failed to read line");                
    let _chem: i32 = _chem.trim().parse().unwrap();
    
    println!("Please enter Maths Marks");
    let mut _math = String::new();
    io::stdin().read_line(&mut _math).expect("Failed to read line");                
    let _math: i32 = _math.trim().parse().unwrap();

    let _cond1 = _phy + _math + _chem;

    if _cond1 >= 180{
        println!("You are eligible!!");
    } 
    else {
        println!("You are not eligible!!");
    }

}
