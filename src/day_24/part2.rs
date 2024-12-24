use super::lib::{parse_input, Gate, Operator};

pub fn part2(lines: &[String]) -> String {
  let (_, mut unsolved_gates) = parse_input(lines);
  let mut current_carry_wire = None;
  let mut bit = 0;
  let z_wires = unsolved_gates.iter().filter(|g| g.output.starts_with("z")).count() - 1;
  let mut swaps = vec![];
  loop {
    let x_wire = format!("x{:0>2}", bit);
    let y_wire = format!("y{:0>2}", bit);
    let z_wire = format!("z{:0>2}", bit);
    if bit == 0 {
      current_carry_wire = Some(find_output(&x_wire, &y_wire, Operator::And, &unsolved_gates).unwrap());
    } else {
      let ab_xor_gate = find_output(&x_wire, &y_wire, Operator::Xor, &unsolved_gates).unwrap();
      let ab_and_gate = find_output(&x_wire, &y_wire, Operator::And, &unsolved_gates).unwrap();
      let carry_in_xor_gate = find_output(&ab_xor_gate, &current_carry_wire.clone().unwrap(), Operator::Xor, &unsolved_gates);
      if carry_in_xor_gate.is_none() {
        swaps.push(ab_xor_gate.clone());
        swaps.push(ab_and_gate.clone());
        unsolved_gates = swap_outputs(ab_xor_gate.clone(), ab_and_gate.clone(), &unsolved_gates);
        bit = 0;
        continue;
      }
      if carry_in_xor_gate.clone().unwrap() != z_wire {
        swaps.push(carry_in_xor_gate.clone().unwrap());
        swaps.push(z_wire.clone());
        unsolved_gates = swap_outputs(carry_in_xor_gate.clone().unwrap(), z_wire.clone(), &unsolved_gates);
        bit = 0;
        continue;
      }
      let carry_in_and_gate = find_output(&ab_xor_gate, &current_carry_wire.clone().unwrap(), Operator::And, &unsolved_gates);
      let carry_wire = find_output(&ab_and_gate, &carry_in_and_gate.unwrap(), Operator::Or, &unsolved_gates).unwrap();
      current_carry_wire = Some(carry_wire);
    }
    bit += 1;
    if bit >= z_wires {
      break;
    }
  }
  swaps.sort();
  swaps.join(",")
}

fn find_output(input1: &String, input2: &String, operator: Operator, unsolved_gates: &Vec<Gate>) -> Option<String> {
  for gate in unsolved_gates {
    if ((gate.input1 == *input1 && gate.input2 == *input2) || (gate.input1 == *input2 && gate.input2 == * input1)) && gate.operator == operator {
      return Some(gate.output.clone());
    }
  }
  None
}

fn swap_outputs(input1: String, input2: String, unsolved_gates: &[Gate]) -> Vec<Gate> {
  unsolved_gates.iter().map(|g| {
    if g.output == *input1 {
      Gate{
        input1: g.input1.clone(),
        input2: g.input2.clone(),
        output: input2.clone(),
        operator: g.operator.clone(),
      }
    } else if g.output == *input2 {
      Gate{
        input1: g.input1.clone(),
        input2: g.input2.clone(),
        output: input1.clone(),
        operator: g.operator.clone(),
      }
    } else {
      g.clone()
    }
  }).collect()
}
