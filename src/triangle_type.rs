pub fn divide_array(nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    let mut nums = nums.clone();
    nums.sort();
    let chunks = nums.chunks(3).collect::<Vec<_>>();
    if chunks.iter().all(|chunk| chunk[2] - chunk[0] <= k) {
        chunks.into_iter().map(|chunk| chunk.to_vec()).collect()
    } else {
        vec![]
    }
}

pub fn triangle_type(nums: Vec<i32>) -> String {
    let a = nums[0];
    let b = nums[1];
    let c = nums[2];
    if a > 0 && b > 0 && c > 0 && a + b > c && a + c > b && b + c > a {
        if a == b && b == c {
            return "equilateral".to_string();
        }
        if a == b || b == c || a == c {
            return "isosceles".to_string();
        }
        return "scalene".to_string();
    }
    "none".to_string()
}

pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
    dimensions
        .iter()
        .map(|dimension| {
            (
                dimension[0] * dimension[0] + dimension[1] * dimension[1],
                dimension[0] * dimension[1],
            )
        })
        .max()
        .unwrap()
        .1
}

mod ha_test {
    use super::area_of_max_diagonal;
    use super::divide_array;

    #[test]
    fn divide_array_test() {
        let case1 = (vec![1, 3, 4, 8, 7, 9, 3, 5, 1], 2, vec![vec![1, 1, 3], vec![3, 4, 5], vec![7, 8, 9]]);
        let case2: (Vec<i32>, i32, Vec<Vec<i32>>) = (vec![1, 3, 3, 2, 7, 3], 3, vec![]);
        assert_eq!(divide_array(case1.0, case1.1), case1.2);
        assert_eq!(divide_array(case2.0, case2.1), case2.2);
    }

    #[test]
    fn test_t() {
        let case1 = (vec![vec![9, 3], vec![8, 6]], 48);
        assert_eq!(area_of_max_diagonal(case1.0), case1.1)
    }
}
