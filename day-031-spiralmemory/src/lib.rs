pub fn distance(square: u32) -> u32 {
    let ring = ring(square);
    let midpoints = midpoints(ring);
    let mut best = u32::max_value();
    for m in midpoints {
        let distance_to_midpoint = distance_between(square, m);
        best = best.min(distance_to_midpoint);
    }

    best + ring
}

fn distance_between(a: u32, b: u32) -> u32 {
    let difference = i64::from(a) - i64::from(b);
    difference.abs() as u32
} 

fn ring(square: u32) -> u32 {
    // highest value in ring 0 is 1 (1^2)
    // highest value in ring 1 is 9 (3^2)
    // highest value in ring 2 is 25 (5^2)
    // highest value in ring n is (1+2n)^2

    for ring in 0.. {
        if square <= end_of_ring(ring) {
            return ring;
        }
    }

    panic!("Out of bounds: square {}", square);
}

fn end_of_ring(ring: u32) -> u32 {
    ((1 + 2 * ring) as u32).pow(2)
}

fn start_of_ring(ring: u32) -> u32 {
    match ring {
        0 => 1,
        n => end_of_ring(n - 1) + 1,
    }
}

fn midpoints(ring: u32) -> Vec<u32> {
    if ring == 0 {
        vec![1, 1, 1, 1]
    }
    else {
        let ring_size = start_of_ring(ring + 1) - start_of_ring(ring);
        let first = start_of_ring(ring) + (ring - 1);
        let interval = ring_size / 4;
        vec![first, first + interval, first + interval * 2, first + interval * 3]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ring_of_1() {
        assert_eq!(ring(1), 0);
    }

    #[test]
    fn ring_of_2() {
        assert_eq!(ring(2), 1);
    }

    #[test]
    fn ring_of_9() {
        assert_eq!(ring(9), 1);
    }

    #[test]
    fn ring_of_10() {
        assert_eq!(ring(10), 2);
    }

    #[test]
    fn ring_of_18() {
        assert_eq!(ring(18), 2);
    }

    #[test]
    fn distance_of_1() {
        assert_eq!(distance(1), 0);
    }

    #[test]
    fn distance_of_12() {
        assert_eq!(distance(12), 3);
    }

    #[test]
    fn distance_of_1024() {
        assert_eq!(distance(1024), 31);
    }
}
