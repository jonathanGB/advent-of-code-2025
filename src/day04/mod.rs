use std::{println, unreachable};

use crate::solver::Solver;
use crate::utils::_display_map;

#[derive(PartialEq)]
enum Tile {
    Roll,
    Empty,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '@' => Self::Roll,
            '.' => Self::Empty,
            _ => unreachable!(),
        }
    }
}

impl From<&Tile> for char {
    fn from(value: &Tile) -> Self {
        match *value {
            Tile::Roll => '@',
            Tile::Empty => '.',
        }
    }
}

impl Tile {
    fn is_empty(&self) -> bool {
        *self == Self::Empty
    }

    fn is_roll(&self) -> bool {
        *self == Self::Roll
    }
}

struct Grid {
    pub map: Vec<Vec<Tile>>,
    size: usize,
}

impl Grid {
    fn new(file: &str) -> Self {
        let map: Vec<_> = file
            .lines()
            .map(|line| line.chars().map(Tile::from).collect())
            .collect();
        let size = map.len();
        Self { map, size }
    }

    fn identify_accessible_rolls(&mut self, remove_accessible_roll: bool) -> u16 {
        let mut accessible_rolls = 0;

        for i in 0..self.size {
            for j in 0..self.size {
                if self.map[i][j].is_empty() {
                    continue;
                }

                let has_adjacent_up = i > 0;
                let has_adjacent_right = j < self.size - 1;
                let has_adjacent_down = i < self.size - 1;
                let has_adjacent_left = j > 0;

                let mut adjacent_rolls = 0;
                if has_adjacent_up {
                    // Top-left.
                    if has_adjacent_left {
                        if self.map[i - 1][j - 1].is_roll() {
                            adjacent_rolls += 1;
                        }
                    }

                    // Top.
                    if self.map[i - 1][j].is_roll() {
                        adjacent_rolls += 1;
                    }

                    // Top-right.
                    if has_adjacent_right {
                        if self.map[i - 1][j + 1].is_roll() {
                            adjacent_rolls += 1;
                        }
                    }
                }

                // Left.
                if has_adjacent_left {
                    if self.map[i][j - 1].is_roll() {
                        adjacent_rolls += 1;
                    }
                }

                // Right.
                if has_adjacent_right {
                    if self.map[i][j + 1].is_roll() {
                        adjacent_rolls += 1;
                    }
                }

                if has_adjacent_down {
                    // Down-left.
                    if has_adjacent_left {
                        if self.map[i + 1][j - 1].is_roll() {
                            adjacent_rolls += 1;
                        }
                    }

                    // Down.
                    if self.map[i + 1][j].is_roll() {
                        adjacent_rolls += 1;
                    }

                    // Down-right.
                    if has_adjacent_right {
                        if self.map[i + 1][j + 1].is_roll() {
                            adjacent_rolls += 1;
                        }
                    }
                }

                if adjacent_rolls < 4 {
                    accessible_rolls += 1;

                    if remove_accessible_roll {
                        self.map[i][j] = Tile::Empty;
                    }
                }
            }
        }

        accessible_rolls
    }
}

pub struct SolverImpl {}

impl Solver for SolverImpl {
    fn solve_part1(file: &str) {
        let mut grid = Grid::new(file);
        println!("{}", _display_map(&grid.map));
        println!(
            "There are {} accessible rolls.",
            grid.identify_accessible_rolls(false)
        );
    }

    fn solve_part2(file: &str) {
        let mut grid = Grid::new(file);
        let mut total_removed_rolls = 0;

        loop {
            println!("{}", _display_map(&grid.map));
            let num_removed_rolls = grid.identify_accessible_rolls(true);
            total_removed_rolls += num_removed_rolls;
            println!("There are {} accessible rolls.", num_removed_rolls);

            if num_removed_rolls == 0 {
                break;
            }
        }

        println!("In total, {total_removed_rolls} rolls were removed.");
    }
}
