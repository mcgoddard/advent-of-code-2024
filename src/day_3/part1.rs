use regex::Regex;

pub fn part1(lines: &[String]) -> i64 {
  let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
  let mut result = 0;
  for line in lines {
    for (_, [first, second]) in re.captures_iter(line).map(|c| c.extract()) {
      result += first.parse::<i64>().unwrap() * second.parse::<i64>().unwrap();
    }
  }
  result
}
