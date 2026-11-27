#![feature(test)]
extern crate test;

use clap::Parser;
use seq_macro::seq;

mod args;
mod day1;
mod day10;
mod day11;
mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod solver;
mod trie;
mod utils;

use args::{Args, Day};
use solver::Solver;

fn main() {
    let cli = Args::parse();

    seq!(N in 1..=12 {
        match cli.day {
            #(
                Day::Day~N {part, input} => {
                    let path = format!("src/day{}/{}.txt", N, input);
                    match std::fs::read_to_string(&path) {
                        Ok(file_content) => day~N::SolverImpl::solve(part, &file_content),
                        Err(e) => panic!("Could not read content of file {}, err: {}", path, e),
                    };
                },
            )*
        }
    });
}
