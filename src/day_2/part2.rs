use crate::day_2::lib::test_report;

pub fn part2(lines: Vec<String>) -> i64 {
  let reports = lines.iter().map(|line| line.split(" ").collect::<Vec<&str>>().iter().map(|v| v.parse().unwrap()).collect()).collect::<Vec<Vec<i64>>>();
  let report_safe = reports.iter().map(|report| {
    let mut variations = vec![report.clone()];
    for i in 0..report.len() {
      let mut new_variation = variations[0].clone();
      new_variation.remove(i);
      variations.push(new_variation);
    }
    let variation_results = variations.iter().map(test_report).collect::<Vec<bool>>();
    variation_results.iter().any(|v| *v)
  }).collect::<Vec<bool>>();
  let report_safe_true = report_safe.iter().filter(|v| **v).collect::<Vec<&bool>>();
  report_safe_true.len() as i64
}
