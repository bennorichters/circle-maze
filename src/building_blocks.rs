use fraction::Fraction;

const ANGLE_FULL_CIRCLE: usize = 360;

// An arc with this angle (60 degrees) is the first arc which angle is a divider of the
// full circle (360 degrees) and has a length greater than the radius of the circle.
// (The angle for which the length of the arc equals the radius is 1 rad ~ 57.3 degrees.)
const ANGULAR_DIVISOR_FIRST_CIRCLE: usize = 60;

const ARCS_FIRST_CIRCLE: usize = ANGLE_FULL_CIRCLE / ANGULAR_DIVISOR_FIRST_CIRCLE;

#[derive(Debug)]
pub struct CircleCoordinate {
    circle: usize,
    arc_index: usize,
    angle: Fraction,
}

impl CircleCoordinate {
    pub fn new(circle: usize, arc_index: usize, angle: Fraction) -> Result<Self, String> {
        if arc_index > (calc_total_arcs(circle) - 1) {
            Err(format!(
                "arcIndex too big for {}, arcIndex: {}",
                circle, arc_index
            ))
        } else {
            Ok(CircleCoordinate {
                circle,
                arc_index,
                angle,
            })
        }
    }

    pub fn create_with_arc_index(circle: usize, arc_index: usize) -> Result<Self, String> {
        let angle = calc_angle_step(circle) * Fraction::from(arc_index);
        Self::new(circle, arc_index, angle)
    }

    pub fn create_with_fraction(circle: usize, angle: Fraction) -> Result<Self, String> {
        let step = calc_angle_step(circle);
        let arc_index_fraction = angle / step;

        if *arc_index_fraction.denom().unwrap() != 1 {
            return Err(format!(
                "no such angle for circle. {}, angle: {}",
                circle, angle
            ));
        }

        let arc_index = *arc_index_fraction.numer().unwrap() as usize;
        Self::new(circle, arc_index, angle)
    }

    pub fn next_clockwise(&self) -> Result<Self, String> {
        let next_arc_index = (self.arc_index + 1) % calc_total_arcs(self.circle);
        Self::create_with_arc_index(self.circle, next_arc_index)
    }
}

pub fn calc_total_arcs(circle: usize) -> usize {
    round_down_to_power_of2(circle) * ARCS_FIRST_CIRCLE
}

fn round_down_to_power_of2(n: usize) -> usize {
    if n == 0 {
        return 1;
    }
    let msb_pos = usize::BITS - 1 - n.leading_zeros();
    1 << msb_pos
}

fn calc_angle_step(circle: usize) -> Fraction {
    Fraction::from(ANGLE_FULL_CIRCLE) / Fraction::from(calc_total_arcs(circle))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_down_to_power_of2() {
        assert_eq!(round_down_to_power_of2(0), 1);
        assert_eq!(round_down_to_power_of2(1), 1);
        assert_eq!(round_down_to_power_of2(2), 2);
        assert_eq!(round_down_to_power_of2(3), 2);
        assert_eq!(round_down_to_power_of2(4), 4);
        assert_eq!(round_down_to_power_of2(5), 4);
        assert_eq!(round_down_to_power_of2(8), 8);
        assert_eq!(round_down_to_power_of2(15), 8);
        assert_eq!(round_down_to_power_of2(1025), 1024);
    }

    #[test]
    fn test_calc_total_arcs() {
        assert_eq!(calc_total_arcs(0), 6);
        assert_eq!(calc_total_arcs(1), 6);
        assert_eq!(calc_total_arcs(2), 12);
        assert_eq!(calc_total_arcs(3), 12);
        assert_eq!(calc_total_arcs(4), 24);
        assert_eq!(calc_total_arcs(5), 24);
        assert_eq!(calc_total_arcs(6), 24);
        assert_eq!(calc_total_arcs(7), 24);
        assert_eq!(calc_total_arcs(8), 48);
        assert_eq!(calc_total_arcs(9), 48);
        assert_eq!(calc_total_arcs(10), 48);
    }

    #[test]
    fn test_create_with_arc_index() {
        let coord0 = CircleCoordinate::create_with_arc_index(0, 0).unwrap();
        assert_eq!(coord0.angle, Fraction::from(0));

        let coord1 = CircleCoordinate::create_with_arc_index(0, 1).unwrap();
        assert_eq!(coord1.angle, Fraction::from(60));

        let coord2 = CircleCoordinate::create_with_arc_index(0, 2).unwrap();
        assert_eq!(coord2.angle, Fraction::from(120));

        let coord3 = CircleCoordinate::create_with_arc_index(0, 3).unwrap();
        assert_eq!(coord3.angle, Fraction::from(180));

        let coord4 = CircleCoordinate::create_with_arc_index(0, 4).unwrap();
        assert_eq!(coord4.angle, Fraction::from(240));

        let coord5 = CircleCoordinate::create_with_arc_index(0, 5).unwrap();
        assert_eq!(coord5.angle, Fraction::from(300));

        let coord0_c1 = CircleCoordinate::create_with_arc_index(1, 0).unwrap();
        assert_eq!(coord0_c1.angle, Fraction::from(0));

        let coord1_c1 = CircleCoordinate::create_with_arc_index(1, 1).unwrap();
        assert_eq!(coord1_c1.angle, Fraction::from(60));

        let coord2_c1 = CircleCoordinate::create_with_arc_index(1, 2).unwrap();
        assert_eq!(coord2_c1.angle, Fraction::from(120));

        let coord3_c1 = CircleCoordinate::create_with_arc_index(1, 3).unwrap();
        assert_eq!(coord3_c1.angle, Fraction::from(180));

        let coord4_c1 = CircleCoordinate::create_with_arc_index(1, 4).unwrap();
        assert_eq!(coord4_c1.angle, Fraction::from(240));

        let coord5_c1 = CircleCoordinate::create_with_arc_index(1, 5).unwrap();
        assert_eq!(coord5_c1.angle, Fraction::from(300));

        let coord1_c10 = CircleCoordinate::create_with_arc_index(10, 1).unwrap();
        assert_eq!(coord1_c10.angle, Fraction::new(360u64, 48u64));
    }

    #[test]
    fn test_create_with_fraction() {
        let coord0 = CircleCoordinate::create_with_fraction(0, Fraction::from(0)).unwrap();
        assert_eq!(coord0.arc_index, 0);

        let coord1 = CircleCoordinate::create_with_fraction(0, Fraction::from(60)).unwrap();
        assert_eq!(coord1.arc_index, 1);

        let coord2 = CircleCoordinate::create_with_fraction(0, Fraction::from(120)).unwrap();
        assert_eq!(coord2.arc_index, 2);

        let coord3 = CircleCoordinate::create_with_fraction(0, Fraction::from(180)).unwrap();
        assert_eq!(coord3.arc_index, 3);

        let coord4 = CircleCoordinate::create_with_fraction(0, Fraction::from(240)).unwrap();
        assert_eq!(coord4.arc_index, 4);

        let coord5 = CircleCoordinate::create_with_fraction(0, Fraction::from(300)).unwrap();
        assert_eq!(coord5.arc_index, 5);
    }
}
