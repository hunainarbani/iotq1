fn main() {
    
    // let x = 10;
    // let y = x;

    // println!("{},{}",x,y);

    // let s1 = String::from("Hello") ;
    // let s2 = s1.clone();
    // let s3 = &s1;
    // let s4 = &s1;

    // println!("{}, {}, {}, {}",s1,s2,s3,s4);


    let s = String::from("Hello");
    //println!("{}",s);
    takes_ownership(s.clone());
    println!("{}",s);
    let x = 34;
    takes_copy(x);
    
    }
fn takes_ownership(para : String)
{
    println!("{}",para);
}
fn takes_copy(para : i32)
{
    println!("{}",para);
}