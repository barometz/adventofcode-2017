use std::cmp;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Position {
    pub ring: u32,
    pub offset: u32,
}

impl Position {
    pub fn new(ring: u32, offset: u32) -> Self {
        Position { ring, offset }
    }

    pub fn from_square(square: u32) -> Self {
        let ring = ring(square);
        let offset = square - start_of_ring(ring);

        Position { ring, offset }
    }
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

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        match self.ring.cmp(&other.ring) {
            cmp::Ordering::Equal => Some(self.offset.cmp(&other.offset)),
            x => Some(x)
        }
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
