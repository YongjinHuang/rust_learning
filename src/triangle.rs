pub struct Triangle(u64, u64, u64);

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if sides.iter().any(|&side| side == 0) {
            return None;
        }
        let [a, b, c] = sides;
        if a + b >= c && b + c >= a && a + c >= b {
            Some(Triangle(a, b, c))
        } else {
            None
        }
    }
    pub fn is_equilateral(&self) -> bool {
        self.0 == self.1 && self.1 == self.2
    }
    pub fn is_scalene(&self) -> bool {
        !self.is_isosceles()
    }
    pub fn is_isosceles(&self) -> bool {
        self.0 == self.1 || self.1 == self.2 || self.0 == self.2
    }
}

mod triangle_test {
    use crate::triangle::Triangle;

    #[test]
    fn all_sides_are_equal() {
        let input = [2, 2, 2];
        let output = Triangle::build(input).unwrap();
        assert!(output.is_equilateral());
    }

    #[test]
    fn any_side_is_unequal() {
        let input = [2, 3, 2];
        let output = Triangle::build(input).unwrap();
        assert!(!output.is_equilateral());
    }

    #[test]
    fn no_sides_are_equal() {
        let input = [5, 4, 6];
        let output = Triangle::build(input).unwrap();
        assert!(!output.is_equilateral());
    }

    #[test]
    #[cfg(feature = "generic")]
    fn sides_may_be_floats() {
        let input = [0.5, 0.5, 0.5];
        let output = Triangle::build(input).unwrap();
        assert!(output.is_equilateral());
    }
}

mod isosceles {
    use crate::triangle::Triangle;

    #[test]
    fn last_two_sides_are_equal() {
        let input = [3, 4, 4];
        let output = Triangle::build(input).unwrap();
        assert!(output.is_isosceles());
    }

    #[test]
    fn first_two_sides_are_equal() {
        let input = [4, 4, 3];
        let output = Triangle::build(input).unwrap();
        assert!(output.is_isosceles());
    }

    #[test]
    fn first_and_last_sides_are_equal() {
        let input = [4, 3, 4];
        let output = Triangle::build(input).unwrap();
        assert!(output.is_isosceles());
    }

    #[test]
    fn equilateral_triangles_are_also_isosceles() {
        let input = [4, 4, 4];
        let output = Triangle::build(input).unwrap();
        assert!(output.is_isosceles());
    }

    #[test]
    fn no_sides_are_equal() {
        let input = [2, 3, 4];
        let output = Triangle::build(input).unwrap();
        assert!(!output.is_isosceles());
    }

    #[test]
    #[cfg(feature = "generic")]
    fn sides_may_be_floats() {
        let input = [0.5, 0.4, 0.5];
        let output = Triangle::build(input).unwrap();
        assert!(output.is_isosceles());
    }
}

mod scalene {
    use crate::triangle::Triangle;

    #[test]
    fn no_sides_are_equal() {
        let input = [5, 4, 6];
        let output = Triangle::build(input).unwrap();
        assert!(output.is_scalene());
    }

    #[test]
    fn all_sides_are_equal() {
        let input = [4, 4, 4];
        let output = Triangle::build(input).unwrap();
        assert!(!output.is_scalene());
    }

    #[test]
    fn first_and_second_sides_are_equal() {
        let input = [4, 4, 3];
        let output = Triangle::build(input).unwrap();
        assert!(!output.is_scalene());
    }

    #[test]
    fn first_and_third_sides_are_equal() {
        let input = [3, 4, 3];
        let output = Triangle::build(input).unwrap();
        assert!(!output.is_scalene());
    }

    #[test]
    fn second_and_third_sides_are_equal() {
        let input = [4, 3, 3];
        let output = Triangle::build(input).unwrap();
        assert!(!output.is_scalene());
    }

    #[test]
    #[cfg(feature = "generic")]
    fn sides_may_be_floats() {
        let input = [0.5, 0.4, 0.6];
        let output = Triangle::build(input).unwrap();
        assert!(output.is_scalene());
    }
}

mod invalid {
    use crate::triangle::Triangle;

    #[test]
    fn all_zero_sides_is_not_a_triangle() {
        let input = [0, 0, 0];
        let output = Triangle::build(input);
        assert!(output.is_none());
    }

    #[test]
    fn first_triangle_inequality_violation() {
        let input = [1, 1, 3];
        let output = Triangle::build(input);
        assert!(output.is_none());
    }

    #[test]
    fn second_triangle_inequality_violation() {
        let input = [1, 3, 1];
        let output = Triangle::build(input);
        assert!(output.is_none());
    }

    #[test]
    fn third_triangle_inequality_violation() {
        let input = [3, 1, 1];
        let output = Triangle::build(input);
        assert!(output.is_none());
    }

    #[test]
    fn may_not_violate_triangle_inequality() {
        let input = [7, 3, 2];
        let output = Triangle::build(input);
        assert!(output.is_none());
    }
}