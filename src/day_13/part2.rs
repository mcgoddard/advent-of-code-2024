use super::lib::parse_machines;

pub fn part2(lines: &[String]) -> i64 {
  let machines = parse_machines(lines, 10000000000000);
  let tokens = machines.iter().map(|m| {
    let determinant = m.a.0 * m.b.1 - m.a.1 * m.b.0;
    let n1 = (m.b.1 * m.target.0 - m.b.0 * m.target.1) / determinant;
    let n2 = (m.a.0 * m.target.1 - m.a.1 * m.target.0) / determinant;
    let a_correct = n1 * m.a.0 + n2 * m.b.0 == m.target.0;
    let b_correct = n1 * m.a.1 + n2 * m.b.1 == m.target.1;
    if a_correct && b_correct {
        (n1, n2)
    } else {
        (0, 0)
    }
  }).map(|t| t.0 * 3 + t.1).collect::<Vec<i64>>();
  tokens.iter().sum()
}
