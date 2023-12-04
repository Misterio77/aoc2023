use aho_corasick::AhoCorasick;
use anyhow::Result;
use maplit::hashmap as map;
use std::collections::HashMap as Map;
use std::io;

fn replace(input: &str, mapping: &Map<&str, &str>) -> Result<String> {
    let values: Vec<&str> = mapping.values().copied().collect();
    let ac = AhoCorasick::new(mapping.keys())?;
    let mut wtr = vec![];
    ac.try_stream_replace_all(input.as_bytes(), &mut wtr, &values)?;
    let output = String::from_utf8(wtr)?;
    Ok(output)
}

fn main() -> Result<()> {
    let sum: u32 = io::stdin()
        .lines()
        // Filter only valid lines
        .filter_map(|l| l.ok())
        // Replace spelled out numbers with actual numbers
        .filter_map(|l| {
            replace(
                &l,
                &map! {
                    "zero"=> "0",
                    "one"=> "1",
                    "two"=> "2",
                    "three"=> "3",
                    "four"=> "4",
                    "five"=> "5",
                    "six"=> "6",
                    "seven"=> "7",
                    "eight"=> "8",
                    "nine"=> "9"
                },
            )
            .ok()
        })
        // Parse each string into a list of numbers
        .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect())
        // Get first and last number, concatenate them
        .map(|n: Vec<_>| {
            let first = n.first().unwrap();
            let last = n.last().unwrap_or(first);
            dbg!(first)*10 + dbg!(last)
        })
        .sum();

    println!("{sum}");
    Ok(())
}
