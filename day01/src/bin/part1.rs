fn solve_part1(input: &str) -> u32 {
    input
        .lines()
        .fold((50i32, 0u32), |(pos, count), line| {
            let (dir, amt) = line.split_at(1);
            let amt = amt.parse::<i32>().unwrap();

            let delta = match dir {
                "R" => amt,
                "L" => -amt,
                _ => panic!("Unknown direction: {}", dir),
            };

            let new_pos = (pos + delta).rem_euclid(100);
            let new_count = count + (new_pos == 0) as u32;

            (new_pos, new_count)
        })
        .1
}

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", solve_part1(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example() {
        let test_input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(solve_part1(test_input), 3);
    }
}
