use std::collections::{HashMap};

pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    nums.iter().filter(|&&num| num < k).count() as i32
}

pub fn is_possible_to_split(nums: Vec<i32>) -> bool {
    let mut frequency_map : HashMap<i32, usize> = HashMap::new();
    for num in nums {
        *frequency_map.entry(num).or_insert(0) += 1
    }
    *frequency_map.values().max().unwrap() <= 2
}