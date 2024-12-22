pub enum Opcode {
  Adv,
  Bxl,
  Bst,
  Jnz,
  Bxc,
  Out,
  Bdv,
  Csv,
}

pub fn get_combo(n: i64, a: i64, b: i64, c: i64) -> i64 {
  match n {
    0..=3 => n,
    4 => a,
    5 => b,
    6 => c,
    7 => panic!("Reserved value 7 found"),
    _ => panic!("Unknown combo value"),
  }
}

pub fn run_program(program: &[i64], a: i64, b: i64, c: i64) -> Vec<i64> {
  let mut a = a;
  let mut b = b;
  let mut c = c;
  let mut instruction_pointer = 0;
  let mut outputs = vec![];
  loop {
    if instruction_pointer >= program.len() as i64 {
      break;
    }
    let opcode = match program[instruction_pointer as usize] {
      0 => Opcode::Adv,
      1 => Opcode::Bxl,
      2 => Opcode::Bst,
      3 => Opcode::Jnz,
      4 => Opcode::Bxc,
      5 => Opcode::Out,
      6 => Opcode::Bdv,
      7 => Opcode::Csv,
      _ => panic!("Unknown opcode"),
    };
    if instruction_pointer + 1 >= program.len() as i64 {
      break;
    }
    let operand = program[(instruction_pointer + 1) as usize];
    match opcode {
      Opcode::Adv => {
        let combo = get_combo(operand, a, b, c);
        a /= (2_i64).pow(combo as u32);
      },
      Opcode::Bxl => {
        b ^= operand;
      },
      Opcode::Bst => {
        b = get_combo(operand, a, b, c) % 8;
      },
      Opcode::Jnz => {
        if a != 0 {
          instruction_pointer = operand;
          continue;
        }
      },
      Opcode::Bxc => {
        b ^= c;
      },
      Opcode::Out => {
        let combo = get_combo(operand, a, b, c);
        outputs.push(combo % 8);
      },
      Opcode::Bdv => {
        let combo = get_combo(operand, a, b, c);
        b = a / (2_i64).pow(combo as u32);
      },
      Opcode::Csv => {
        let combo = get_combo(operand, a, b, c);
        c = a / (2_i64).pow(combo as u32);
      },
    }
    instruction_pointer += 2;
  }
  outputs
}
