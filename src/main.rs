#![feature(test)]
extern crate test;

use clap::Parser;
use seq_macro::seq;

mod args;
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
mod solver;
mod trie;
mod utils;

use args::{Args, Day};
use solver::Solver;

fn main() {
    let cli = Args::parse();

    seq!(N in 01..=12 {
        match cli.day {
            #(
                Day::Day~N {part, input} => {
                    let path = format!("src/day{:02}/{}.txt", N, input);
                    match std::fs::read_to_string(&path) {
                        Ok(file_content) => day~N::SolverImpl::solve(part, &file_content),
                        Err(e) => panic!("Could not read content of file {}, err: {}", path, e),
                    };
                },
            )*
        }
    });
}
