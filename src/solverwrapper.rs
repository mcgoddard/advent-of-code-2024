use advent_of_code_2024::SolverTrait;
use crate::day_1;
use crate::day_2;
use crate::day_3;
use crate::day_4;
use crate::day_5;
use crate::day_6;
use crate::day_7;

pub struct SolverWrapper {
  pub solver: Box<dyn SolverTrait>,
}

impl SolverWrapper {
  pub fn new(day: u8) -> Self {
    let solver: Box<dyn SolverTrait> = match day {
      1 => Box::new(day_1::Solver),
      2 => Box::new(day_2::Solver),
      3 => Box::new(day_3::Solver),
      4 => Box::new(day_4::Solver),
      5 => Box::new(day_5::Solver),
      6 => Box::new(day_6::Solver),
      7 => Box::new(day_7::Solver),
      _ => panic!("Invalid day {}", day),
    };

    Self { solver }
  }

  pub fn solve(&self, part: u8, lines: Vec<String>) -> i64 {
    match part {
      1 => self.solver.part_1(lines),
      2 => self.solver.part_2(lines),
      _ => panic!("Invalid part {}", part),
    }
  }
}
