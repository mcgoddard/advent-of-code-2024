use itertools::Itertools;

use super::lib::{parse_input, Operator};

pub fn part1(lines: &[String]) -> String {
  let (mut wires, mut unsolved_gates) = parse_input(lines);
  while !unsolved_gates.is_empty() {
    let gate = unsolved_gates.remove(0);
    if let Some(input1) = wires.get(&gate.input1) {
      if let Some(input2) = wires.get(&gate.input2) {
        let result = match gate.operator {
          Operator::And => input1 & input2,
          Operator::Or => input1 | input2,
          Operator::Xor => input1 ^ input2,
        };
        wires.insert(gate.output, result);
      } else {
        unsolved_gates.push(gate);
      }
    } else {
      unsolved_gates.push(gate);
    }
  }
  let result_string = wires.iter().filter(|(k, _)| k.starts_with("z"))
    .sorted_by(|(k1, _), (k2, _)| k2.cmp(k1)).map(|(_, v)| v.to_string()).join("");
  i64::from_str_radix(&result_string, 2).unwrap().to_string()
}
