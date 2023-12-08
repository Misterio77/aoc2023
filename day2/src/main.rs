use anyhow::{anyhow, Context, Error, Result};
use std::format as f;
use std::{io, str::FromStr};

#[derive(Default, Debug)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

impl Cubes {
    fn rgb(red: u32, green: u32, blue: u32) -> Cubes {
        Cubes { red, green, blue }
    }
    fn contains(&self, other: &Cubes) -> bool {
        (self.green >= other.green) && (self.red >= other.red) && (self.blue >= other.blue)
    }
    fn power(&self) -> u32 {
        self.green * self.red * self.blue
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Cubes>,
}

impl Game {
    fn is_possible(&self, bag: &Cubes) -> bool {
        self.rounds.iter().all(|round| bag.contains(round))
    }
    fn minimum_bag(&self) -> Cubes {
        Cubes {
            green: self.rounds.iter().map(|b| b.green).max().unwrap_or(0),
            red: self.rounds.iter().map(|b| b.red).max().unwrap_or(0),
            blue: self.rounds.iter().map(|b| b.blue).max().unwrap_or(0),
        }
    }
}
impl FromStr for Game {
    type Err = Error;
    fn from_str(input: &str) -> Result<Game> {
        let mut tokens = input.split(":");
        // First value is game id
        let id = tokens
            .next()
            .context("No game ID found")?
            .split_whitespace()
            .nth(1)
            .context("No game ID found")?
            .parse()?;
        // Second is rounds
        let rounds = tokens
            .next()
            .context(f!("No rounds found in game {id}"))?
            // For every round, extract a set of Cubes
            .split(";")
            .map(|round| {
                let mut cubes = Cubes::default();
                for pair in round.split(",") {
                    let (qty, color) = {
                        let mut iter = pair.split_whitespace();
                        (
                            iter.next().context("No qty found")?.parse()?,
                            iter.next().context("No color found")?,
                        )
                    };
                    match color {
                        "blue" => cubes.blue = qty,
                        "red" => cubes.red = qty,
                        "green" => cubes.green = qty,
                        x => return Err(anyhow!("Unknown color in game {id}: {x}")),
                    }
                }
                Ok(cubes)
            })
            .collect::<Result<_>>()?;
        Ok(Game { id, rounds })
    }
}

fn part1(games: &[Game]) -> u32 {
    let bag = Cubes::rgb(12, 13, 14);
    let possible_games = games.iter().filter(|g| g.is_possible(&bag));
    possible_games.map(|g| g.id).sum()
}

fn part2(games: &[Game]) -> u32 {
    games.iter().map(|g| g.minimum_bag().power()).sum()
}

fn main() -> Result<()> {
    let input = io::read_to_string(io::stdin())?;
    let games: Vec<Game> = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<_>>()?;

    println!("Part 1: {}", part1(&games));
    println!("Part 2: {}", part2(&games));
    Ok(())
}
