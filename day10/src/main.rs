#![feature(strip_circumfix)]

use good_lp::{Expression, ProblemVariables, SolverModel, constraint, default_solver, variable, variables, Solution, Variable};
use itertools::Itertools;
use std::str::FromStr;

struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<u32>>,
    joltages: Vec<u32>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseMachineError;

impl Machine {
    fn pack_buttons(&self) -> Vec<u64> {
        self.buttons
            .iter()
            .map(|button| pack_from_indices(button))
            .collect()
    }

    fn pack_target(&self) -> u64 {
        pack_bools(&self.lights)
    }
}
impl FromStr for Machine {
    type Err = ParseMachineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace().peekable();

        let lights = parts
            .next()
            .ok_or(ParseMachineError)?
            .strip_circumfix("[", "]")
            .ok_or(ParseMachineError)?
            .chars()
            .map(|c| match c {
                '.' => Ok(false),
                '#' => Ok(true),
                _ => Err(ParseMachineError),
            })
            .collect::<Result<Vec<_>, _>>()?;

        let joltages = parts
            .next_back()
            .ok_or(ParseMachineError)?
            .strip_circumfix('{', '}')
            .ok_or(ParseMachineError)?
            .split(',')
            .map(|c| c.parse::<_>().map_err(|_| ParseMachineError))
            .collect::<Result<Vec<_>, _>>()?;

        let buttons = parts
            .map(|part| {
                part.strip_circumfix('(', ')')
                    .ok_or(ParseMachineError)?
                    .split(',')
                    .map(|n| n.parse::<_>().map_err(|_| ParseMachineError))
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            lights,
            buttons,
            joltages,
        })
    }
}

fn pack_bools(bools: &[bool]) -> u64 {
    bools
        .iter()
        .enumerate()
        .fold(0u64, |acc, (i, &b)| acc | ((b as u64) << i))
}

fn pack_from_indices(indices: &[u32]) -> u64 {
    indices.iter().fold(0u64, |acc, &i| acc | (1 << i))
}

fn parse(input: &str) -> Vec<Machine> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn calc_presses(machine: &Machine) -> usize {
    let target = machine.pack_target();
    let buttons = machine.pack_buttons();
    let button_indices = (0..buttons.len()).collect::<Vec<_>>();

    for i in 1..buttons.len() {
        for idxes in button_indices.iter().combinations(i) {
            let result = idxes.iter().fold(0u64, |acc, &&idx| acc ^ buttons[idx]);

            if result == target {
                return i;
            }
        }
    }

    unreachable!()
}

fn solve_pt1(input: &[Machine]) -> usize {
    input.iter().map(calc_presses).sum()
}

fn calc_presses_pt2(input: &Machine) -> usize {
    // Create variables for the amount of button presses
    let mut problem_vars = ProblemVariables::new();
    let vars: Vec<_> = input
        .buttons
        .iter()
        .map(|button| problem_vars.add(variable().integer().min(0)))
        .collect();

    // Set up target objective
    let objective: Expression = vars.iter().sum();

    // Set up constraints
    let mut constraints = input
        .joltages
        .iter()
        .enumerate()
        .map(|(pos, &joltage)| {
            constraint!(
                input
                    .buttons
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, button)| {
                        if button.contains(&(pos as u32)) {
                            return Some(vars[idx]);
                        }
                        None
                    })
                    .sum::<Expression>()
                    == joltage
            )
        })
        .collect::<Vec<_>>();

    let mut problem = problem_vars.clone().minimise(objective).using(default_solver);
    for constraint in constraints {
        problem = problem.with(constraint);
    }

    let result = problem.solve().expect("Failed to solve");

    vars.iter().map(|&v| result.value(v).round() as usize).sum()
}

fn solve_pt2(input: &[Machine]) -> usize {
    input.iter().map(calc_presses_pt2).sum()
}

fn main() {
    let input = include_str!("input.txt");
    let machines = parse(input);

    println!("Part 1: {}", solve_pt1(&machines));
    println!("Part 2: {}", solve_pt2(&machines));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_machine() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let machine = Machine::from_str(input).unwrap();
        assert_eq!(machine.lights, vec![false, true, true, false]);
        assert_eq!(
            machine.buttons,
            vec![
                vec![3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![0, 2],
                vec![0, 1],
            ]
        );
        assert_eq!(machine.joltages, vec![3, 5, 4, 7]);
    }

    #[test]
    fn test_example() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

        let machines = parse(input);
        assert_eq!(solve_pt1(&machines), 7);
        assert_eq!(solve_pt2(&machines), 33);
    }
}
