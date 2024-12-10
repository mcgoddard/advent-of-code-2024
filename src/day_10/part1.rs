use super::lib::{get_ratings, get_trailheads, parse_map};

pub fn part1(lines: &[String]) -> i64 {
  let map = parse_map(lines);
  let trailheads = get_trailheads(&map);
  let ratings = get_ratings(&map, trailheads);
  ratings.keys().len() as i64
}
