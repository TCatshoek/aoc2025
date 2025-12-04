fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn find_best(input: &[u32]) -> u32 {
    let mut max = 0;
    let mut max_total = 0;
    for i in 0..input.len() {
        let left = input[i] * 10;
        if left > max {
            max = left;
            for j in i + 1..input.len() {
                if left + input[j] > max_total {
                    max_total = left + input[j];
                }
            }
        }
    }
    max_total
}

fn solve(input: &str) -> u32 {
    let banks = parse(input);
    banks.iter().map(|bank| find_best(bank)).sum()
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
        assert_eq!(solve(input), 357);
    }
}
