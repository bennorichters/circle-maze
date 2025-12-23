use fraction::Fraction;

const ANGLE_FULL_CIRCLE: usize = 360;

// An arc with this angle (60 degrees) is the first arc which angle is a divider of the
// full circle (360 degrees) and has a length greater than the radius of the circle.
// (The angle for which the length of the arc equals the radius is 1 rad ~ 57.3 degrees.)
const ANGULAR_DIVISOR_FIRST_CIRCLE: usize = 60;

const ARCS_FIRST_CIRCLE: usize = ANGLE_FULL_CIRCLE / ANGULAR_DIVISOR_FIRST_CIRCLE;

#[derive(Debug, Clone)]
pub struct CircleCoordinate {
    circle: usize,
    arc_index: usize,
    angle: Fraction,
}

impl PartialEq for CircleCoordinate {
    fn eq(&self, other: &Self) -> bool {
        self.circle == other.circle && self.arc_index == other.arc_index
    }
}

impl Eq for CircleCoordinate {}

impl CircleCoordinate {
    pub fn new(circle: usize, arc_index: usize, angle: Fraction) -> Self {
        CircleCoordinate {
            circle,
            arc_index,
            angle,
        }
    }

    pub fn create_with_arc_index(circle: usize, arc_index: usize) -> Self {
        let angle = calc_angle_step(circle) * Fraction::from(arc_index);
        Self::new(circle, arc_index, angle)
    }

    pub fn create_with_fraction(circle: usize, angle: Fraction) -> Self {
        let step = calc_angle_step(circle);
        let arc_index_fraction = angle / step;
        let arc_index = *arc_index_fraction.numer().unwrap() as usize;
        Self::new(circle, arc_index, angle)
    }

    pub fn next_clockwise(&self) -> Self {
        let next_arc_index = (self.arc_index + 1) % calc_total_arcs(self.circle);
        Self::create_with_arc_index(self.circle, next_arc_index)
    }

    pub fn next_out(&self) -> Self {
        Self::create_with_fraction(self.circle + 1, self.angle.clone())
    }

    pub fn next_counter_clockwise(&self) -> Self {
        let total_arcs = calc_total_arcs(self.circle);
        let next_arc_index = (self.arc_index + total_arcs - 1) % total_arcs;
        Self::create_with_arc_index(self.circle, next_arc_index)
    }

    pub fn next_in(&self) -> Result<Self, String> {
        if self.circle == 0 {
            return Err("Cannot move inward from circle 0".to_string());
        }
        Ok(Self::create_with_fraction(self.circle - 1, self.angle.clone()))
    }

    pub fn angle(&self) -> &Fraction {
        &self.angle
    }

    pub fn circle(&self) -> usize {
        self.circle
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
        let coord0 = CircleCoordinate::create_with_arc_index(0, 0);
        assert_eq!(coord0.angle, Fraction::from(0));

        let coord1 = CircleCoordinate::create_with_arc_index(0, 1);
        assert_eq!(coord1.angle, Fraction::from(60));

        let coord2 = CircleCoordinate::create_with_arc_index(0, 2);
        assert_eq!(coord2.angle, Fraction::from(120));

        let coord3 = CircleCoordinate::create_with_arc_index(0, 3);
        assert_eq!(coord3.angle, Fraction::from(180));

        let coord4 = CircleCoordinate::create_with_arc_index(0, 4);
        assert_eq!(coord4.angle, Fraction::from(240));

        let coord5 = CircleCoordinate::create_with_arc_index(0, 5);
        assert_eq!(coord5.angle, Fraction::from(300));

        let coord0_c1 = CircleCoordinate::create_with_arc_index(1, 0);
        assert_eq!(coord0_c1.angle, Fraction::from(0));

        let coord1_c1 = CircleCoordinate::create_with_arc_index(1, 1);
        assert_eq!(coord1_c1.angle, Fraction::from(60));

        let coord2_c1 = CircleCoordinate::create_with_arc_index(1, 2);
        assert_eq!(coord2_c1.angle, Fraction::from(120));

        let coord3_c1 = CircleCoordinate::create_with_arc_index(1, 3);
        assert_eq!(coord3_c1.angle, Fraction::from(180));

        let coord4_c1 = CircleCoordinate::create_with_arc_index(1, 4);
        assert_eq!(coord4_c1.angle, Fraction::from(240));

        let coord5_c1 = CircleCoordinate::create_with_arc_index(1, 5);
        assert_eq!(coord5_c1.angle, Fraction::from(300));

        let coord1_c10 = CircleCoordinate::create_with_arc_index(10, 1);
        assert_eq!(coord1_c10.angle, Fraction::new(360u64, 48u64));
    }

