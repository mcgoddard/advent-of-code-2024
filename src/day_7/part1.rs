use itertools::Itertools;

struct Equation {
  result: i64,
  values: Vec<i64>,
}

#[derive(Debug, Eq, PartialEq, Hash)]
enum Operators {
  Add,
  Multiply,
}

pub fn part1(lines: Vec<String>) -> i64 {
  let equations = lines.iter().map(|line| {
    let parts = line.split(": ").collect::<Vec<&str>>();
    Equation {
      result: parts[0].parse().unwrap(),
      values: parts[1].split(" ").map(|v| v.parse().unwrap()).collect(),
    }
  }).collect::<Vec<Equation>>();
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

fn variations_up_to_length<T>(items: &[T], length: usize) -> impl Iterator<Item = Vec<&T>> {
  std::iter::repeat(items.iter()).take(length).multi_cartesian_product()
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
