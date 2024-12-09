#[derive(Debug, Clone, Copy)]
pub struct File {
  pub file_num: i64,
  pub length: i64,
}

#[derive(Debug, Clone, Copy)]
pub struct Empty {
  pub length: i64,
}

#[derive(Debug, Clone, Copy)]
pub enum Blocks {
  File(File),
  Empty(Empty),
}

pub fn parse_blocks(lines: &[String]) -> (Vec<Blocks>, i64, i64) {
  let numbers = lines[0].chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();
  let mut file_num = 0;
  let mut blocks = vec![];
  let mut total_empty = 0;
  for (index, number) in numbers.iter().enumerate() {
    match index % 2 {
      0 => {
        blocks.push(Blocks::File(File {
          file_num,
          length: *number as i64,
        }));
        file_num += 1;
      },
      1 => {
        blocks.push(Blocks::Empty(Empty {
          length: *number as i64,
        }));
        total_empty += *number as i64;
      },
      _ => {}
    }
  }
  (blocks, file_num, total_empty)
}
