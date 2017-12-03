pub struct Rotate<T> {
    source: T,
    rotate_by: usize,
    position: usize,
}

impl<'a, T: Copy> Rotate<&'a [T]> {
    pub fn new(source: &'a [T], rotate_by: usize) -> Rotate<&'a [T]> {
        Rotate { source, rotate_by, position: 0 }
    }
}

impl<'a, T: Copy> Iterator for Rotate<&'a [T]> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.position < self.source.len() {
            let index = (self.position + self.rotate_by) % self.source.len();
            self.position += 1;
            Some(self.source[index])
        }
        else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_norotate() {
        let mut rotated : Rotate<&[u32]> = Rotate::new(&[], 0);
        assert_eq!(None, rotated.next());
    }

    #[test]
    fn empty_rotated() {
        let mut rotated : Rotate<&[u32]> = Rotate::new(&[], 1);
        assert_eq!(None, rotated.next());
    }

    #[test]
    fn single_norotate() {
        let mut rotated = Rotate::new(&[10], 0);
        assert_eq!(Some(10), rotated.next());
        assert_eq!(None, rotated.next());
    }

    #[test]
    fn single_1rotate() {
        let mut rotated = Rotate::new(&[10], 1);
        assert_eq!(Some(10), rotated.next());
        assert_eq!(None, rotated.next());
    }

    #[test]
    fn three_2rotate() {
        let mut rotated = Rotate::new(&[3, 5, 8], 2);
        assert_eq!(Some(8), rotated.next());
        assert_eq!(Some(3), rotated.next());
        assert_eq!(Some(5), rotated.next());
        assert_eq!(None, rotated.next());
    }
}
