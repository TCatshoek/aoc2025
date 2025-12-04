use aoc2025::world::World;
use glam::IVec2;

fn count_around(world: &World, pos: &IVec2, c: char) -> usize {
    World::directions()
        .iter()
        .map(|dir| pos + dir)
        .filter(|&pos| world.get_at(pos).is_some_and(|x| x == c))
        .count()
}

fn step(world: &mut World) -> Option<usize> {
    let removable = world
        .find_ivec2('@')
        .into_iter()
        .filter(|pos| count_around(world, pos, '@') < 4)
        .collect::<Vec<_>>();

    let count = removable.len();

    for pos in removable {
        world.set_at(pos, '.');
    }

    (count > 0).then_some(count)
}

fn solve(world: &mut World) -> usize {
    let mut count = 0;

    while let Some(step) = step(world) {
        count += step;
    }

    count
}

fn main() {
    let input = include_str!("../input.txt");
    let mut world = World::new(input);
    println!("Result: {}", solve(&mut world));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        let mut world = World::new(input);
        assert_eq!(solve(&mut world), 43)
    }
}
