use super::lib::{parse_equations, variations_up_to_length, Equation};

#[derive(Debug, Eq, PartialEq, Hash)]
enum Operators {
  Add,
  Multiply,
}

pub fn part1(lines: Vec<String>) -> i64 {
  let equations = parse_equations(lines);
  let mut valid_results_sum = 0;
  let operators = vec![Operators::Add, Operators::Multiply];
  for equation in equations {
    let test_cases = variations_up_to_length(&operators, equation.values.len() - 1).collect::<Vec<Vec<&Operators>>>();
    for test_case in test_cases {
      if valid_test_case(&equation, &test_case) {
        valid_results_sum += equation.result;
        break;
      }
    }
  }
  valid_results_sum
}

fn valid_test_case(equation: &Equation, test_case: &[&Operators]) -> bool {
  let mut working_result = equation.values[0];
  for (i, operator) in test_case.iter().enumerate() {
    working_result = match operator {
      Operators::Add => working_result + equation.values[i + 1],
      Operators::Multiply => working_result * equation.values[i + 1],
    };
    if working_result > equation.result {
      return false;
    }
  }
  working_result == equation.result
}
