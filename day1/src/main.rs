use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();

    let mut unique_nums_right = HashMap::<i64, i64>::new();

    for line in input.lines() {
        let mut segments = line.split_whitespace();

        let left_num = segments.next().expect("no left value").parse().unwrap();
        let right_num = segments.next().expect("no right value").parse().unwrap();

        left.push(left_num);
        right.push(right_num);

        unique_nums_right.insert(
            right_num,
            unique_nums_right.get(&right_num).unwrap_or(&0) + 1,
        );
    }

    left.sort();
    right.sort();

    let mut total_diff = 0;

    for (l, r) in left.iter().zip(right.iter()) {
        let abs_diff = (l - r).abs();
        total_diff += abs_diff;
    }

    println!("Total difference: {}", total_diff);

    let similarity_score = left.iter().fold(0, |acc, num| {
        let right_count = unique_nums_right.get(num).unwrap_or(&0);

        acc + num * right_count
    });

    println!("Similarity score: {}", similarity_score);
}
