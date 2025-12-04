use std::cmp::max;
use std::collections::HashMap;

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn find_best(input: &[u32], remaining: usize, cache: &mut HashMap<(usize, usize), u64>) -> u64 {
    if remaining == 0 || remaining > input.len() {
        return 0;
    }

    let key = (input.len(), remaining);
    if let Some(&result) = cache.get(&key) {
        return result;
    }

    let result = if input.len() == 1 {
        input[0] as u64
    } else {
        let (&head, tail) = input.split_first().unwrap();
        let cur = (head as u64) * 10u64.pow((remaining - 1) as u32);
        max(
            cur + find_best(tail, remaining - 1, cache),
            find_best(tail, remaining, cache),
        )
    };

    cache.insert(key, result);

    result
}

fn solve(input: &str) -> u64 {
    let banks = parse(input);
    banks.iter().map(|bank| {
        let mut cache = HashMap::new();
        find_best(bank, 12, &mut cache)
    }).sum()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", solve(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(solve(input), 3121910778619);
    }

    #[test]
    fn test_example_1() {
        let input = "987654321111111";
        assert_eq!(solve(input), 987654321111);
    }

    #[test]
    fn test_example_2() {
        let input = "811111111111119";
        assert_eq!(solve(input), 811111111119);
    }

    #[test]
    fn test_example_3() {
        let input = "234234234234278";
        assert_eq!(solve(input), 434234234278);
    }
}
