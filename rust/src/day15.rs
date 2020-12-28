use crate::common;
use std::collections::HashMap;

fn eval(seed: &Vec<u64>, stop_k: u64) -> u64 {
  let mut positions = HashMap::new();
  let mut next_num = 0;
  let mut k: usize = 1;
  loop {
    let num = if k <= seed.len() {
      seed[k - 1]
    } else {
      next_num
    };
    if k as u64 == stop_k {
      return num;
    }
    match positions.get(&num) {
      Some(&x) => next_num = (k as u64) - x,
      _ => next_num = 0,
    };
    positions.insert(num, k as u64);
    k += 1;
  }
}

pub(crate) fn solution() {
  let seed = std::fs::read_to_string(&common::data_file(15))
    .unwrap()
    .split(",")
    .map(|x| x.parse::<u64>().unwrap())
    .collect();
  println!("Answer 1: {}", eval(&seed, 2020));
  println!("Answer 2: {}", eval(&seed, 30000000));
}
