use super::lib::parse_machines;

pub fn part1(lines: &[String]) -> String {
  let machines = parse_machines(lines, 0);
  let tokens = machines.iter().filter_map(|m| {
    let mut valid_tokens = vec![];
    for i in 0..100 {
      for j in 0..100 {
        if i * m.a.0 + j * m.b.0 == m.target.0 && i * m.a.1 + j * m.b.1 == m.target.1 {
          valid_tokens.push(3 * i + j);
        }
      }
    }
    valid_tokens.into_iter().min()
  }).collect::<Vec<i64>>();
  tokens.iter().sum::<i64>().to_string()
}
