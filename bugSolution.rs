fn main() {
    let mut x = 5;
    {  //using a block to limit the scope of the mutable borrow
        let y = &mut x; 
        *y = 10;
        println!("x = {}", x);
    }
    { //another block for the second borrow 
        let z = &mut x;
        *z = 100;
        println!("x = {}", x);
    }
    
} 