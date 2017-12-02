extern crate day_01_inversecaptcha;

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let result = day_01_inversecaptcha::inverse_captcha(&input.trim());
    println!("Result: {}", result);
}
