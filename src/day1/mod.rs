use crate::{solver::Solver, utils::generate_benchmark};

#[derive(Debug)]
enum Rotation {
    Left,
    Right,
}

impl From<&str> for Rotation {
    fn from(value: &str) -> Self {
        match value {
            "L" => Self::Left,
            "R" => Self::Right,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    rotation: Rotation,
    num: i16,
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let (rotation, num) = value.split_at(1);
        let (rotation, num) = (
            rotation.into(),
            num.parse().expect("Should parse into an int"),
        );
        Instruction { rotation, num }
    }
}

struct DialSolver {
    dial: i16,
}

impl Default for DialSolver {
    fn default() -> Self {
        Self { dial: 50 }
    }
}

impl DialSolver {
    fn count_terminal_zeros(
        &mut self,
        instructions: impl IntoIterator<Item = Instruction>,
    ) -> usize {
        let mut zero_dial = 0;

        for Instruction { rotation, mut num } in instructions {
            // Prevent overflowing by limiting to at most one full rotation.
            num %= 100;

            let dial = match rotation {
                Rotation::Left => self.dial - num,
                Rotation::Right => self.dial + num,
            };

            self.dial = dial.rem_euclid(100);

            if self.dial == 0 {
                zero_dial += 1;
            }
        }

        zero_dial
    }

    fn count_intermediary_zeros(
        &mut self,
        instructions: impl IntoIterator<Item = Instruction>,
    ) -> usize {
        let mut zero_dial = 0;

        for Instruction { rotation, mut num } in instructions {
            let initial_dial = self.dial;

            // Count number of full rotations.
            zero_dial += (num / 100) as usize;

            // Prevent overflowing by limiting to at most one full rotation.
            num %= 100;

            // Either we don't move, or we move exactly N full rotations.
            // These N full rotations are already accounted above.
            if num == 0 {
                continue;
            }

            match rotation {
                Rotation::Left => self.dial -= num,
                Rotation::Right => self.dial += num,
            };

            // If we land between 1 (inclusively) and 100 (exclusively),
            // then we never crossed or stopped at zero during this instruction.
            if (1..100).contains(&self.dial) {
                continue;
            }

            // Don't account a rotation that started from zero itself.
            if initial_dial != 0 {
                zero_dial += 1;
            }

            self.dial = self.dial.rem_euclid(100);
        }

        zero_dial
    }
}

#[derive(Debug)]
pub struct SolverImpl {}

impl Solver for SolverImpl {
    fn solve_part1(file: &str) {
        let mut dial_solver = DialSolver::default();
        let zero_dials = dial_solver.count_terminal_zeros(file.lines().map(Instruction::from));
        println!("{zero_dials}");
    }

    fn solve_part2(file: &str) {
        let mut dial_solver = DialSolver::default();
        let zero_dials = dial_solver.count_intermediary_zeros(file.lines().map(Instruction::from));
        println!("{zero_dials}");
    }
}

generate_benchmark!(day1);
