fn main() {
 let chocolate1 = 10;
 let chocolate2 = 10;
 //const total: u32 = chocolate1 + chocolate2; ^^^^^^^^^^ non-constant value
 let total = chocolate1 + chocolate2;
 //println!("The sum of x and y is: ",total); {} was missing
 println!("The sum of chocolate1 and chocolate2 is:{} ",total);
}
