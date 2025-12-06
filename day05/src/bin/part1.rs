struct Range(u64, u64);

impl Range {
    fn is_in(&self, pos: u64) -> bool {
        pos >= self.0 && pos <= self.1
    }

    fn from_str(input: &str) -> Option<Range> {
        if let Some((left, right)) = input.split_once("-") {
            return Some(Range(left.parse().unwrap(), right.parse().unwrap()));
        }
        None
    }
}

fn parse(input: &str) -> (Vec<Range>, Vec<u64>) {
    let lines = input.lines().into_iter();

    let mut ranges = Vec::new();
    let mut numbers = Vec::new();
    let mut processing_ranges = true;
    for line in lines {
        if line.is_empty() {
            processing_ranges = false;
            continue;
        }

        if processing_ranges {
            ranges.push(Range::from_str(line).unwrap())
        } else {
            numbers.push(line.parse::<u64>().unwrap());
        }
    }

    (ranges, numbers)
}

fn solve(input: &str) -> u64 {
    let (ranges, numbers) = parse(input);
    let mut count = 0;
    for num in numbers {
        if ranges.iter().map(|r| r.is_in(num)).any(|x| x) {
            count += 1;
        }
    }
    count
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
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!(solve(input), 3);
    }
}
