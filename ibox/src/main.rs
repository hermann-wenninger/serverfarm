fn main() {
   let mut x = Box::new(5);
    *x += 1;
   let y = &x;
    
    println!("Wert in Box: {}", &x);
    println!("Referenz auf Box: {}", **y);
}
