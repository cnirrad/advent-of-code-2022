use anyhow::Result;
use clap::Parser;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod utils;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(default_value_t = u8::MAX)]
    day: u8,

    #[clap(default_value_t = u8::MAX)]
    part: u8,
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.day {
        1 => day01::run(args.part),
        2 => day02::run(args.part),
        3 => day03::run(args.part),
        4 => day04::run(args.part),
        5 => day05::run(args.part),
        6 => day06::run(args.part),
        7 => day07::run(args.part),
        8 => day08::run(args.part),
        9 => day09::run(args.part),
        10 => day10::run(args.part),
        11 => day11::run(args.part),
        12 => day12::run(args.part),
        13 => day13::run(args.part),
        14 => day14::run(args.part),
        _ => day15::run(args.part),
    }
}
