type Point = (i64, i64);

#[derive(Debug)]
struct Robot {
  pub position: Point,
  pub velocity: Point,
}

pub fn part1(lines: &[String]) -> i64 {
  let parts = lines.iter().map(|l| l.split(" ").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
  let robots = parts.iter().map(|p| {
    let positions: Vec<i64> = p[0].replace("p=", "").split(",").map(|v| v.parse().unwrap()).collect();
    let velocities: Vec<i64> = p[1].replace("v=", "").split(",").map(|v| v.parse().unwrap()).collect();
    Robot {
      position: (positions[0], positions[1]),
      velocity: (velocities[0], velocities[1]),
    }
  });
  let height = 103;
  let width = 101;
  let seconds = 100;
  let final_robots = robots.map(|r| {
    let mut new_x = (r.position.0 + (r.velocity.0 * seconds)) % width;
    if new_x < 0 {
      new_x += width;
    }
    let mut new_y = (r.position.1 + (r.velocity.1 * seconds)) % height;
    if new_y < 0 {
      new_y += height;
    }
    Robot {
      position: (new_x, new_y),
      velocity: r.velocity,
    }
  }).collect::<Vec<Robot>>();
  let mut quad_scores = vec![0,0,0,0];
  for r in final_robots {
    match r.position.0.cmp(&(width / 2)) {
      std::cmp::Ordering::Less => {
        match r.position.1.cmp(&(height / 2)) {
          std::cmp::Ordering::Less => quad_scores[0] += 1,
          std::cmp::Ordering::Greater => quad_scores[1] += 1,
          _ => (),
        }
      },
      std::cmp::Ordering::Greater => {
        match r.position.1.cmp(&(height / 2)) {
          std::cmp::Ordering::Less => quad_scores[2] += 1,
          std::cmp::Ordering::Greater => quad_scores[3] += 1,
          _ => (),
        }
      },
      _ => (),
    }
  }
  quad_scores.iter().product()
}
