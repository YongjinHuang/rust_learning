pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let n = grid.len();
    let mut count_vec = vec![0; n * n + 1];
    for row in grid {
        for value in row {
            count_vec[value as usize] += 1;
        }
    }
    let mut res = vec![0, 0];
    for i in 1..=n * n {
        if count_vec[i] == 2 {
            res[0] = i as i32;
        } else if count_vec[i] == 0 {
            res[1] = i as i32;
        }
    }
    res
}

pub fn last_visited_integers(words: Vec<String>) -> Vec<i32> {
    let mut seen = vec![];
    let mut res = vec![];
    let mut accu = 0;
    for word in words {
        if word != "prev" {
            seen.push(word);
            accu = 0;
        } else {
            match seen.get(seen.len() - 1 - accu) {
                None => res.push(-1),
                Some(v) => res.push(v.parse::<i32>().unwrap()),
            }
            accu += 1;
        }
    }
    res
}

pub fn distribute_candies(n: i32, limit: i32) -> i32 {
    (0..=limit)
        .flat_map(|i| (0..=limit)
            .flat_map(move |j| (0..=limit)
                .filter(move |&k| i + j + k == n)))
        .count() as i32
}