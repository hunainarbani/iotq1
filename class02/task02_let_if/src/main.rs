fn main() {
    // let condition = true;
    // let num = if condition {
    //     10
    // }
    // else {
    //     16
    // };

    // println!("{}",num);

    let num = 99;
    let result = if num % 2 == 0 {
        "Even"
    }
    else {
        "Odd"
    };

    println!("Number is {}",result);

}
