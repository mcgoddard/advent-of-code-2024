use std::cmp::Ordering;

use super::lib::{parse_blocks, Blocks, Empty};

pub fn part2(lines: Vec<String>) -> i64 {
  let (mut blocks, max_file_num, _total_empty) = parse_blocks(&lines);
  for i in (0..max_file_num).rev() {
    let file_index = match blocks.iter().position(|b| {
      if let Blocks::File(f) = b {
        return f.file_num == i;
      }
      false
    }) {
      Some(i) => i,
      None => panic!("No index found for file {:?}", i),
    };
    let current_block = blocks[file_index];
    let current_file = match current_block {
      Blocks::File(f) => f,
      _ => panic!("Current block is not a file"),
    };
    let empty_index = match blocks.iter().position(|b| {
      if let Blocks::Empty(e) = b {
        return e.length >= current_file.length;
      }
      false
    }) {
      Some(i) => i,
      None => continue,
    };
    let current_empty = match blocks[empty_index] {
      Blocks::Empty(e) => e,
      _ => panic!("Current block is not an empty"),
    };
    if empty_index > file_index {
      continue;
    }
    match current_file.length.cmp(&current_empty.length) {
      Ordering::Greater => {},
      Ordering::Less => {
        blocks.remove(file_index);
        blocks.remove(empty_index);
        blocks.insert(empty_index, Blocks::File(current_file));
        blocks.insert(empty_index + 1, Blocks::Empty(Empty {
          length: current_empty.length - current_file.length,
        }));
        blocks.insert(file_index, Blocks::Empty(Empty {
          length: current_file.length,
        }))
      },
      Ordering::Equal => {
        blocks.remove(empty_index);
        blocks.insert(empty_index, Blocks::File(current_file));
        blocks.remove(file_index);
        blocks.insert(file_index, Blocks::Empty(Empty {
          length: current_empty.length,
        }));
      },
    }
  }
  get_checksum(&blocks)
}

fn get_checksum(blocks: &[Blocks]) -> i64 {
  let mut index = 0;
  let mut checksum = 0;
  for block in blocks {
    match block {
      Blocks::File(f) => {
        for _ in 0..f.length {
          checksum += index * f.file_num;
          index += 1
        }
      },
      Blocks::Empty(e) => {
        for _ in 0..e.length {
          index += 1
        }
      },
    }
  }
  checksum
}
