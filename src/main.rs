use anyhow::Result;
use clap::Parser;

mod day1;
mod day2;

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
        1 => day1::run(args.part),
        2 => day2::run(args.part),
        _ => day2::run(args.part),
    }
}
