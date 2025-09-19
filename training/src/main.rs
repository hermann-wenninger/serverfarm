
fn main() {


let name = "Hermann";
let name2 = "Arnika";
let name3 = "Felix";
let name4 = "Carla";
let mut x = Vec::new();
x.push(name);
x.push(name2);
x.push(name3);
x.push(name4);
println!("{:#?}",&x as &[&str] );
}
