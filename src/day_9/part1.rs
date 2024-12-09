use std::cmp::Ordering;

use super::lib::{parse_blocks, Blocks, Empty, File};

pub fn part1(lines: Vec<String>) -> i64 {
  let (mut blocks, _max_file_num, total_empty) = parse_blocks(&lines);
  if let Blocks::File(_) = blocks[blocks.len() - 1] {
    blocks.push(Blocks::Empty(Empty {
      length: 0,
    }));
  }
  loop {
    let last_empty_length;
    match &blocks[blocks.len() - 1] {
      Blocks::Empty(e) => {
        last_empty_length = e.length;
        if e.length == total_empty {
          break;
        }
      }
      Blocks::File(_) => panic!("Last block is a file!"),
    }
    let file_index = blocks.len() - 1 - match blocks.iter().rev().position(|b| {
      matches!(b, Blocks::File(_))
    }) {
      Some(i) => i,
      None => panic!("No last file in blocks!"),
    };
    let last_file = match blocks[file_index] {
      Blocks::File(f) => f,
      _ => panic!("Last file was not a file!"),
    };
    let empty_index = match blocks.iter().position(|b| {
      matches!(b, Blocks::Empty(_))
    }) {
      Some(i) => i,
      None => panic!("No empty found!"),
    };
    let first_empty = match blocks[empty_index] {
      Blocks::Empty(e) => e,
      _ => panic!("First empty was a file!"),
    };
    if first_empty.length == 0 {
      blocks.remove(empty_index);
      continue;
    }
    match last_file.length.cmp(&first_empty.length) {
      Ordering::Greater => {
        blocks.remove(empty_index);
        blocks.insert(empty_index, Blocks::File(File {
          length: first_empty.length,
          file_num: last_file.file_num,
        }));
        blocks.remove(file_index);
        blocks.insert(file_index, Blocks::File(File {
          length: last_file.length - first_empty.length,
          file_num: last_file.file_num,
        }));
        blocks.remove(blocks.len() - 1);
        blocks.push(Blocks::Empty(Empty {
          length: last_empty_length + first_empty.length,
        }))
      },
      Ordering::Less => {
        blocks.remove(file_index);
        blocks.remove(empty_index);
        blocks.insert(empty_index, Blocks::File(last_file));
        blocks.insert(empty_index + 1, Blocks::Empty(Empty {
          length: first_empty.length - last_file.length,
        }));
        blocks.remove(blocks.len() - 1);
        blocks.push(Blocks::Empty(Empty {
          length: last_empty_length + last_file.length,
        }));
      },
      Ordering::Equal => {
        blocks.remove(empty_index);
        blocks.insert(empty_index, Blocks::File(last_file));
        blocks.remove(file_index);
        blocks.remove(blocks.len() - 1);
        blocks.push(Blocks::Empty(Empty {
          length: last_empty_length + first_empty.length,
        }))
      },
    }
  }
  get_checksum(&blocks)
}

fn get_checksum(blocks: &Vec<Blocks>) -> i64 {
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
      Blocks::Empty(_) => {
        break;
      }
    }
  }
  checksum
}
