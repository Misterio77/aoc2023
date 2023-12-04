use anyhow::Result;
use std::io;

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let f = l.chars().find_map(|c| c.to_digit(10)).unwrap_or(0);
            let l = l.chars().rev().find_map(|c| c.to_digit(10)).unwrap_or(f);
            f * 10 + l
        })
        .sum()
}

fn convert_num(name: &str) -> Option<u32> {
    if name.starts_with("zero") {
        Some(0)
    } else if name.starts_with("one") {
        Some(1)
    } else if name.starts_with("two") {
        Some(2)
    } else if name.starts_with("three") {
        Some(3)
    } else if name.starts_with("four") {
        Some(4)
    } else if name.starts_with("five") {
        Some(5)
    } else if name.starts_with("six") {
        Some(6)
    } else if name.starts_with("seven") {
        Some(7)
    } else if name.starts_with("eight") {
        Some(8)
    } else if name.starts_with("nine") {
        Some(9)
    } else {
        name.chars().nth(0)?.to_digit(10)
    }
}

fn find_num(line: &str, reverse: bool) -> Option<u32> {
    let chars: Vec<char> = if reverse {
        line.chars().rev().collect()
    } else {
        line.chars().collect()
    };
    let index = |i: usize| if reverse { chars.len() - i - 1 } else { i };
    chars
        .iter()
        .enumerate()
        .find_map(|(i, _)| convert_num(&line[index(i)..]))
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let first = find_num(l, false).unwrap_or(0);
            let last = find_num(l, true).unwrap_or(first);
            dbg!(10 * first + last)
        })
        .sum()
}

fn main() -> Result<()> {
    let input = io::read_to_string(io::stdin())?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
