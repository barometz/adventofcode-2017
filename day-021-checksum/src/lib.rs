pub fn checksum(input: &str) -> u32 {
    let converted = convert_table(input);
    converted.into_iter().fold(0, |acc, v| acc + rowdiff(&v))
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

fn rowdiff(row: &[u32]) -> u32 {
    let iter = row.into_iter();
    let highest = iter.clone().max().unwrap_or(&0);
    let lowest = iter.clone().min().unwrap_or(&0);
    highest - lowest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rowdiff_1() {
        assert_eq!(rowdiff(&[5, 1, 9, 5]), 8)
    }

    #[test]
    fn rowdiff_2() {
        assert_eq!(rowdiff(&[7, 5, 3]), 4)
    }

    #[test]
    fn rowdiff_eq() {
        assert_eq!(rowdiff(&[1, 1, 1, 1]), 0)
    }

    #[test]
    fn checksum_sample() {
        let input = "5 1 9 5
         7 5 3
         2 4 6 8";
        assert_eq!(checksum(input), 18)
    }
}
