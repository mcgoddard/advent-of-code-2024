pub fn test_report(report: &Vec<i64>) -> bool {
  let start = report[0];
  let mut previous = report[1];
  if start == previous || (start - previous).abs() > 3 {
    return false;
  }
  let increasing = start < previous;
  for level in report.iter().skip(2) {
    if (increasing && previous > *level) || (!increasing && previous < *level) {
      return false;
    }
    let diff = (previous - level).abs();
    if !(1..=3).contains(&diff) {
      return false;
    }
    previous = *level;
  }
  true
}
