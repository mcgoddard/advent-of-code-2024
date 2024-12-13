#[derive(Debug)]
pub struct Machine {
  pub target: (i64, i64),
  pub a: (i64, i64),
  pub b: (i64, i64),
}

pub fn parse_machines(lines: &[String], add_target: i64) -> Vec<Machine> {
  let mut machines = vec![];
  for i in 0..(lines.len() + 1) /4 {
    let first_line = lines[i * 4].split(" ").collect::<Vec<&str>>();
    let a_x = first_line[2].replace("X+", "").replace(",", "").parse().unwrap();
    let a_y = first_line[3].replace("Y+", "").parse().unwrap();
    let second_line = lines[i * 4 + 1].split(" ").collect::<Vec<&str>>();
    let b_x = second_line[2].replace("X+", "").replace(",", "").parse().unwrap();
    let b_y = second_line[3].replace("Y+", "").parse().unwrap();
    let third_line = lines[i * 4 + 2].split(" ").collect::<Vec<&str>>();
    let prize_x = third_line[1].replace("X=", "").replace(",", "").parse::<i64>().unwrap() + add_target;
    let prize_y = third_line[2].replace("Y=", "").parse::<i64>().unwrap() + add_target;
    machines.push(Machine {
      target: (prize_x, prize_y),
      a: (a_x, a_y),
      b: (b_x, b_y),
    });
  }
  machines
}
