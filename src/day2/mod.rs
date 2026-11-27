use crate::{solver::Solver, utils::generate_benchmark};
use std::{ops::RangeInclusive, println, unreachable, vec};

struct IdRange {
    range: RangeInclusive<u64>,
    min_digits: u32,
}
pub struct SolverImpl {}

impl SolverImpl {
    fn output_id_ranges(file: &str) -> impl IntoIterator<Item = IdRange> {
        file.split(',').map(|range| {
            let (start, end) = range.split_once('-').expect("Range must look like A-B");
            IdRange {
                range: start.parse().unwrap()..=end.parse().unwrap(),
                min_digits: start.len() as u32,
            }
        })
    }

    fn find_num_digits(num: u64, min_digits_hint: u32) -> u32 {
        for num_digits in min_digits_hint.. {
            if num / 10u64.pow(num_digits) == 0 {
                return num_digits;
            }
        }

        unreachable!()
    }
}

impl Solver for SolverImpl {
    fn solve_part1(file: &str) {
        let mut total_invalid_ids = 0;
        let mut visited_ids = 0;

        for IdRange {
            range: id_range,
            min_digits,
        } in Self::output_id_ranges(file)
        {
            for potential_id in id_range {
                visited_ids += 1;

                let num_digits = Self::find_num_digits(potential_id, min_digits);
                if num_digits % 2 == 1 {
                    continue;
                }

                let half_divider = 10u64.pow(num_digits / 2);
                let (potential_id_left, potential_id_right) =
                    (potential_id / half_divider, potential_id % half_divider);
                if potential_id_left == potential_id_right {
                    total_invalid_ids += potential_id;
                }
            }
        }

        println!("Total invalid IDs: {total_invalid_ids} (visited {visited_ids})");
    }

    fn solve_part2(file: &str) {
        let mut total_invalid_ids = 0;
        let mut visited_ids = 0;

        let repeated_sequences_lengths_per_num_digits = [
            vec![],        // 0 ... must have at least two repeated sequences
            vec![],        // 1 ... must have at least two repeated sequences
            vec![1],       // 2
            vec![1],       // 3
            vec![1, 2],    // 4
            vec![1],       // 5
            vec![1, 2, 3], // 6
            vec![1],       // 7
            vec![1, 2, 4], // 8
            vec![1, 3],    // 9
            vec![1, 2, 5], // 10
        ];

        for IdRange {
            range: id_range,
            min_digits,
        } in Self::output_id_ranges(file)
        {
            for potential_id in id_range {
                let potential_id_str = potential_id.to_string();
                let potential_id_bytes = potential_id_str.as_bytes();
                visited_ids += 1;

                let num_digits = Self::find_num_digits(potential_id, min_digits) as usize;
                for repeated_sequence_length in
                    &repeated_sequences_lengths_per_num_digits[num_digits]
                {
                    let (first_sequence, rest_potential_id_bytes) =
                        potential_id_bytes.split_at(*repeated_sequence_length);

                    if rest_potential_id_bytes
                        .chunks(*repeated_sequence_length)
                        .all(|other_sequence| first_sequence == other_sequence)
                    {
                        total_invalid_ids += potential_id;
                        break;
                    }
                }
            }
        }

        println!("Total invalid IDs: {total_invalid_ids} (visited {visited_ids})");
    }
}