    #[test]
    fn test_create_with_fraction() {
        let coord0 = CircleCoordinate::create_with_fraction(0, Fraction::from(0));
        assert_eq!(coord0.arc_index, 0);

        let coord1 = CircleCoordinate::create_with_fraction(0, Fraction::from(60));
        assert_eq!(coord1.arc_index, 1);

        let coord2 = CircleCoordinate::create_with_fraction(0, Fraction::from(120));
        assert_eq!(coord2.arc_index, 2);

        let coord3 = CircleCoordinate::create_with_fraction(0, Fraction::from(180));
        assert_eq!(coord3.arc_index, 3);

        let coord4 = CircleCoordinate::create_with_fraction(0, Fraction::from(240));
        assert_eq!(coord4.arc_index, 4);

        let coord5 = CircleCoordinate::create_with_fraction(0, Fraction::from(300));
        assert_eq!(coord5.arc_index, 5);
    }

    #[test]
    fn test_next_clockwise() {
        let coord0 = CircleCoordinate::create_with_arc_index(0, 0);
        let next = coord0.next_clockwise();
        assert_eq!(next.circle, 0);
        assert_eq!(next.arc_index, 1);
        assert_eq!(next.angle, Fraction::from(60));

        let coord1 = CircleCoordinate::create_with_arc_index(0, 1);
        let next = coord1.next_clockwise();
        assert_eq!(next.circle, 0);
        assert_eq!(next.arc_index, 2);
        assert_eq!(next.angle, Fraction::from(120));

        let coord5 = CircleCoordinate::create_with_arc_index(0, 5);
        let next = coord5.next_clockwise();
        assert_eq!(next.circle, 0);
        assert_eq!(next.arc_index, 0);
        assert_eq!(next.angle, Fraction::from(0));

        let coord2_0 = CircleCoordinate::create_with_arc_index(2, 0);
        let next = coord2_0.next_clockwise();
        assert_eq!(next.circle, 2);
        assert_eq!(next.arc_index, 1);
        assert_eq!(next.angle, Fraction::from(30));

        let coord2_11 = CircleCoordinate::create_with_arc_index(2, 11);
        let next = coord2_11.next_clockwise();
        assert_eq!(next.circle, 2);
        assert_eq!(next.arc_index, 0);
        assert_eq!(next.angle, Fraction::from(0));

        let coord4_23 = CircleCoordinate::create_with_arc_index(4, 23);
        let next = coord4_23.next_clockwise();
        assert_eq!(next.circle, 4);
        assert_eq!(next.arc_index, 0);
        assert_eq!(next.angle, Fraction::from(0));
    }

    #[test]
    fn test_next_out() {
        let coord0_0 = CircleCoordinate::create_with_arc_index(0, 0);
        let next = coord0_0.next_out();
        assert_eq!(next.circle, 1);
        assert_eq!(next.arc_index, 0);
        assert_eq!(next.angle, Fraction::from(0));

        let coord0_1 = CircleCoordinate::create_with_arc_index(0, 1);
        let next = coord0_1.next_out();
        assert_eq!(next.circle, 1);
        assert_eq!(next.angle, Fraction::from(60));
        assert_eq!(next.arc_index, 1);

        let coord1_1 = CircleCoordinate::create_with_arc_index(1, 1);
        let next = coord1_1.next_out();
        assert_eq!(next.circle, 2);
        assert_eq!(next.angle, Fraction::from(60));
        assert_eq!(next.arc_index, 2);

        let coord2_4 = CircleCoordinate::create_with_arc_index(2, 4);
        let next = coord2_4.next_out();
        assert_eq!(next.circle, 3);
        assert_eq!(next.angle, Fraction::from(120));
        assert_eq!(next.arc_index, 4);

        let coord3_3 = CircleCoordinate::create_with_arc_index(3, 3);
        let next = coord3_3.next_out();
        assert_eq!(next.circle, 4);
        assert_eq!(next.angle, Fraction::from(90));
        assert_eq!(next.arc_index, 6);
    }

