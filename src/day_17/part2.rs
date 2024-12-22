use super::lib::run_program;

pub fn part2(lines: &[String]) -> i64 {
  let program = lines[4].replace("Program: ", "").split(",").map(|v| v.parse().unwrap()).collect::<Vec<i64>>();
  let mut rega = 0;
  let mut j: i64 = 1;
  let mut istart = 0;
  while j <= program.len() as i64 && j >= 0 {
    rega <<= 3;
    let mut proceed = true;
    let mut found_i = 0;
    for i in istart..8 {
      found_i = i;
      if program[program.len() - j as usize..program.len()] == run_program(&program, rega + i, 0, 0) {
        proceed = false;
        break;
      }
    }
    if proceed {
      j -= 1;
      rega >>= 3;
      istart = rega%8+1;
      rega >>= 3;
      continue;
    }
    j += 1;
    rega += found_i;
    istart = 0;
  }
  rega
}

