use std::collections::{HashMap, HashSet};
use std::ops::{Mul, Sub};
pub fn max_operations(nums: Vec<i32>) -> i32 {
    let mut res = 1;
    let sum = nums[0] + nums[1];
    for i in (2..(nums.len()-2)).step_by(2) {
        if nums[i] + nums[i+1] == sum {
            res += 1
        } else {
            break
        }
    }
    res
}
pub fn min_processing_time(processor_time: Vec<i32>, tasks: Vec<i32>) -> i32 {
    let mut processor_time = processor_time.clone();
    processor_time.sort_by(|a, b| a.cmp(b));
    let mut tasks = tasks.clone();
    tasks.sort_by(|a, b| b.cmp(a));
    tasks
        .chunks(4)
        .enumerate()
        .map(|(idx, chunk)| processor_time[idx] + chunk.iter().max().unwrap())
        .max().unwrap()
}

pub fn minimum_pushes(word: String) -> i32 {
    let n = word.len() as i32;
    n + (n - 8).max(0) + (n - 16).max(0) + (n - 24).max(0)
}

pub fn minimum_cost(nums: Vec<i32>) -> i32 {
    let mut res = nums[0];
    let mut tmp = i32::MAX;
    for i in 1..(nums.len() - 1) {
        for j in i + 1..nums.len() {
            tmp = tmp.min(nums[i] + nums[j])
        }
    }
    res + tmp
}

pub fn find_k_or(nums: Vec<i32>, k: i32) -> i32 {
    let bits = nums.iter()
        .max()
        .map(|&maxv| format!("{:b}", maxv).len())
        .unwrap();
    let mut res = 0;
    for bit in 0..bits {
        let mut tmp = 0;
        for &num in nums.iter() {
            if num & (1 << bit) > 0 {
                tmp += 1;
            }
        }
        if tmp >= k {
            res += 1 << bit;
        }
    }
    res
}

pub fn modified_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut column_max_value = vec![-1; matrix[0].len()];
    for j in 0..matrix[0].len() {
        for i in 0..matrix.len() {
            column_max_value[j] = column_max_value[j].max(matrix[i][j])
        }
    }
    let mut res = matrix.clone();
    for row in res.iter_mut() {
        for i in 0..row.len() {
            if row[i] == -1 {
                row[i] = column_max_value[i]
            }
        }
    }
    res
}
pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut res = 0;
    for i in 0..n - 2 {
        for j in i + 1..n - 1 {
            for k in j + 1..n {
                res = res.max((nums[i] as i64 - nums[j] as i64) * nums[k] as i64)
            }
        }
    }
    res
}

pub fn number_count(a: i32, b: i32) -> i32 {
    (a..=b)
        .filter(is_number_unique_digits)
        .count() as i32
}

fn is_number_unique_digits(num: &i32) -> bool {
    let mut digits = vec![0; 10];
    let mut num = *num;
    while num > 0 {
        let digit = num % 10;
        if digits[digit as usize] > 0 {
            return false;
        }
        digits[digit as usize] += 1;
        num /= 10;
    }
    true
}

const KEYPAD: [(char, usize); 26] = [
    ('a', 1), ('b', 1),
    ('c', 2), ('d', 2), ('e', 2),
    ('f', 3), ('g', 3), ('h', 3),
    ('i', 4), ('j', 4), ('k', 4),
    ('l', 5), ('m', 5), ('n', 5),
    ('o', 6), ('p', 6), ('q', 6),
    ('r', 7), ('s', 7), ('t', 7),
    ('u', 8), ('v', 8), ('w', 8),
    ('x', 9), ('y', 9), ('z', 9),
];

fn create_keypad_map() -> HashMap<char, usize> {
    KEYPAD.iter().cloned().collect()
}

pub fn count_divisible_substrings(word: String) -> i32 {
    let chars = word.chars().collect::<Vec<char>>();
    let hmap = create_keypad_map();
    let mut partial_sums: Vec<usize> = vec![0];
    let mut accu = 0usize;
    for c in chars.iter() {
        accu += hmap.get(c).unwrap();
        partial_sums.push(accu);
    }
    (1..=word.len()).into_iter()
        .fold(0, |accu, i| accu +
            chars.windows(i)
                .enumerate()
                .filter(|(idx, _)| (partial_sums[idx + i] - partial_sums[*idx]) % i == 0)
                .count() as i32,
        )
}

pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut kset = HashSet::new();
    for idx in (0..nums.len()).rev() {
        if nums[idx] <= k {
            kset.insert(nums[idx]);
        }
        if kset.len() == k as usize {}
    }
    panic!("input error")
}

pub fn most_expensive_item(prime_one: i32, prime_two: i32) -> i32 {
    prime_one * prime_two - prime_one - prime_two
}

pub fn return_to_boundary_count(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut accu = 0;
    for num in nums {
        accu += num;
        if accu == 0 {
            res += 1;
        }
    }
    res
}

pub fn count_key_changes(s: String) -> i32 {
    s.chars().collect::<Vec<char>>()
        .windows(2)
        .filter(|&w| w[0].eq_ignore_ascii_case(&w[1]))
        .count() as i32
}


pub fn missing_integer(nums: Vec<i32>) -> i32 {
    let lsp = (0..nums.len() - 1)
        .into_iter()
        .take_while(|&i| nums[i] + 1 == nums[i + 1])
        .last()
        .and_then(|i| Some(nums[0..=i + 1].iter().sum()))
        .unwrap_or(nums[0]);
    (lsp..=50)
        .take_while(|target| nums.contains(target))
        .last()
        .and_then(|num| Some(num + 1))
        .unwrap_or(lsp)
}

pub fn find_minimum_operations(s1: String, s2: String, s3: String) -> i32 {
    (0..s1.len().min(s2.len().min(s3.len())))
        .into_iter()
        .take_while(|&i| s1[i..i + 1] == s2[i..i + 1] && s2[i..i + 1] == s3[i..i + 1])
        .last()
        .and_then(|mi| Some((s1.len() + s2.len() + s3.len() - 3 * (mi + 1)) as i32))
        .unwrap_or(-1)
}

pub fn maximum_strong_pair_xor(nums: Vec<i32>) -> i32 {
    nums.iter()
        .flat_map(|&x| nums.iter().filter_map(move |&y| ((x - y).abs() <= x.min(y)).then(|| x ^ y)))
        .max()
        .unwrap_or_default()
}

pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
    let map = nums.iter().fold(HashMap::new(), |mut hmap, &num| {
        *hmap.entry(num).or_insert(0) += 1;
        hmap
    });
    let max_freq = *map.values().max().unwrap_or(&0);
    map.values().fold(0, |acc, &v| if v == max_freq { acc + v } else { acc })
}

pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
    words
        .iter()
        .enumerate()
        .filter_map(|(index, element)| {
            if element.contains(x) {
                Some(index as i32)
            } else {
                None
            }
        })
        .collect()
}

pub fn number_game(nums: Vec<i32>) -> Vec<i32> {
    let mut sorted_nums = nums.clone();
    sorted_nums.sort();
    for i in 0..sorted_nums.len() / 2 {
        sorted_nums.swap(i * 2, i * 2 + 1);
    }
    sorted_nums
}

pub fn count_tested_devices(battery_percentages: Vec<i32>) -> i32 {
    battery_percentages
        .iter()
        .fold((0, 0), |(tested_devices, penalty), &battery_percentage| {
            let battery_percentage = battery_percentage.saturating_sub(penalty);
            if battery_percentage > 0 {
                (tested_devices + 1, penalty + 1)
            } else {
                (tested_devices, penalty)
            }
        })
        .0
}

pub fn difference_of_sums(n: i32, m: i32) -> i32 {
    (1..=n).filter(|&num| num % m != 0).sum::<i32>()
        - (1..=n).filter(|&num| num % m == 0).sum::<i32>()
}

pub fn find_intersection_values(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut c1 = 0;
    let mut c2 = 0;
    for &num1 in nums1.iter() {
        for &num2 in nums2.iter() {
            if num1 == num2 {
                c1 += 1;
                break;
            }
        }
    }
    for &num2 in nums2.iter() {
        for &num1 in nums1.iter() {
            if num1 == num2 {
                c2 += 1;
                break;
            }
        }
    }
    vec![c1, c2]
}

pub fn sum_counts(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut result = 0;
    for i in 0..n {
        for j in i..n {
            let k = nums[i..=j]
                .iter()
                .collect::<std::collections::HashSet<&i32>>()
                .len();
            result += (k as i32) * (k as i32);
        }
    }
    result
}

