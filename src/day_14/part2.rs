type Point = (i64, i64);

#[derive(Debug)]
struct Robot {
  pub position: Point,
  pub velocity: Point,
}

pub fn part2(lines: &[String]) -> String {
  let parts = lines.iter().map(|l| l.split(" ").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
  let mut robots = parts.iter().map(|p| {
    let positions: Vec<i64> = p[0].replace("p=", "").split(",").map(|v| v.parse().unwrap()).collect();
    let velocities: Vec<i64> = p[1].replace("v=", "").split(",").map(|v| v.parse().unwrap()).collect();
    Robot {
      position: (positions[0], positions[1]),
      velocity: (velocities[0], velocities[1]),
    }
  }).collect::<Vec<Robot>>();
  let height = 103;
  let width = 101;
  let seconds = 10000;
  for s in 0..seconds {
    let new_robots = robots.iter().map(|r| {
      let mut new_x = (r.position.0 + r.velocity.0) % width;
      if new_x < 0 {
        new_x += width;
      }
      let mut new_y = (r.position.1 + r.velocity.1) % height;
      if new_y < 0 {
        new_y += height;
      }
      Robot {
        position: (new_x, new_y),
        velocity: r.velocity,
      }
    }).collect::<Vec<Robot>>();
    let mut map = vec![];
    for _ in 0..height {
      map.push(vec![false; width as usize]);
    }
    for r in robots {
      map[r.position.1 as usize][r.position.0 as usize] = true;
    }
    let mut mirrored = true;
    for i in 0..height {
      for (index, j) in (width/2 + 1..width).enumerate() {
        if map[i as usize][j as usize] != map[i as usize][(width as usize / 2) - (index + 1)] {
          mirrored = false;
          break;
        }
      }
      if !mirrored {
        break;
      }
    }
    if mirrored {
      return s.to_string();
    }
    println!("{:?}", s);
    for line in map {
      for entry in line {
        if entry {
          print!("1");
        } else {
          print!("0");
        }
      }
      println!();
    }
    robots = new_robots;
  }
  panic!("Not found!");
}
