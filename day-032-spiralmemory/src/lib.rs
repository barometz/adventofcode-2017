mod coords;
mod position;

use coords::Coords;
use position::Position;
use std::cmp;
use std::collections::btree_map;

pub fn adjacent_sum(square: u32) -> u32 {
    let position = Position::from_square(square);
    let mut cache = btree_map::BTreeMap::new();
    adjacent_sum_position(position, &mut cache)
}

fn adjacent_sum_position(position: Position, mut cache: &mut btree_map::BTreeMap<Position, u32>) -> u32 {
    let cached = cache.get(&position).map(|&x| x);
    
    match cached {
        Some(x) => x,
        None => {
            let sum = match position.ring {
                0 => 1,
                _ => coords::adjacent(&Coords::from(position))
                    .into_iter()
                    .filter(|c| Position::from(*c) < position)
                    .fold(0, |acc, c| acc + adjacent_sum_position(Position::from(c), &mut cache)),
            };
            cache.insert(position, sum);
            sum
        }
    }
}

struct RingCoords {
    ring: u32,
    index: u32,
    coords: Coords,
}

impl RingCoords {
    fn new(ring: u32) -> Self {
        let x = i64::from(ring);
        let y = match ring {
            0 => 0,
            n => 0 - (i64::from(n) - 1),
        };
        RingCoords {
            ring,
            index: 0,
            coords: Coords { x, y },
        }
    }
}

impl Iterator for RingCoords {
    type Item = Coords;

    fn next(&mut self) -> Option<Self::Item> {
        let end = ring_size(self.ring);
        let result: Option<Coords>;
        if self.index >= end {
            result = None;
        } else {
            result = Some(self.coords);
            self.index += 1;
            let ring = i64::from(self.ring);
            if self.coords.x == ring && self.coords.y < ring {
                self.coords.y += 1;
            } else if self.coords.y == ring && self.coords.x > -ring {
                self.coords.x -= 1;
            } else if self.coords.x == -ring && self.coords.y > -ring {
                self.coords.y -= 1;
            } else if self.coords.y == -ring && self.coords.x < ring {
                self.coords.x += 1;
            }
        }

        result
    }
}

impl From<Position> for Coords {
    fn from(position: Position) -> Self {
        RingCoords::new(position.ring)
            .nth(position.offset as usize)
            .unwrap()
    }
}

impl From<Coords> for Position {
    fn from(coords: Coords) -> Self {
        let ring = cmp::max(coords.x.abs(), coords.y.abs()) as u32;
        for (i, c) in RingCoords::new(ring).enumerate() {
            if coords == c {
                return Position {
                    ring,
                    offset: i as u32,
                };
            }
        }

        panic!("internal screwup");
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

fn ring_size(ring: u32) -> u32 {
    start_of_ring(ring + 1) - start_of_ring(ring)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ring_coords_ring_2() {
        let mut ci = RingCoords::new(1);
        assert_eq!(Some(Coords::new(1, 0)), ci.next())
    }

    #[test]
    fn position_from_center() {
        let p = Position::from(Coords::new(0, 0));
        assert_eq!(p, Position::new(0, 0));
    }

    #[test]
    fn position_from_1_0() {
        let p = Position::from(Coords::new(1, 0));
        assert_eq!(p, Position::new(1, 0));
    }

    #[test]
    fn position_from_0_1() {
        let p = Position::from(Coords::new(0, 1));
        assert_eq!(p, Position::new(1, 2));
    }

    #[test]
    fn sample_square_1() {
        assert_eq!(adjacent_sum(1), 1);
    }

    #[test]
    fn sample_square_3() {
        assert_eq!(adjacent_sum(3), 2);
    }

    #[test]
    fn sample_square_5() {
        assert_eq!(adjacent_sum(5), 5);
    }
}
