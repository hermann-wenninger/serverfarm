use std::ops::Deref;

struct MioBox<T>(T);

impl <T> MioBox<T> {
    fn new(x: T) -> MioBox<T> {
        MioBox(x)
    }
}
impl<T> Deref for MioBox<T>{
    type Target =T;
    fn deref(&self)->&Self::Target{
        &self.0
    }
}
impl<T: std::fmt::Display> MioBox<T> {
    fn print(&self) {
        println!("MioBox enthält: {}", self.0);
    }
}
impl<T> Drop for MioBox<T> where T: std::fmt::Debug {
    fn drop(&mut self){
        println!("MioBox wird gelöscht {:?}",&self.0);
    }
}
fn main() {
   let mut x = Box::new(5);
    *x += 1;
   let y = &x;
    
    println!("Wert in Box: {}", &x);
    println!("Referenz auf Box: {}", **y);

    let mio = MioBox::new(23);
    println!("Wert in MioBox: {}", mio.0);
    println!("Wert in MioBox via Deref: {}", *mio+1);
    drop(mio);
}
