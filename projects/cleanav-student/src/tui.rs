use std::{
    num::ParseIntError,
    path::{self, Path},
};

use clap::Parser;

pub fn read_map(map_path: impl AsRef<Path>) -> String {
    std::fs::read_to_string(map_path).unwrap()
}

pub fn output(output: &str) -> () {
    println!("{}", output);
}

fn parse_slope(slope: &str) -> Result<(usize, usize), ParseIntError> {
    let (right, down) = slope.split_once(",").unwrap();
    Ok((right.parse::<usize>()?, down.parse::<usize>()?))
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Opt {
    #[arg(short, long, value_parser = parse_slope)]
    pub slope: (usize, usize),
    #[arg(short, long)]
    pub termination: String,
    #[arg(short, long)]
    pub map: path::PathBuf,
    #[arg(short, long)]
    pub output: String,
}

pub fn parse_args() -> Opt {
    Opt::parse()
}
