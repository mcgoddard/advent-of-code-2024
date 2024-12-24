use super::lib::run_program;

pub fn part2(lines: &[String]) -> String {
  let program = lines[4].replace("Program: ", "").split(",").map(|v| v.parse().unwrap()).collect::<Vec<i64>>();
  let mut a = 0;
  let mut j: i64 = 1;
  let mut i_start = 0;
  while j <= program.len() as i64 && j >= 0 {
    a <<= 3;
    let mut proceed = true;
    let mut found_i = 0;
    for i in i_start..8 {
      found_i = i;
      if program[program.len() - j as usize..program.len()] == run_program(&program, a + i, 0, 0) {
        proceed = false;
        break;
      }
    }
    if proceed {
      j -= 1;
      a >>= 3;
      i_start = a%8+1;
      a >>= 3;
      continue;
    }
    j += 1;
    a += found_i;
    i_start = 0;
  }
  a.to_string()
}

