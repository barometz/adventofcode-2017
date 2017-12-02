pub fn inverse_captcha(input: &str) -> u32
{
    let mut sum = 0;
    
    if let Some(mut previous) = input.chars().last()
    {
        for current in input.chars() {
            if previous == current {
                let converted : u32 = current.to_string().parse().unwrap();
                sum += converted;
            }

            previous = current;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_1122() {
        assert_eq!(inverse_captcha("1122"), 3);
    }

    #[test]
    fn sample_1111() {
        assert_eq!(inverse_captcha("1111"), 4);
    }
    
    #[test]
    fn sample_1234() {
        assert_eq!(inverse_captcha("1234"), 0);
    }

    #[test]
    fn sample_91212129() {
        assert_eq!(inverse_captcha("91212129"), 9);
    }
}
