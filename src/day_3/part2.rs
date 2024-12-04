use regex::Regex;

pub fn part2(lines: Vec<String>) -> i64 {
  let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
  let mut full_line = lines[0].to_owned();
  for line in lines.iter().skip(1) {
    full_line.push_str(line);
  }
  let sections = full_line.split("do()").map(|s| s.split("don't()").collect::<Vec<&str>>()[0]).collect::<Vec<&str>>();
  let mut result = 0;
  for section in sections {
    for (_, [first, second]) in re.captures_iter(section).map(|c| c.extract()) {
      result += first.parse::<i64>().unwrap() * second.parse::<i64>().unwrap();
    }
  }
  result
}
