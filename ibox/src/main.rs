use std::ops::{Deref, DerefMut};
use std::fmt::Debug;

#[derive(Debug)]
struct MioBox<T: Debug>(T);

impl<T: Debug> MioBox<T> {
    fn new(x: T) -> Self {
        MioBox(x)
    }
}

impl<T: Debug> Deref for MioBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Debug> DerefMut for MioBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Debug> Drop for MioBox<T> {
    fn drop(&mut self) {
        println!("MioBox wird gelöscht: {:?}", self.0);
    }
}

fn main() {
    let a = MioBox::new(42);
    let b = MioBox::new(String::from("Hallo"));

    println!("a + 1 = {}", *a + 1);
    println!("b Länge = {}", b.len());
}