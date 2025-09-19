
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
println!("{:?}",x );
let y= vec![1,2,3,4,5,6,7,8,9,10];
println!("{:#?}",&y as &[i32] );
println!("{:?}",&y[1..3] );
}
