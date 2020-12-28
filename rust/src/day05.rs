use crate::common;
use std::collections::HashSet;

fn get_id(seat: &str) -> u32 {
  seat
    .chars()
    .fold(0, |res, c| res * 2 + "BR".contains(c) as u32)
}

pub(crate) fn solution() {
  let lines = common::read_lines(&common::data_file(5)).unwrap();
  let ids: Vec<u32> = lines.iter().map(|x| get_id(x)).collect();

  let max_id: u32 = *ids.iter().max().unwrap();
  let id_set = ids.iter().cloned().collect::<HashSet<u32>>();

  let mut seat: u32 = 0;
  for i in 0..=(1 << lines[0].len()) {
    let id = i as u32;
    if !id_set.contains(&id) && id_set.contains(&(id + 1)) && id_set.contains(&(id - 1)) {
      seat = id;
      break;
    }
  }
  println!("Max ID: {}, Seat: {}", max_id, seat);
}
