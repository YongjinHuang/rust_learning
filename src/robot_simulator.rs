#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn diff(self, x: i32, y: i32, step: i32) -> (i32, i32) {
        match self {
            Direction::North => (x, y + step),
            Direction::East => (x + step, y),
            Direction::South => (x, y - step),
            Direction::West => (x - step, y),
        }
    }
    pub fn right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
    pub fn left(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self {
            x,
            y,
            d,
        }
    }
    #[must_use]
    pub fn turn_right(self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            d: self.d.right(),
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            d: self.d.left(),
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let dc = self.d;
        let (nx, ny) = dc.diff(self.x, self.y, 1);
        Self {
            x: nx,
            y: ny,
            d: dc,
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut robot = self;
        for c in instructions.chars() {
            robot = match c {
                'L' => robot.turn_left(),
                'R' => robot.turn_right(),
                'A' => robot.advance(),
                _ => robot,
            }
        }
        robot
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}

mod robot_simulator_test {
    use super::*;

    #[test]
    fn at_origin_facing_north() {
        let robot = Robot::new(0, 0, Direction::North);
        assert_eq!(robot.position(), (0, 0));
        assert_eq!(robot.direction(), &Direction::North);
    }

    #[test]
    fn at_negative_position_facing_south() {
        let robot = Robot::new(-1, -1, Direction::South);
        assert_eq!(robot.position(), (-1, -1));
        assert_eq!(robot.direction(), &Direction::South);
    }

    #[test]
    fn changes_north_to_east() {
        let robot_start = Robot::new(0, 0, Direction::North);
        let robot_end = robot_start.turn_right();
        assert_eq!(robot_end.position(), (0, 0));
        assert_eq!(robot_end.direction(), &Direction::East);
    }

    #[test]
    fn changes_east_to_south() {
        let robot_start = Robot::new(0, 0, Direction::East);
        let robot_end = robot_start.turn_right();
        assert_eq!(robot_end.position(), (0, 0));
        assert_eq!(robot_end.direction(), &Direction::South);
    }

    #[test]
    fn changes_south_to_west() {
        let robot_start = Robot::new(0, 0, Direction::South);
        let robot_end = robot_start.turn_right();
        assert_eq!(robot_end.position(), (0, 0));
        assert_eq!(robot_end.direction(), &Direction::West);
    }

    #[test]
    fn changes_west_to_north() {
        let robot_start = Robot::new(0, 0, Direction::West);
        let robot_end = robot_start.turn_right();
        assert_eq!(robot_end.position(), (0, 0));
        assert_eq!(robot_end.direction(), &Direction::North);
    }

    #[test]
    fn changes_north_to_west() {
        let robot_start = Robot::new(0, 0, Direction::North);
        let robot_end = robot_start.turn_left();
        assert_eq!(robot_end.position(), (0, 0));
        assert_eq!(robot_end.direction(), &Direction::West);
    }

    #[test]
    fn changes_west_to_south() {
        let robot_start = Robot::new(0, 0, Direction::West);
        let robot_end = robot_start.turn_left();
        assert_eq!(robot_end.position(), (0, 0));
        assert_eq!(robot_end.direction(), &Direction::South);
    }

    #[test]
    fn changes_south_to_east() {
        let robot_start = Robot::new(0, 0, Direction::South);
        let robot_end = robot_start.turn_left();
        assert_eq!(robot_end.position(), (0, 0));
        assert_eq!(robot_end.direction(), &Direction::East);
    }

    #[test]
    fn changes_east_to_north() {
        let robot_start = Robot::new(0, 0, Direction::East);
        let robot_end = robot_start.turn_left();
        assert_eq!(robot_end.position(), (0, 0));
        assert_eq!(robot_end.direction(), &Direction::North);
    }

    #[test]
    fn facing_north_increments_y() {
        let robot_start = Robot::new(0, 0, Direction::North);
        let robot_end = robot_start.advance();
        assert_eq!(robot_end.position(), (0, 1));
        assert_eq!(robot_end.direction(), &Direction::North);
    }

    #[test]
    fn facing_south_decrements_y() {
        let robot_start = Robot::new(0, 0, Direction::South);
        let robot_end = robot_start.advance();
        assert_eq!(robot_end.position(), (0, -1));
        assert_eq!(robot_end.direction(), &Direction::South);
    }

    #[test]
    fn facing_east_increments_x() {
        let robot_start = Robot::new(0, 0, Direction::East);
        let robot_end = robot_start.advance();
        assert_eq!(robot_end.position(), (1, 0));
        assert_eq!(robot_end.direction(), &Direction::East);
    }

    #[test]
    fn facing_west_decrements_x() {
        let robot_start = Robot::new(0, 0, Direction::West);
        let robot_end = robot_start.advance();
        assert_eq!(robot_end.position(), (-1, 0));
        assert_eq!(robot_end.direction(), &Direction::West);
    }

    #[test]
    fn moving_east_and_north_from_readme() {
        let robot_start = Robot::new(7, 3, Direction::North);
        let robot_end = robot_start.instructions("RAALAL");
        assert_eq!(robot_end.position(), (9, 4));
        assert_eq!(robot_end.direction(), &Direction::West);
    }

    #[test]
    fn moving_west_and_north() {
        let robot_start = Robot::new(0, 0, Direction::North);
        let robot_end = robot_start.instructions("LAAARALA");
        assert_eq!(robot_end.position(), (-4, 1));
        assert_eq!(robot_end.direction(), &Direction::West);
    }

    #[test]
    fn moving_west_and_south() {
        let robot_start = Robot::new(2, -7, Direction::East);
        let robot_end = robot_start.instructions("RRAAAAALA");
        assert_eq!(robot_end.position(), (-3, -8));
        assert_eq!(robot_end.direction(), &Direction::South);
    }

    #[test]
    fn moving_east_and_north() {
        let robot_start = Robot::new(8, 4, Direction::South);
        let robot_end = robot_start.instructions("LAAARRRALLLL");
        assert_eq!(robot_end.position(), (11, 5));
        assert_eq!(robot_end.direction(), &Direction::North);
    }
}