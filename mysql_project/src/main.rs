extern crate dblayer;
#[macro_use] extern crate text_io;

fn main() {
    
    loop {
            println!("Enter Select Operation\n1)Select\n2)Add\n3)Edit\n4)Delete\n5)Quit");
            let opt :i8 = read!();
            
            match opt{
                1 => select_students(),
                2 => add_student(),
                3 => update_student(),
                4 => delete_student(),                
                5 => println!("Thank you!!!"),
                _ => println!("Invalid Input")
            }           
            if opt == 5 {
                break;
            } 
        }
    
}

fn print_line(llen :i16){
    for i in 0..llen {
        print!("=");
    }
    print!("\n");
}

fn print_column(llen :i16, ltext : &str, plinebreak :bool){
    
    let mut txt ="|".to_string();
    let txtlen = ltext.len();
    if txtlen < llen as usize {
        txt = txt + ltext;
    }
    else {
        let txtlen = txtlen + 1 - llen as usize;
        txt = txt + &ltext[..txtlen];
    }
    

    
    print!("{}",txt);
    for i in 0..(llen - txt.len() as i16) {
        print!(" ");
    }    

    if plinebreak {
        print!("|\n");
    }
}


fn select_students(){

    let selected_students :Vec<dblayer::db::Students> = dblayer::db::Students::select(0);

    print_line(115);
    print_column(13,"Id",false);
    print_column(37,"Student Name",false);
    print_column(32,"Email",false);
    print_column(13,"Age",false);
    print_column(19,"Marital Status",true);
    print_line(115);
    for x in &selected_students {
        
        
        print_column(13,&x.id.to_string(),false);
        let mut txt = match &x.student_name {
            None => "".to_string(),
            Some(x)=> x.to_string(),
        };
        print_column(37,&txt.to_string(),false);
        txt = match &x.email {
            None => "".to_string(),
            Some(x)=> x.to_string(),
        };                
        print_column(32,&txt.to_string(),false);
        txt = match &x.age {
            None => "".to_string(),
            Some(x)=> x.to_string(),
        };                
        print_column(13,&txt.to_string(),false);
        txt = match &x.marital_status {
            None => "".to_string(),
            Some(x)=> x.to_string(),
        };                
        print_column(19,&txt.to_string(),true);
    }
    print_line(115);
}


fn add_student(){

    println!("Please enter Name");
    let name :String = read!("{}\n");

    println!("Please enter email");
    let email :String =read!("{}\n");

    println!("Please enter age");
    let age :i8 = read!("{}\n");

    println!("Please select martial status.\n1)Single\n2)Married");
    let mut maritalstatus :String = "".to_string();
    let maritalstatusid :i8 = read!("{}\n");
    if maritalstatusid ==  1 {
        maritalstatus = "Single".to_string();
    }
    else if maritalstatusid == 2{
        maritalstatus = "Married".to_string();
    }    
    dblayer::db::Students::new(0,name,email,age,maritalstatus);
}

fn update_student(){

    println!("Please select student Id for Update.");
    select_students();
    let pId :i32 = read!("{}\n");

    let mut students :Vec<dblayer::db::Students> = dblayer::db::Students::select(pId);

    let mut i = 0;
    for mut x in students {
        let id = x.id;
        println!("Please update for Student's Id = {}, leave blank for no change",id);
        
        println!("Please enter Name");
        let mut student_name :String = read!("{}\n");
        let mut txt = match x.student_name {
            None => "".to_string(),
            Some(x)=> x.to_string(),
        };        
        if student_name.trim() == ""{
            student_name = txt;
        }
        
        println!("Please enter email");
        let mut email :String =read!("{}\n");    
        txt = match x.email {
            None => "".to_string(),
            Some(x)=> x.to_string(),
        };
        if email.trim() == ""{
            email = txt;
        }
        println!("Please enter age");
        let mut iage :String = read!("{}\n");
        let age :i8;
        txt = match x.age {
            None => "".to_string(),
            Some(x)=> x.to_string(),
        };
        
        if iage.trim() == "" || !is_number(&iage){
            age = txt.trim().parse().unwrap(); 
        }
        else{                
            age = iage.trim().parse().unwrap(); 
        }
        println!("Please select martial status.\n0)No Change\n1)Single\n2)Married");
        let mut maritalstatus :String = "".to_string();
        let mut  maritalstatusid :i8 = read!("{}\n");
        if maritalstatusid ==  1 {
            maritalstatus = "Single".to_string();
        }
        else if maritalstatusid == 2{
             maritalstatus = "Married".to_string();        
        }    
        
        txt = match x.marital_status {
            None => "".to_string(),
            Some(x)=> x.to_string(),
        };
        if maritalstatus == ""{
            maritalstatus = txt;
        }
  
        let student = dblayer::db::Students::getStruct(id,
                    student_name,
                    email,
                    age,
                    maritalstatus
        );
        dblayer::db::Students::update(student);
        i += 1;
    }

    


}

fn delete_student(){

    println!("Please select student Id for Delete.");
    select_students();
    let pId :i32 = read!("{}\n");

    let mut students :Vec<dblayer::db::Students> = dblayer::db::Students::select(pId);

    let mut i = 0;
    for mut x in students {
        let id = x.id;
        
        println!("Are you sure you want to delete Student's Id = {}?\nPlease write 'Del' to confirm",id);
        let dc :String = read!("{}\n");

        if dc == "Del" {
            println!("Deleting...{}",pId);
            dblayer::db::Students::delete(pId);
        }

    }
}

fn is_number(input :&String) -> bool{
    
    for vchar in input.trim().chars(){
        let ctype = vchar as u32;            

        if !(ctype >= 48 && ctype <= 57){      
            return false
        }
    }
    true
}