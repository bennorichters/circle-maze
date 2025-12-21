use fraction::Fraction;

pub struct CircleCoordinate {
    circle: usize,
    arc_index: usize,
    angle: Fraction,
}

impl CircleCoordinate {
    pub fn new(circle: usize, arc_index: usize, angle: Fraction) -> Self {
        CircleCoordinate {
            circle,
            arc_index,
            angle,
        }
    }
}

fn roundDownToPowerOf2(n: usize) -> usize {
    if n == 0 {
        return 0;
    }
    let msb_pos = usize::BITS - 1 - n.leading_zeros();
    1 << msb_pos
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundDownToPowerOf2() {
        assert_eq!(roundDownToPowerOf2(0), 0);
        assert_eq!(roundDownToPowerOf2(1), 1);
        assert_eq!(roundDownToPowerOf2(2), 2);
        assert_eq!(roundDownToPowerOf2(3), 2);
        assert_eq!(roundDownToPowerOf2(4), 4);
        assert_eq!(roundDownToPowerOf2(5), 4);
        assert_eq!(roundDownToPowerOf2(8), 8);
        assert_eq!(roundDownToPowerOf2(15), 8);
        assert_eq!(roundDownToPowerOf2(1025), 1024);
    }
}
