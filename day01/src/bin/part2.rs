fn solve_part2(input: &str) -> u32 {
    println!("(50, 0)");
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

            let moved = (pos + delta);
            let passes = (moved / 100).abs() as u32;
            let new_pos = moved.rem_euclid(100);
            let modifier = (moved <= 0 && pos != 0) as u32;
            (new_pos, count + passes + modifier)
        })
        .1
}

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", solve_part2(input));
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
        assert_eq!(solve_part2(test_input), 6);
    }

    #[test]
    fn test_example2() {
        let test_input = "R1000";
        assert_eq!(solve_part2(test_input), 10);
    }

    #[test]
    fn test_example3() {
        let test_input = "R100
R101
R149";
        assert_eq!(solve_part2(test_input), 4);
    }

    #[test]
    fn wtf() {
        println!("{:?}", -18i32.div_euclid(100));
    }

    #[test]
    fn test_example4() {
        let test_input = "L50
R50
L50
L50
R50
L50
R50
R50";
        assert_eq!(solve_part2(test_input), 4);
    }

    #[test]
    fn test_example5() {
        let test_input = "L150
L50
L150
R50
R150
L50
R150
R50";
        assert_eq!(solve_part2(test_input), 8);
    }
}
