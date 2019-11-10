#[macro_use] extern crate text_io;
pub mod numbers{

    pub fn check_number_sign(){

        println!("Please enter any number!!");
        let x :i32 = read!();

        if x > 0 {
            println!("Postive number entered!!");
        }
        else if x < 0 {
            println!("Negative number entered!!");
        }
        else {
            println!("Zero Entered!!");
        }
        
    }

    pub fn check_divisble(){

        println!("Enter numerator!!");
        let x :i32 = read!();

        println!("Enter Denominator!!");
        let y :i32 = read!();

        if x % y == 0{
            println!("Number {} is Completely divisible by {}",x,y);
        }
        else {
            println!("Number {} is not Completely divisible by {}",x,y);
        }
    }

    pub fn check_even_odd(){

        println!("Enter Number!!");
        let num :i32 = read!();

        if num % 2 == 0 {
            println!("{} is Even.",num);
        }
        else {
            println!("{} is Odd.",num);
        }
    }    
}

pub mod calculations{

    pub fn calc_interest(){
        println!("Please enter principal amount");
        let pamount :f32 = read!();
        println!("Please Enter Rate of interest in %");
        let rate :f32 = read!();
        println!("Enter number of years for investment");
        let yr :i32 = read!();
        let mut invamount :f32 = pamount;
        for ll in 0..yr {
            invamount += rate*invamount;
        }

        println!("After {} years your principal amount {} over an interest rate of {}% will be {}",yr,pamount,rate,invamount);    
    }

    pub fn eucl_dist(){

        println!("Enter Coordinate for x1");
        let x1 :f32 = read!();
        println!("Enter Coordinate for x2");
        let x2 :f32 = read!();
        println!("Enter Coordinate for y1");
        let y1 :f32 = read!();
        println!("Enter Coordinate for y2");
        let y2 :f32 = read!();

        let a :f32 = (x2 - x1) * (x2 - x1);
        let b :f32 = (y2 - y1) * (y2 - y1);
        let c :f32 = a + b;
        let d :f32 = (c).sqrt();

        println!("Distance between points ({}, {}) and ({}, {}) is {}",x1,x2,y1,y2,d);
    }

    pub fn feet_to_cm(){

        println!("Enter Height in Feet");
        let ft :f32 = read!();

        let cm :f32 = ft * 30.48;
        println!("There are {} Cm in {} ft",cm,ft);

    }

    pub fn calc_bmi(){

        println!("Enter Height in Cm");
        let h :f32 = read!();
        let h = h/100.0 * h/100.0;

        println!("Enter Weight in Kg");
        let wt :f32 = read!();  

        let bmi = wt/h;

        println!("Your BMI is {}",bmi) ;

    }

    pub fn sum_pos_int(){

        println!("Enter value of n");
        let n :i32 = read!();
        let mut sum_n = 0;
        for x in 1..=n{            
            sum_n += x;
        }

        println!("Sum of n Positive integers till {} is {}",n,sum_n);
    }

    pub fn sum_digits(){

        println!("Enter a number");
        let mut num :String = read!();
        let mut sum :i32 = 0;
        let mut sstr :String = "".to_string();
        let mut digit :i32 = 0;
        for n in num.chars() {
            digit = n.to_string().trim().parse().unwrap();            
            sum = sum + digit;
            sstr = sstr + &n.to_string() + &"+".to_string();
        }
        
        println!("Sum of {} is {}",&sstr[0..sstr.len()-1],sum);

    }

    pub fn decimal_to_binary(){
        
        println!("Enter a decimal number");
        let num :i32 = read!();

        let mut qot :i32 = 0;
        let mut rem :i32 = 0;
        let mut bin :String = "".to_string();
        rem = num % 2;
        qot = (num  - rem)/ 2;
        println!("{}",qot);
        
        bin = rem.to_string() + &bin;
        loop {
            rem = qot % 2;
            qot = (qot - rem)/ 2;            
            bin = rem.to_string() + &bin;
            if qot == 0{
                break;
            }
        }

        println!("Binary Representation of {} is {}",num,bin);

    }

        pub fn binary_to_decimal(){
            
            println!("Enter a binary number");
            let num :String = read!();
            let mut len :i32 = num.len() as i32;
            let mut digit :i32 = 0;
            let mut dec :i32 =0;
            len = len -1;
            for n in num.chars(){
                digit = n.to_string().trim().parse().unwrap();                                
                dec += digit * num_power(2,len);                            
                len -= 1;
            }

            println!("Decimal Representation of {} is {}",num,dec);
        
        }

        fn num_power(num :i32, pwr :i32) -> i32{
            
            let mut result :i32 = 1;
            for x in 0..pwr{
                result *= num;
            }

            result

        }
}

