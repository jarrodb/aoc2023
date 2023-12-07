use std::io;
use clap::{Parser, Subcommand};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

#[derive(Parser)]
#[command(about="Advent of Code 2023")]
struct Cli {
    #[command(subcommand)]
    pub subcommands: Option<SubCommand>,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    Day1,
    Day2,
    Day3,
    Day4,
    Day5,
    Day6,
    Day7,
}

fn read_from_stdin() -> Vec<String> {
    let mut data = Vec::<String>::new();

    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).unwrap() > 0 {
        data.push(buffer.clone());
        buffer.clear();
    }
    return data;
}

fn main() {
    let cli = Cli::parse();
    match cli.subcommands {
        Some(SubCommand::Day1) => {
            let data = read_from_stdin();
            println!("part1: {}", day1::part1(&data));
            println!("part2: {}", day1::part2(&data));
        },
        Some(SubCommand::Day2) => {
            let data = read_from_stdin();
            println!("part1: {}", day2::part1(&data));
            println!("part2: {}", day2::part2(&data));
        },
        Some(SubCommand::Day3) => {
            let data = read_from_stdin();
            println!("part1: {}", day3::part1(&data));
            println!("part2: {}", day3::part2(&data));
        },
        Some(SubCommand::Day4) => {
            let data = read_from_stdin();
            println!("part1: {}", day4::part1(&data));
            println!("part2: {}", day4::part2(&data));
        },
        Some(SubCommand::Day5) => {
            let data = read_from_stdin();
            println!("part1: {}", day5::part1(&data));
            println!("part2: {}", day5::part2(&data));
        },
        Some(SubCommand::Day6) => {
            println!("part1: {}", day6::part1(day6::Data::Real));
            println!("part2: {}", day6::part2(day6::Data::Real));
        },
        Some(SubCommand::Day7) => {
            let data = read_from_stdin();
            println!("part1: {}", day7::part1(&data));
            println!("part2: {}", day7::part2(&data));
        },
        _ => {
            println!("no subcommand given");
        },
    }
}
