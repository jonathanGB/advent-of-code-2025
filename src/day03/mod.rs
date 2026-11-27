use crate::solver::Solver;

struct Bank<'a> {
    batteries: &'a [u8],
}

impl<'a> Bank<'a> {
    fn largest_joltage(&self, num_batteries: usize) -> u64 {
        let mut joltage = 0;
        let mut batteries_subset = self.batteries;

        for num_batteries_left in (0..num_batteries).rev() {
            // We have to stop looking for a leading battery before this point, otherwise there won't be enough
            // batteries left in the next iteration.
            let batteries_subset_end = batteries_subset.len() - num_batteries_left;
            let mut max_leading_battery = 0;
            let mut max_leading_battery_index = 0;

            for (i, battery) in batteries_subset[..batteries_subset_end].iter().enumerate() {
                if *battery > max_leading_battery {
                    max_leading_battery = *battery;
                    max_leading_battery_index = i;
                }
            }

            // We know that the byte representations are integers (between b'0' and b'9' inclusively).
            // We multiply by a factor of 10 to the power of the position of the digit, e.g. 100s, 10s, 1s, etc.
            joltage += (max_leading_battery - b'0') as u64 * 10_u64.pow(num_batteries_left as u32);
            // The remainder of the batteries to look must be after the previous leading battery in the bank.
            batteries_subset = &batteries_subset[max_leading_battery_index + 1..];
        }

        joltage
    }
}

pub struct SolverImpl {}

impl SolverImpl {
    fn iter_banks(file: &str) -> impl Iterator<Item = Bank<'_>> {
        file.lines().map(|line| Bank {
            batteries: line.as_bytes(),
        })
    }
}

impl Solver for SolverImpl {
    fn solve_part1(file: &str) {
        let max_joltage_sum: u64 = Self::iter_banks(file)
            .map(|bank| bank.largest_joltage(2))
            .sum();

        println!("The max 2-joltage sum across all banks is {max_joltage_sum}");
    }

    fn solve_part2(file: &str) {
        let max_joltage_sum: u64 = Self::iter_banks(file)
            .map(|bank| bank.largest_joltage(12))
            .sum();

        println!("The max 12-joltage sum across all banks is {max_joltage_sum}");
    }
}
