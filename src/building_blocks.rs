use fraction::Fraction;

const ANGLE_FULL_CIRCLE: usize = 360;

// An arc with this angle (60 degrees) is the first arc which angle is a divider of the
// full circle (360 degrees) and has a length greater than the radius of the circle.
// (The angle for which the length of the arc equals the radius is 1 rad ~ 57.3 degrees.)
const ANGLE_STEP_FIRST_CIRCLE: usize = 60;

const ARCS_FIRST_CIRCLE: usize = ANGLE_FULL_CIRCLE / ANGLE_STEP_FIRST_CIRCLE;

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
        return 1;
    }
    let msb_pos = usize::BITS - 1 - n.leading_zeros();
    1 << msb_pos
}

fn calcTotalArcs(circle: usize) -> usize {
    roundDownToPowerOf2(circle) * ARCS_FIRST_CIRCLE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundDownToPowerOf2() {
        assert_eq!(roundDownToPowerOf2(0), 1);
        assert_eq!(roundDownToPowerOf2(1), 1);
        assert_eq!(roundDownToPowerOf2(2), 2);
        assert_eq!(roundDownToPowerOf2(3), 2);
        assert_eq!(roundDownToPowerOf2(4), 4);
        assert_eq!(roundDownToPowerOf2(5), 4);
        assert_eq!(roundDownToPowerOf2(8), 8);
        assert_eq!(roundDownToPowerOf2(15), 8);
        assert_eq!(roundDownToPowerOf2(1025), 1024);
    }

    #[test]
    fn test_calcTotalArcs() {
        assert_eq!(calcTotalArcs(0), 6);
        assert_eq!(calcTotalArcs(1), 6);
        assert_eq!(calcTotalArcs(2), 12);
        assert_eq!(calcTotalArcs(3), 12);
        assert_eq!(calcTotalArcs(4), 24);
        assert_eq!(calcTotalArcs(5), 24);
        assert_eq!(calcTotalArcs(6), 24);
        assert_eq!(calcTotalArcs(7), 24);
        assert_eq!(calcTotalArcs(8), 48);
        assert_eq!(calcTotalArcs(9), 48);
        assert_eq!(calcTotalArcs(10), 48);
    }
}