pub fn find_peaks(mountain: Vec<i32>) -> Vec<i32> {
    (1..mountain.len() - 1)
        .into_iter()
        .filter_map(|idx| {
            if mountain[idx] > mountain[idx - 1] && mountain[idx] > mountain[idx + 1] {
                Some(idx as i32)
            } else {
                None
            }
        })
        .collect()
}

pub fn find_champion(grid: Vec<Vec<i32>>) -> i32 {
    let a: Vec<i32> = grid
        .iter()
        .enumerate()
        .filter_map(|(i, team)| {
            if team
                .iter()
                .filter(|&&score| score == 1)
                .count() == grid.len() - 1 {
                Some(i as i32)
            } else {
                None
            }
        }
        ).collect();
    if a.len() >= 1 {
        a[0]
    } else {
        -1
    }
}

pub fn find_indices(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in i..nums.len() {
            if (i as i32 - j as i32).abs() >= index_difference && (nums[i] - nums[j]).abs() >= value_difference {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![-1, -1]
}

fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
    todo!()
}

fn has_trailing_zeros(nums: Vec<i32>) -> bool {
    nums.iter().filter(|&num| num & 1 == 0).count() > 1
}
pub fn minimum_sum(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let nums: &Vec<i32> = nums.as_ref();
    (0..n - 2)
        .flat_map(|i| (i + 1..n - 1)
            .flat_map(move |j| (j + 1..n)
                .filter_map(move |k| if nums[i] < nums[j] && nums[k] < nums[j] { Some(nums[i] + nums[j] + nums[k]) } else { None })))
        .min().unwrap_or(-1)
}




#[cfg(test)]
mod leetcode_tests {
    #[test]
    fn test_count_divisible_substrings() {
        let case1 = ("asdf".to_string(), 6);
        let case2 = ("bdh".to_string(), 4);
        let case3 = ("abcd".to_string(), 6);
        assert_eq!(count_divisible_substrings(case1.0), case1.1);
        assert_eq!(count_divisible_substrings(case2.0), case2.1);
        assert_eq!(count_divisible_substrings(case3.0), case3.1);
}
    #[test]
    fn test_min_processing_time() {
        let case1 = (vec![8, 10], vec![2, 2, 3, 1, 8, 7, 4, 5], 16);
        let case2 = (vec![10, 20], vec![2, 3, 1, 2, 5, 8, 4, 3], 23);
        let case3 = (vec![121, 99], vec![287, 315, 293, 260, 333, 362, 69, 233], 461);
        assert_eq!(min_processing_time(case1.0, case1.1), case1.2);
        assert_eq!(min_processing_time(case2.0, case2.1), case2.2);
        assert_eq!(min_processing_time(case3.0, case3.1), case3.2);
    }

    #[test]
    fn test_find_k_or() {
        let case1 = (vec![7, 12, 9, 8, 9, 15], 4, 9);
        let case2 = (vec![2, 12, 1, 11, 4, 5], 6, 0);
        let case3 = (vec![10, 8, 5, 9, 11, 6, 8], 1, 15);
        assert_eq!(find_k_or(case1.0, case1.1), case1.2);
        assert_eq!(find_k_or(case2.0, case2.1), case2.2);
        assert_eq!(find_k_or(case3.0, case3.1), case3.2);
    }

    #[test]
    fn test_maximum_triplet_value() {
        assert_eq!(maximum_triplet_value(vec![12, 6, 1, 2, 7]), 77);
        assert_eq!(maximum_triplet_value(vec![1, 10, 3, 4, 19]), 133);
        assert_eq!(maximum_triplet_value(vec![1, 2, 3]), 0);
        assert_eq!(maximum_triplet_value(vec![1000000, 1, 1000000]), 999999000000);
    }

    #[test]
    fn test_missing_integers() {
        assert_eq!(missing_integer(vec![1, 2, 3, 2, 5]), 6);
        assert_eq!(missing_integer(vec![3, 4, 5, 1, 12, 14, 13]), 15);
    }

    #[test]
    fn test_find_minimum_operations() {
        assert_eq!(find_minimum_operations("bcbb".into(), "bccbabb".into(), "bcabb".into()), 10);
    }

    #[test]
    fn test_minimum_sum() {
        let case1 = vec![8, 6, 1, 5, 3];
        let case2 = vec![5, 4, 8, 7, 10, 2];
        let case3 = vec![6, 5, 4, 3, 4, 5];
        assert_eq!(minimum_sum(case1), 9);
        assert_eq!(minimum_sum(case2), 13);
        assert_eq!(minimum_sum(case3), -1);
    }

    #[test]
    fn test_has_trailing_zeros() {
        let case1 = (vec![1, 2, 3, 4, 5], true);
        let case2 = (vec![2, 4, 8, 16], true);
        let case3 = (vec![1, 3, 5, 7, 9], false);
        assert_eq!(has_trailing_zeros(case1.0), case1.1);
        assert_eq!(has_trailing_zeros(case2.0), case2.1);
        assert_eq!(has_trailing_zeros(case3.0), case3.1);
    }

    #[test]
    fn test_find_indices() {
        let case1 = (vec![5, 1, 4, 1], 2, 4, vec![0, 3]);
        assert_eq!(find_indices(case1.0, case1.1, case1.2), case1.3);
    }

    #[test]
    fn test_find_champion() {
        let case1 = (vec![vec![0, 1], vec![0, 0]], 0);
        let case2 = (vec![vec![0, 0, 1], vec![1, 0, 1], vec![0, 0, 0]], 1);
        assert_eq!(find_champion(case1.0), case1.1);
        assert_eq!(find_champion(case2.0), case2.1);
    }

    #[test]
    fn test_find_peaks() {
        let case1 = (vec![1, 2, 3, 2, 1], vec![2]);
        let case2 = (vec![1, 2, 3, 3, 4, 5, 4, 3, 2, 1], vec![5]);
        let case3 = (vec![1, 2, 3, 3, 4, 5, 4, 3, 2, 1, 2, 3], vec![5]);
        let case4 = (vec![2, 4, 4], vec![]);
        let case5 = (vec![1, 4, 3, 8, 5], vec![1, 3]);
        assert_eq!(find_peaks(case1.0), case1.1);
        assert_eq!(find_peaks(case2.0), case2.1);
        assert_eq!(find_peaks(case3.0), case3.1);
        assert_eq!(find_peaks(case4.0), case4.1);
        assert_eq!(find_peaks(case5.0), case5.1);
    }

    #[test]
    fn test_sum_counts() {
        let case1 = (vec![1, 2, 1], 15);
        let case2 = (vec![1, 1], 3);
        assert_eq!(sum_counts(case1.0), case1.1);
        assert_eq!(sum_counts(case2.0), case2.1);
    }

    #[test]
    fn test_find_intersection_values() {
        let case1 = (vec![4, 3, 2, 3, 1], vec![2, 2, 5, 2, 3, 6], vec![3, 4]);
        let case2 = (vec![3, 4, 2, 3], vec![1, 5], vec![0, 0]);
        assert_eq!(find_intersection_values(case1.0, case1.1), case1.2);
        assert_eq!(find_intersection_values(case2.0, case2.1), case2.2);
    }

    use super::*;

    #[test]
    fn test_count_tested_devices() {
        let case1 = (vec![1, 1, 2, 1, 3], 3);
        let case2 = (vec![0, 1, 2], 2);
        assert_eq!(count_tested_devices(case1.0), case1.1);
        assert_eq!(count_tested_devices(case2.0), case2.1);
    }

    #[test]
    fn test_difference_of_sums() {
        let case1 = (10, 3, 19);
        let case2 = (5, 6, 15);
        let case3 = (5, 1, -15);
        assert_eq!(difference_of_sums(case1.0, case1.1), case1.2);
        assert_eq!(difference_of_sums(case2.0, case2.1), case2.2);
        assert_eq!(difference_of_sums(case3.0, case3.1), case3.2);
    }

    #[test]
    fn test_number_game() {
        let case1 = vec![5, 4, 2, 3];
        assert_eq!(number_game(case1), vec![3, 2, 5, 4]);
        let case1 = vec![2, 5];
        assert_eq!(number_game(case1), vec![5, 2]);
    }

    #[test]
    fn test_find_words_containing() {
        let words = vec![
            String::from("hello"),
            String::from("world"),
            String::from("yes"),
        ];
        let x = 'o';
        let result = find_words_containing(words, x);
        assert_eq!(result, vec![0, 1]);
    }
}
