fn main() {
    let mut x = 5; 
    { // New scope to limit the mutable borrow of y
        let y = &mut x; 
        *y += 1;
    }
    { // New scope to limit the mutable borrow of z
        let z = &mut x; 
        *z += 1;
    }
    println!("x = {}", x); 
} 