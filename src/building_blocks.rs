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
