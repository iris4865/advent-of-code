use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part_1(input: &[u64]) -> u64 {
    input
        .iter()
        .zip(input.iter().skip(1))
        .filter(|(prev, now)| prev < now)
        .count() as u64
}

#[aoc(day1, part2)]
pub fn solve_part_2(input: &[u64]) -> u64 {
    let sums: Vec<u64> = input
        .iter()
        .zip(input.iter().skip(1))
        .zip(input.iter().skip(2))
        .map(|((a, b), c)| a + b + c)
        .collect();

    sums.iter()
        .zip(sums.iter().skip(1))
        .filter(|(prev, now)| prev < now)
        .count() as u64
}
