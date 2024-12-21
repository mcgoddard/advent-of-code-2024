#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Colors {
  White,
  Blue,
  Black,
  Red,
  Green,
}

pub fn parse_input(lines: &[String]) -> (Vec<Vec<Colors>>, Vec<Vec<Colors>>) {
  let available_towels = lines[0].split(", ").map(|t| {
    t.chars().map(|c| match c {
      'w' => Colors::White,
      'u' => Colors::Blue,
      'b' => Colors::Black,
      'r' => Colors::Red,
      'g' => Colors::Green,
      _ => panic!("Invalid available color"),
    }).collect()
  }).collect::<Vec<Vec<Colors>>>();
  let mut desired_designs = vec![];
  for line in &lines[2..] {
    let design = line.chars().map(|c| match c {
      'w' => Colors::White,
      'u' => Colors::Blue,
      'b' => Colors::Black,
      'r' => Colors::Red,
      'g' => Colors::Green,
      _ => panic!("Invalid design color"),
    }).collect::<Vec<Colors>>();
    desired_designs.push(design);
  }
  (available_towels, desired_designs)
}
