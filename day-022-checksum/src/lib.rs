pub fn checksum(input: &str) -> u32 {
    let converted = convert_table(input);
    converted.into_iter().fold(0, |acc, v| acc + rowdiv(&v))
}

fn convert_table(input: &str) -> Vec<Vec<u32>> {
    input.lines().map(|row| convert_row(row)).collect()
}

fn convert_row(input: &str) -> Vec<u32> {
    input
        .split_whitespace()
        .map(|x| x.to_string().parse().unwrap())
        .collect()
}

fn rowdiv(row: &[u32]) -> u32 {
    let mut div: u32 = 0;

    'outer: for &m in row {
        for &n in row {
            if m % n == 0 && m != n {
                div = m / n;
                break 'outer;
            }
        }
    }

    div
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample_1() {
        assert_eq!(rowdiv(&[5, 9, 2, 8]), 4);
    }

    #[test]
    fn sample_2() {
        assert_eq!(rowdiv(&[9, 4, 7, 3]), 3);
    }
}