use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(String, u64)> {
    input
        .lines()
        .map(|l| {
            let mut p = l.trim().split(' ').map(String::from);
            (p.next().unwrap(), p.next().unwrap().parse().unwrap())
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part_1(input: &[(String, u64)]) -> u64 {
    #[inline(always)]
    fn compute((horizontal, depth): (u64, u64), (direction, step): &(String, u64)) -> (u64, u64) {
        match direction.as_str() {
            "forward" => (horizontal + step, depth),
            "down" => (horizontal, depth + step),
            "up" => (horizontal, depth - step),
            _ => panic!("Invalid direction: {}", direction),
        }
    }

    let result = input.into_iter().fold((0, 0), compute);

    result.0 * result.1
}

#[aoc(day2, part2)]
pub fn solve_part_2(input: &[(String, u64)]) -> u64 {
    #[inline(always)]
    fn compute(
        (horizontal, depth, aim): (u64, u64, u64),
        (direction, step): &(String, u64),
    ) -> (u64, u64, u64) {
        match direction.as_str() {
            "down" => (horizontal, depth, aim + step),
            "up" => (horizontal, depth, aim - step),
            "forward" => (horizontal + step, depth + aim * step, aim),
            _ => panic!("Invalid direction: {}", direction),
        }
    }

    let result = input.into_iter().fold((0, 0, 0), compute);

    result.0 * result.1
}
