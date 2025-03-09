use std::io::stdin;
fn main() {
    let mut msg = String::new();
    println!("Please enter message:");
    stdin().read_line(&mut msg).unwrap();
    println!("message is {}", msg);
}
