use regex::Regex;

pub fn part1(lines: Vec<String>) -> i64 {
  let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
  let mut results = vec![];
  for line in lines {
    for (_, [first, second]) in re.captures_iter(&line).map(|c| c.extract()) {
      results.push((first.parse::<i64>().unwrap(), second.parse::<i64>().unwrap()));
    }
  }
  let mut result = 0;
  for (first, second) in results {
    result += first * second;
  }
  result
}
