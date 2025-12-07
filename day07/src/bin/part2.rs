use aoc2025::world::World;
use glam::IVec2;
use std::collections::{HashMap, VecDeque};

const DOWN: IVec2 = IVec2::new(0, 1);
const LEFT: IVec2 = IVec2::new(-1, 0);
const RIGHT: IVec2 = IVec2::new(1, 0);

fn descend(position: IVec2, world: &World, cache: &mut HashMap::<IVec2, u64>) {
    let mut count = 0;

    let mut pending: VecDeque::<IVec2> = match world.get_at(position).unwrap() {
        '^' => VecDeque::from([position + LEFT, position + RIGHT]),
        'S' => VecDeque::from([position]),
        _ => panic!("Unexpected square")
    };

    while let Some(position) = pending.pop_front() {
        let next_pos = position + DOWN;

        match world.get_at(next_pos) {
            Some('.') => pending.push_back(next_pos),
            Some('^') => {
                count += cache.get(&next_pos).expect("Should have already computed")
            }
            Some(_) => panic!("Unknown tile"),
            None => count += 1,
        }
    }
    cache.insert(position, count);
}

fn solve(input: &str) -> u64 {
    let world = World::new(input);

    let mut splitters = world.find_ivec2('^');
    let start_pos = *world.find_ivec2('S').first().unwrap();

    splitters.sort_by(|a, b| b.y.cmp(&a.y));
    splitters.push(start_pos);

    let mut timelines_per_splitter = HashMap::<IVec2, u64>::new();

    for splitter in splitters {
        descend(splitter, &world, &mut timelines_per_splitter)
    }

    *timelines_per_splitter.get(&start_pos).unwrap()
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
        assert_eq!(solve(input), 40);
    }
}
