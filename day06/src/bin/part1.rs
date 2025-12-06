fn parse(input: &str) -> (Vec<&str>, Vec<Vec<&str>>) {
    let parts = input.lines().map(|line| line.split_whitespace().collect()).collect::<Vec<Vec<&str>>>();
    let (operators, numbers) = parts.split_last().unwrap();

    assert!(numbers.iter().all(|x| x.len() == numbers[0].len()));
    assert_eq!(numbers[0].len(), operators.len());

    (operators.clone(), Vec::from(numbers))
}

fn solve(input: &str) -> u64 {
    let mut total = 0;
    let (operators, numbers) = parse(input);
    for problem_idx in 0..operators.len() {
        let mut cur_numbers: Vec<u64> = Vec::new();
        for number_idx in 0..numbers.len() {
            cur_numbers.push(numbers[number_idx][problem_idx].parse().unwrap());
        }

        let cur_operator = operators[problem_idx];
        total += match cur_operator {
            "+" => cur_numbers.iter().sum::<u64>(),
            "*" => cur_numbers.iter().product::<u64>(),
            _ => panic!("Unsupported operator"),
        }
    }

    total
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
        let input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +";

        assert_eq!(solve(input), 4277556);
    }
}
