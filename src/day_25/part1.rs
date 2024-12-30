struct Key {
  tumblers: Vec<i32>,
}

struct Lock {
  tumblers: Vec<i32>,
}

enum Schematic {
  Key(Key),
  Lock(Lock),
}

pub fn part1(lines: &[String]) -> String {
  let mut schematics = vec![];
  for i in 0..(lines.len() + 1) / 8 {
    let mut schematic = vec![];
    for j in 0..8 {
      if j == 7 {
        continue;
      }
      schematic.push(lines[i * 8 + j].chars().collect::<Vec<char>>());
    }
    schematics.push(schematic);
  }
  let locks_and_keys = schematics.iter().map(|s| {
    let mut tumblers = vec![0; 5];
    if s[0] == vec!['.'; 5] {
      for i in (1..6).rev() {
        for (j, tumbler) in tumblers.iter_mut().enumerate().take(5) {
          if s[i][j] == '#' {
            *tumbler += 1;
          }
        }
      }
      Schematic::Key(Key {
        tumblers,
      })
    } else {
      for row in s.iter().take(6).skip(1) {
        for (j, tumbler) in tumblers.iter_mut().enumerate().take(5) {
          if row[j] == '#' {
            *tumbler += 1;
          }
        }
      }
      Schematic::Lock(Lock {
        tumblers,
      })
    }
  }).collect::<Vec<_>>();
  let locks = locks_and_keys.iter().filter_map(|lk| {
    match lk {
      Schematic::Lock(l) => Some(l),
      _ => None,
    }
  }).collect::<Vec<_>>();
  let keys = locks_and_keys.iter().filter_map(|lk| {
    match lk {
      Schematic::Key(k) => Some(k),
      _ => None,
    }
  }).collect::<Vec<_>>();
  locks.iter().map(|l| {
    keys.iter().filter(|k| {
      l.tumblers.iter().zip(k.tumblers.iter()).all(|(lock_tumbler, key_tumbler)| lock_tumbler + key_tumbler <= 5)
    }).count() as i64
  }).sum::<i64>().to_string()
}
