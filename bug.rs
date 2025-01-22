fn main() {
    let mut x = 5;
    let y = &mut x; 
    let z = &mut x; //this will give a compile time error because we are trying to borrow x mutably twice
    *y = 10;
    *z = 100;
    println!("x = {}", x);
}