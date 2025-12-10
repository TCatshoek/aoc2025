use day08::unionfind::UnionFind;
use glam::Vec3;
use itertools::Itertools;
fn parse(input: &str) -> Vec<Vec3> {
    input
        .lines()
        .map(|line| {
            let (x, y, z): (i32, i32, i32) = line
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap();
            Vec3::new(x as f32, y as f32, z as f32)
        })
        .collect()
}

fn calc_distances(positions: &[Vec3]) -> Vec<(usize, usize, f32)> {
    positions
        .iter()
        .enumerate()
        .combinations(2)
        .map(|pair| {
            let (i, a) = pair[0];
            let (j, b) = pair[1];
            (i, j, a.distance(*b))
        })
        .collect()
}

fn solve(input: &str, n_connections: usize) -> usize {
    let positions = parse(input);
    let mut distances = calc_distances(&positions);
    distances.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    let mut circuits = UnionFind::new(positions.len());

    for dist in distances[0..n_connections].iter() {
        circuits.union(dist.0, dist.1);
    }

    let counts: Vec<usize> = (0..positions.len())
        .map(|i| circuits.find(i))
        .counts()
        .into_iter()
        .sorted_by(|a, b| b.1.cmp(&a.1))
        .map(|(_, count)| count)
        .collect();

    counts[0..3].iter().product()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", solve(input, 1000));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        assert_eq!(solve(input, 10), 40);
    }
}
