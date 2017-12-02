mod rotate;

use rotate::Rotate;

pub fn inverse_captcha(input: &str) -> u32 {
    let mut sum: u32 = 0;
    let v : Vec<char> = input.chars().collect();
    let rotated = Rotate::new(&v, input.len() / 2);
    for (a, &b) in rotated.zip(&v) {
        if a == b {
            let x: u32 = a.to_string().parse().unwrap();
            sum += x;
        }
    }

    sum
}
