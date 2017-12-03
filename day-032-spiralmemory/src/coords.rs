#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Coords {
    pub x: i64,
    pub y: i64,
}

impl Coords {
    pub fn new(x: i64, y: i64) -> Self {
        Coords { x, y }
    }
}

pub fn adjacent(coords: &Coords) -> Vec<Coords> {
    let x = coords.x;
    let y = coords.y;

    let c = Coords::new;
    vec![c(x - 1, y + 1), c(x, y + 1), c(x + 1, y + 1),
         c(x - 1, y),                  c(x + 1, y),
         c(x - 1, y - 1), c(x, y - 1), c(x + 1, y - 1)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adjacent_origin() {
        let v = adjacent(&Coords::new(0, 0));
        assert!(v.contains(&Coords::new(0, 1)));
        assert!(v.contains(&Coords::new(0, -1)));
        assert!(v.contains(&Coords::new(1, 0)));
        assert!(v.contains(&Coords::new(1, 1)));
        assert!(v.contains(&Coords::new(1, -1)));
        assert!(v.contains(&Coords::new(-1, -1)));
        assert!(v.contains(&Coords::new(-1, 0)));
        assert!(v.contains(&Coords::new(-1, 1)));
        assert_eq!(v.len(), 8)
    }
}
