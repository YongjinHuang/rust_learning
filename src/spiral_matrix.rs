
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn turn(&self) -> Self {
        match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }

    fn next_step(&self, (x, y): (usize, usize), size: usize, matrix: &Vec<Vec<u32>>) -> Result<(usize, usize), ()> {
        match self {
            Direction::Left => if y  >= 1 && matrix[x][y - 1] == 0 { Ok((x, y - 1)) } else { Err(()) },
            Direction::Right => if y + 1 < size && matrix[x][y + 1] == 0 { Ok((x, y + 1)) } else { Err(()) },
            Direction::Up => if x  >= 1 && matrix[x - 1][y] == 0 { Ok((x - 1, y)) } else { Err(()) },
            Direction::Down => if x + 1 < size && matrix[x + 1][y] == 0 { Ok((x + 1, y)) } else { Err(()) },
        }
    }
}


pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut res = vec![vec![0; size as usize]; size as usize];
    if size == 0 {
        return res;
    }
    let mut direction = Direction::Right;
    let mut x = 0;
    let mut y = 0;
    for idx in 1..size * size {
        res[x][y] = idx;
        while let Err(()) = direction.next_step((x, y), size as usize, &res) {
            direction = direction.turn();
        }
        if let Ok((nx, ny)) = direction.next_step((x, y), size as usize, &res) {
            (x, y) = (nx, ny);
        }
    }
    res[x][y] = size*size;
    res
}

mod spiral_matrix_test {
    use crate::spiral_matrix::spiral_matrix;

    #[test]
    fn empty_spiral() {
        let input = 0;
        let output = spiral_matrix(input);
        let expected: [[u32; 0]; 0] = [];
        assert_eq!(output, expected);
    }

    #[test]
    fn trivial_spiral() {
        let input = 1;
        let output = spiral_matrix(input);
        let expected: [[u32; 1]; 1] = [[1]];
        assert_eq!(output, expected);
    }

    #[test]
    fn spiral_of_size_2() {
        let input = 2;
        let output = spiral_matrix(input);
        let expected: [[u32; 2]; 2] = [[1, 2], [4, 3]];
        assert_eq!(output, expected);
    }

    #[test]
    fn spiral_of_size_3() {
        let input = 3;
        let output = spiral_matrix(input);
        let expected: [[u32; 3]; 3] = [[1, 2, 3], [8, 9, 4], [7, 6, 5]];
        assert_eq!(output, expected);
    }

    #[test]
    fn spiral_of_size_4() {
        let input = 4;
        let output = spiral_matrix(input);
        let expected: [[u32; 4]; 4] = [
            [1, 2, 3, 4],
            [12, 13, 14, 5],
            [11, 16, 15, 6],
            [10, 9, 8, 7],
        ];
        assert_eq!(output, expected);
    }

    #[test]
    fn spiral_of_size_5() {
        let input = 5;
        let output = spiral_matrix(input);
        let expected: [[u32; 5]; 5] = [
            [1, 2, 3, 4, 5],
            [16, 17, 18, 19, 6],
            [15, 24, 25, 20, 7],
            [14, 23, 22, 21, 8],
            [13, 12, 11, 10, 9],
        ];
        assert_eq!(output, expected);
    }
}
