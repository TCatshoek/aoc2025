#![feature(duration_millis_float)]

use std::time::Instant;

fn solve(input: &str) -> u64 {
    let ranges = input.split(',').map(|range| {
        let (left, right) = range.split_once('-').expect("missing dash");
        let left = left.parse::<u64>().expect("not a number");
        let right = right.parse::<u64>().expect("not a number");
        (left, right)
    });

    let mut count = 0;

    for (low, high) in ranges {
        for id in low..=high {
            let id_str = id.to_string();
            let (left, right) = id_str.split_at(id_str.len() / 2);
            if left == right {
                count += id;
            }
        }
    }

    count
}

fn main() {
    let input = include_str!("../input.txt");
    let start = Instant::now();
    let result = solve(input);
    let duration = start.elapsed();
    println!("{:.2?}ms", duration.as_millis_f32());
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(solve(input), 1227775554);
    }
}