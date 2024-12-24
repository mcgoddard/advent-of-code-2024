use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Operator {
  And,
  Or,
  Xor,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Gate {
  pub input1: String,
  pub input2: String,
  pub output: String,
  pub operator: Operator,
}

pub fn parse_input(lines: &[String]) -> (HashMap<String, u8>, Vec<Gate>) {
let mut initial = true;
  let mut wires = HashMap::new();
  let mut unsolved_gates = vec![];
  for line in lines {
    if line.is_empty() {
      initial = false;
      continue;
    }
    if initial {
      let parts = line.split(": ").collect::<Vec<&str>>();
      wires.insert(parts[0].to_string(), parts[1].parse::<u8>().unwrap());
    } else {
      let parts = line.split(" ").collect::<Vec<&str>>();
      unsolved_gates.push(Gate{
        input1: parts[0].to_string(),
        input2: parts[2].to_string(),
        output: parts[4].to_string(),
        operator: match parts[1] {
          "XOR" => Operator::Xor,
          "AND" => Operator::And,
          "OR" => Operator::Or,
          o => panic!("Invalid operator {}", o),
        }
      })
    }
  }
  (wires, unsolved_gates)
}