    #[test]
    fn test_next_counter_clockwise() {
        let coord1 = CircleCoordinate::create_with_arc_index(0, 1);
        let prev = coord1.next_counter_clockwise();
        assert_eq!(prev.circle, 0);
        assert_eq!(prev.arc_index, 0);
        assert_eq!(prev.angle, Fraction::from(0));

        let coord2 = CircleCoordinate::create_with_arc_index(0, 2);
        let prev = coord2.next_counter_clockwise();
        assert_eq!(prev.circle, 0);
        assert_eq!(prev.arc_index, 1);
        assert_eq!(prev.angle, Fraction::from(60));

        let coord0 = CircleCoordinate::create_with_arc_index(0, 0);
        let prev = coord0.next_counter_clockwise();
        assert_eq!(prev.circle, 0);
        assert_eq!(prev.arc_index, 5);
        assert_eq!(prev.angle, Fraction::from(300));

        let coord2_0 = CircleCoordinate::create_with_arc_index(2, 0);
        let prev = coord2_0.next_counter_clockwise();
        assert_eq!(prev.circle, 2);
        assert_eq!(prev.arc_index, 11);
        assert_eq!(prev.angle, Fraction::from(330));

        let coord2_5 = CircleCoordinate::create_with_arc_index(2, 5);
        let prev = coord2_5.next_counter_clockwise();
        assert_eq!(prev.circle, 2);
        assert_eq!(prev.arc_index, 4);
        assert_eq!(prev.angle, Fraction::from(120));

        let coord4_0 = CircleCoordinate::create_with_arc_index(4, 0);
        let prev = coord4_0.next_counter_clockwise();
        assert_eq!(prev.circle, 4);
        assert_eq!(prev.arc_index, 23);
        assert_eq!(prev.angle, Fraction::from(345));
    }

    #[test]
    fn test_next_in() {
        let coord1_0 = CircleCoordinate::create_with_arc_index(1, 0);
        let prev = coord1_0.next_in().unwrap();
        assert_eq!(prev.circle, 0);
        assert_eq!(prev.arc_index, 0);
        assert_eq!(prev.angle, Fraction::from(0));

        let coord1_1 = CircleCoordinate::create_with_arc_index(1, 1);
        let prev = coord1_1.next_in().unwrap();
        assert_eq!(prev.circle, 0);
        assert_eq!(prev.angle, Fraction::from(60));
        assert_eq!(prev.arc_index, 1);

        let coord2_2 = CircleCoordinate::create_with_arc_index(2, 2);
        let prev = coord2_2.next_in().unwrap();
        assert_eq!(prev.circle, 1);
        assert_eq!(prev.angle, Fraction::from(60));
        assert_eq!(prev.arc_index, 1);

        let coord3_4 = CircleCoordinate::create_with_arc_index(3, 4);
        let prev = coord3_4.next_in().unwrap();
        assert_eq!(prev.circle, 2);
        assert_eq!(prev.angle, Fraction::from(120));
        assert_eq!(prev.arc_index, 4);

        let coord4_6 = CircleCoordinate::create_with_arc_index(4, 6);
        let prev = coord4_6.next_in().unwrap();
        assert_eq!(prev.circle, 3);
        assert_eq!(prev.angle, Fraction::from(90));
        assert_eq!(prev.arc_index, 3);

        let coord0_0 = CircleCoordinate::create_with_arc_index(0, 0);
        let result = coord0_0.next_in();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Cannot move inward from circle 0");
    }
}
