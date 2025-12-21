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

pub fn roundDownToPowerOf2(n: usize) -> usize {
    if n == 0 {
        return 0;
    }
    let msb_pos = usize::BITS - 1 - n.leading_zeros();
    1 << msb_pos
}
