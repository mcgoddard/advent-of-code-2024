use core::panic;

enum Opcode {
  Adv,
  Bxl,
  Bst,
  Jnz,
  Bxc,
  Out,
  Bdv,
  Csv,
}

pub fn part1(lines: &[String]) -> i64 {
  let mut instruction_pointer: i64 = 0;
  let mut a = lines[0].replace("Register A: ", "").parse::<i64>().unwrap();
  let mut b = lines[1].replace("Register B: ", "").parse::<i64>().unwrap();
  let mut c = lines[2].replace("Register C: ", "").parse::<i64>().unwrap();
  let program = lines[4].replace("Program: ", "").split(",").map(|v| v.parse().unwrap()).collect::<Vec<i64>>();
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
        a = a / (2 as i64).pow(combo as u32);
      },
      Opcode::Bxl => {
        b = b ^ operand;
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
        b = b ^ c;
      },
      Opcode::Out => {
        let combo = get_combo(operand, a, b, c);
        outputs.push(combo % 8);
      },
      Opcode::Bdv => {
        let combo = get_combo(operand, a, b, c);
        b = a / (2 as i64).pow(combo as u32);
      },
      Opcode::Csv => {
        let combo = get_combo(operand, a, b, c);
        c = a / (2 as i64).pow(combo as u32);
      },
    }
    instruction_pointer += 2;
  }
  println!("{:?}", outputs.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(","));
  0
}

fn get_combo(n: i64, a: i64, b: i64, c: i64) -> i64 {
  match n {
    0..=3 => n,
    4 => a,
    5 => b,
    6 => c,
    7 => panic!("Reserved value 7 found"),
    _ => panic!("Unknown combo value"),
  }
}
