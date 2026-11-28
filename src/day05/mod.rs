use std::{
    ops::Bound::{Excluded, Included, Unbounded},
    println,
    str::Lines,
    unreachable,
};

use crate::solver::Solver;
// I wrote that library so I feel like I'm allowed to use it!!!
use unbounded_interval_tree::interval_tree::IntervalTree;

struct Database {
    fresh_id_ranges: IntervalTree<u64>,
}

impl Database {
    fn new(lines: &mut Lines) -> Self {
        let mut fresh_id_ranges = IntervalTree::default();
        for line in lines {
            if line.is_empty() {
                break;
            }

            let (range_start, range_end) = line.split_once('-').unwrap();
            let range = range_start.parse::<u64>().unwrap()..=range_end.parse::<u64>().unwrap();
            fresh_id_ranges.insert(range);
        }

        Self { fresh_id_ranges }
    }

    fn count_non_overlapping_ids(lines: &mut Lines) -> u64 {
        let mut num_non_overlapping_ids = 0;
        let mut non_overlapping_id_ranges = IntervalTree::default();
        for line in lines {
            if line.is_empty() {
                break;
            }

            let (range_start, range_end) = line.split_once('-').unwrap();
            let range = range_start.parse::<u64>().unwrap()..=range_end.parse::<u64>().unwrap();
            let mut non_overlapping_id_ranges_to_insert = Vec::new();
            for (non_overlapping_range_start, non_overlapping_range_end) in
                non_overlapping_id_ranges.get_interval_difference(&range)
            {
                let inclusive_non_overlapping_range_start = match non_overlapping_range_start {
                    Included(start) => *start,
                    Excluded(start) => *start + 1, // Integer inclusive bound starts at the next integer.
                    Unbounded => unreachable!(
                        "All ranges in the database are inclusive bounds, so the difference cannot be unbounded."
                    ),
                };
                let inclusive_non_overlapping_range_end = match non_overlapping_range_end {
                    Included(end) => *end,
                    Excluded(end) => *end - 1, // Integer inclusive bound ends at the previous integer.
                    Unbounded => unreachable!(
                        "All ranges in the database are inclusive bounds, so the difference cannot be unbounded."
                    ),
                };

                // The range covers (N:N+1), i.e. both bounds are excluded, which means no integer values are included.
                if inclusive_non_overlapping_range_start > inclusive_non_overlapping_range_end {
                    continue;
                }

                num_non_overlapping_ids +=
                    inclusive_non_overlapping_range_end - inclusive_non_overlapping_range_start + 1;
                non_overlapping_id_ranges_to_insert.push(
                    inclusive_non_overlapping_range_start..=inclusive_non_overlapping_range_end,
                );
            }

            for non_overlapping_id_range_to_insert in non_overlapping_id_ranges_to_insert {
                non_overlapping_id_ranges.insert(non_overlapping_id_range_to_insert);
            }
        }

        num_non_overlapping_ids
    }

    fn query_available_ingredients(&self, lines: &mut Lines) -> usize {
        let mut num_fresh_available_ingredients = 0;

        for line in lines {
            let available_ingredient = line.parse().unwrap();
            if self.fresh_id_ranges.contains_point(&available_ingredient) {
                num_fresh_available_ingredients += 1;
            }
        }

        num_fresh_available_ingredients
    }
}

pub struct SolverImpl {}

impl Solver for SolverImpl {
    fn solve_part1(file: &str) {
        let mut lines = file.lines();
        let fresh_ingredients_database = Database::new(&mut lines);
        let num_fresh_available_ingredients =
            fresh_ingredients_database.query_available_ingredients(&mut lines);
        println!(
            "In this database, there are {num_fresh_available_ingredients} fresh available ingredients."
        );
    }

    fn solve_part2(file: &str) {
        let mut lines = file.lines();
        let num_non_overlapping_fresh_ids = Database::count_non_overlapping_ids(&mut lines);
        println!(
            "In this database, there are {num_non_overlapping_fresh_ids} fresh unique available ingredients."
        );
    }
}
