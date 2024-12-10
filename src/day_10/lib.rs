use std::collections::HashMap;

type TrailDef = ((i64, i64), (i64, i64));

pub fn parse_map(lines: &[String]) -> Vec<Vec<i64>> {
  lines.iter().map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i64).collect()).collect::<Vec<Vec<i64>>>()
}

pub fn get_trailheads(map: &[Vec<i64>]) -> Vec<(i64, i64)> {
  let mut trailheads = vec![];
  for y in 0..map.len() {
    for x in 0..map[0].len() {
      if map[y][x] == 0 {
        trailheads.push((x as i64, y as i64));
      }
    }
  }
  trailheads
}

pub fn get_ratings(map: &[Vec<i64>], trailheads: Vec<(i64, i64)>) -> HashMap<TrailDef, i64> {
  let mut ratings: HashMap<TrailDef, i64> = HashMap::new();
  for trailhead in trailheads {
    let mut positions = vec![];
    positions.push(trailhead);
    for target in 1..=9 {
      let mut next_positions = vec![];
      for position in positions {
        if position.0 > 0 && map[position.1 as usize][(position.0 - 1) as usize] == target {
          next_positions.push((position.0 - 1, position.1));
        }
        if position.1 > 0 && map[(position.1 - 1) as usize][position.0 as usize] == target {
          next_positions.push((position.0, position.1 - 1));
        }
        if position.0 + 1 < map[0].len() as i64 && map[position.1 as usize][(position.0 + 1) as usize] == target {
          next_positions.push((position.0 + 1, position.1));
        }
        if position.1 + 1 < map.len() as i64 && map[(position.1 + 1) as usize][position.0 as usize] == target {
          next_positions.push((position.0, position.1 + 1));
        }
      }
      positions = next_positions.clone();
      if target == 9 {
        for next_position in next_positions {
          *ratings.entry((trailhead, next_position)).or_insert(0) += 1;
        }
      }
    }
  }
  ratings
}
