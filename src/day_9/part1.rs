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
    let mut last_file = File { length: 0, file_num: 0 };
    let mut file_index= 0;
    for i in (0..blocks.len() - 1).rev() {
      if let Blocks::File(f) = blocks[i] {
        file_index = i;
        last_file = f;
        break;
      }
    }
    let mut first_empty = Empty { length: 0 };
    let mut empty_index= 0;
    for (i, block) in blocks.iter().enumerate().take(blocks.len() - 1) {
      if let Blocks::Empty(e) = block {
        empty_index = i;
        first_empty = *e;
        break;
      }
    }
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
