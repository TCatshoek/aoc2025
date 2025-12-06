use aoc2025::world::World;



fn solve(input: &str) -> u64 {
    let world = World::new(input);

    let mut operator = ' ';
    let mut numbers: Vec<u64> = Vec::new();
    let mut total = 0;
    for column in world.iter_cols().rev() {
        let mut col_chars = Vec::new();
        for element in column {
            let el: char = element;

            match el {
                d if d.is_numeric() => col_chars.push(d),
                '*' => operator = '*',
                '+' => operator = '+',
                ' ' => {},
                _ => panic!("aah")
            }
        }

        if col_chars.len() == 0 {
            continue;
        }

        numbers.push(col_chars.into_iter().collect::<String>().parse().unwrap());
        total += match operator {
            '*' => numbers.iter().product::<u64>(),
            '+' => numbers.iter().sum::<u64>(),
            ' ' => 0,
            _ => panic!("aaah")
        };
        if operator != ' ' {
            numbers.clear();
            operator = ' ';
        };
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
        let input = include_str!("../example.txt");

        assert_eq!(solve(input), 3263827);
    }
}
