use crate::day_2::lib::test_report;

pub fn part1(lines: &[String]) -> String {
  let reports = lines.iter().map(|line| line.split(" ").collect::<Vec<&str>>().iter().map(|v| v.parse().unwrap()).collect()).collect::<Vec<Vec<i64>>>();
  let report_safe = reports.iter().map(test_report).collect::<Vec<bool>>();
  let report_safe_true = report_safe.iter().filter(|v| **v).collect::<Vec<&bool>>();
  report_safe_true.len().to_string()
}
