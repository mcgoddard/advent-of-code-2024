use std::time::Instant;

use super::lib::{a_star, a_star_2, Space};

pub fn part1(lines: &[String]) -> String {
  let coordinates = lines.iter().map(|l| l.split(",").map(|s| s.parse().unwrap()).collect()).collect::<Vec<Vec<usize>>>();
  let mut map = vec![];
  let width = 71;
  let height = 71;
  let num_bytes = 1024;
  for _ in 0..height {
    let row = vec![Space::Open; width];
    map.push(row);
  }
  for coordinate in coordinates[0..num_bytes].iter() {
    map[coordinate[1]][coordinate[0]] = Space::Corrupted;
  }
  let now = Instant::now();
  if let Some(path) = a_star_2(&map, (width as i64 - 1, height - 1)) {
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    path.len().to_string()
  } else {
    panic!("No route found!");
  }
}
