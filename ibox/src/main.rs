fn main() {
   let mut x = Box::new(5);
    *x += 1;
    println!("Wert in Box: {}", x);
}
