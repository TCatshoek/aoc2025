use glam::{I64Vec2, i64vec2};
use itertools::Itertools;

fn parse(input: &str) -> Vec<I64Vec2> {
    input
        .lines()
        .map(|line| {
            let (x, y): (i64, i64) = line
                .split(',')
                .map(|el| el.parse().unwrap())
                .collect_tuple()
                .unwrap();
            I64Vec2::new(x, y)
        })
        .collect()
}

fn area(a: &I64Vec2, b: &I64Vec2) -> usize {
    (((a.x - b.x).abs() + 1) * ((a.y - b.y).abs() + 1)) as usize
}

fn rectangle(a: &I64Vec2, b: &I64Vec2) -> Vec<I64Vec2> {
    vec![*a, i64vec2(b.x, a.y), *b, i64vec2(a.x, b.y)]
}

fn solve_pt1(input: &str) -> usize {
    parse(input)
        .iter()
        .combinations(2)
        .map(|pair| area(pair[0], pair[1]))
        .max()
        .unwrap()
}

fn cross(o: &I64Vec2, a: &I64Vec2, b: &I64Vec2) -> i64 {
    (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x)
}

fn segments_intersect(a: &I64Vec2, b: &I64Vec2, c: &I64Vec2, d: &I64Vec2) -> bool {
    let d1 = cross(c, d, a);
    let d2 = cross(c, d, b);
    let d3 = cross(a, b, c);
    let d4 = cross(a, b, d);

    // Check if points are on opposite sides
    if ((d1 > 0 && d2 < 0) || (d1 < 0 && d2 > 0)) && ((d3 > 0 && d4 < 0) || (d3 < 0 && d4 > 0)) {
        return true;
    }

    false
}

fn check_intersections(
    segments_a: &[(&I64Vec2, &I64Vec2)],
    segments_b: &[(&I64Vec2, &I64Vec2)],
) -> bool {
    segments_a
        .iter()
        .cartesian_product(segments_b)
        .map(|(a, b)| {
            segments_intersect(&a.0, &a.1, &b.0, &b.1)
        })
        .all(|x| x == false)
}

fn point_in_polygon(point: &I64Vec2, polygon: &[I64Vec2]) -> bool {
    let mut inside = false;
    let n = polygon.len();
    let mut j = n - 1;

    for i in 0..n {
        let vi = &polygon[i];
        let vj = &polygon[j];

        if ((vi.y > point.y) != (vj.y > point.y))
            && (point.x < (vj.x - vi.x) * (point.y - vi.y) / (vj.y - vi.y) + vi.x)
        {
            inside = !inside;
        }
        j = i;
    }
    inside
}

fn solve_pt2(input: &str) -> usize {
    let points = parse(input);
    let segments: Vec<(&I64Vec2, &I64Vec2)> = points.iter().circular_tuple_windows().collect();

    points
        .iter()
        .combinations(2)
        .map(|pair| rectangle(pair[0], pair[1]))
        .filter(|rectangle_points| {
            let corners_inside = rectangle_points.iter().all(|p|
                points.contains(p) || point_in_polygon(p, &points)
            );

            let rectangle_segments: Vec<(&I64Vec2, &I64Vec2)> =
                rectangle_points.iter().circular_tuple_windows().collect();

            corners_inside && check_intersections(&segments, &rectangle_segments)
        })
        .map(|rectangle_points| {
            area(&rectangle_points[0], &rectangle_points[2])
        })
        .max().unwrap()
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", solve_pt1(input));
    println!("Part 2: {}", solve_pt2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        assert_eq!(solve_pt1(input), 50);
        assert_eq!(solve_pt2(input), 24);
    }
}
