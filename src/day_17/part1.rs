use crate::day_17::lib::run_program;

pub fn part1(lines: &[String]) -> i64 {
  let a = lines[0].replace("Register A: ", "").parse::<i64>().unwrap();
  let b = lines[1].replace("Register B: ", "").parse::<i64>().unwrap();
  let c = lines[2].replace("Register C: ", "").parse::<i64>().unwrap();
  let program = lines[4].replace("Program: ", "").split(",").map(|v| v.parse().unwrap()).collect::<Vec<i64>>();
  let outputs = run_program(&program, a, b, c);
  println!("{:?}", outputs.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(","));
  0
}
