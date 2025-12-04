use aoc2025::world::World;
use glam::IVec2;

fn count_around(world: &World, pos: &IVec2, c: char) -> usize {
    World::directions()
        .iter()
        .map(|dir| pos + dir)
        .filter(|&pos| world.get_at(pos).is_some_and(|x| x == c))
        .count()
}

fn solve(world: &World) -> usize {
    world
        .find_ivec2('@')
        .iter()
        .filter(|&pos| count_around(world, pos, '@') < 4)
        .count()
}

fn main() {
    let input = include_str!("../input.txt");
    let world = World::new(input);
    println!("Result: {}", solve(&world));
}

#[cfg(test)]
mod tests {
    use glam::ivec2;
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

        let world = World::new(input);
        assert_eq!(solve(&world), 13)
    }

    #[test]
    fn test_example_1() {
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

        let world = World::new(input);
        assert_eq!(count_around(&world, &ivec2(0, 0), '@'), 2);
        assert_eq!(count_around(&world, &ivec2(2, 0), '@'), 3);
        assert_eq!(count_around(&world, &ivec2(4, 4), '@'), 8)
    }
}
