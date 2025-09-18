fn time_two(number:i32)-> i32{
    number + number
}


fn main() {
    let arg = 5;
    let answer = time_two(arg);
    println!("The answer is: {}", answer);
}
