use std::io::{self, Write};

pub fn prompt_input(prompt: &str) -> String{
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    //println!("{}",input);
    input.trim().to_string()
}

fn main() {
    let target_input = String::new();
    //Main Output
    println!("TCP Port Scanner - Simple Rust Network Scanner");
    println!("Please Enter a Target IP or Domain");
    prompt_input(&target_input);
    println!("Please Enter a Port Range to Scan");
    prompt_input(&target_input);
}
