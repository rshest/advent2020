use crate::common;
use std::collections::HashMap;

const TOTAL: i64 = 2020;

pub(crate) fn solution() {
  let nums: Vec<i64> = common::read_integers(&common::data_file(1)).unwrap();
  let mut reg = HashMap::new();

  for (i, n) in nums.iter().enumerate() {
    let rest = TOTAL - n;
    if reg.contains_key(&rest) {
      println!("Answer 1: {} * {} = {}", n, rest, n * rest);
    }
    reg.insert(n, i);
  }

  for (i, n1) in nums.iter().enumerate() {
    for j in (i + 1)..nums.len() {
      let n2 = nums[j];
      let rest = TOTAL - n1 - n2;
      match reg.get(&rest) {
        Some(k) if k > &j => {
          println!("Answer 2: {} * {} * {} = {}", n1, n2, rest, n1 * n2 * rest)
        }
        _ => (),
      }
    }
  }
}
