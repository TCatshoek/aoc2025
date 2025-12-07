use std::collections::{HashSet, VecDeque};
use glam::IVec2;
use aoc2025::world::World;

const DOWN: IVec2 = IVec2::new(0, 1);
const LEFT: IVec2 = IVec2::new(-1, 0);
const RIGHT: IVec2 = IVec2::new(1, 0);

fn solve(input: &str) -> u64 {
    let world = World::new(input);

    let mut pending = VecDeque::from(world.find_ivec2('S'));
    let mut visited = HashSet::new();
    let mut split_count = 0;

    while let Some(position) = pending.pop_front() {
        if visited.contains(&position) {
            continue;
        }

        visited.insert(position);

        let next_pos = position + DOWN;

        match world.get_at(next_pos) {
            Some('.') => {
                pending.push_back(next_pos)
            }
            Some('^') => {
                pending.push_back(next_pos + LEFT);
                pending.push_back(next_pos + RIGHT);
                split_count += 1;
            }
            Some(_) => panic!("Unknown tile"),
            None => {}
        }
    }
    split_count
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
        assert_eq!(solve(input), 21);
    }
}