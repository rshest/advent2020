use crate::common;
use std::collections::HashSet;

extern crate itertools;
use itertools::Itertools;

type Field = HashSet<Vec<i16>>;

fn count_neighbors(field: &Field, pos: &Vec<i16>) -> usize {
  pos
    .iter()
    .map(|x| (x - 1)..=(x + 1))
    .multi_cartesian_product()
    .filter(|p| p != pos && field.contains(p))
    .count()
}

fn get_bounds(field: &Field) -> Option<Vec<(i16, i16)>> {
  if field.len() == 0 {
    return None;
  }
  let pos0 = field.iter().next().unwrap();
  let mut res: Vec<(i16, i16)> = pos0.iter().map(|x| (*x, *x)).collect();
  for pos in field.iter() {
    for (i, x) in pos.iter().enumerate() {
      res[i] = (*x.min(&res[i].0), *x.max(&res[i].1));
    }
  }
  Some(res)
}

fn step(field: &Field) -> Field {
  let mut res = Field::new();
  let bounds = get_bounds(&field);
  if bounds == None {
    return res;
  }
  let coord_it = bounds
    .unwrap()
    .iter()
    .map(|(a, b)| (a - 1)..=(b + 1))
    .multi_cartesian_product();
  for pos in coord_it {
    let n = count_neighbors(field, &pos);
    if field.contains(&pos) {
      if n == 2 || n == 3 {
        res.insert(pos);
      }
    } else {
      if n == 3 {
        res.insert(pos);
      }
    }
  }
  res
}

fn init_field(seed: &Vec<String>, ndim: usize) -> Field {
  let mut res = Field::new();
  for (y, line) in seed.iter().enumerate() {
    for (x, c) in line.as_bytes().iter().enumerate() {
      if *c as char == '#' {
        let mut key: Vec<i16> = Vec::new();
        key.resize(ndim, 0);
        key[0] = x as i16;
        key[1] = y as i16;
        res.insert(key);
      }
    }
  }
  res
}

const NUM_ITER: usize = 6;

fn run(seed: &Vec<String>, ndim: usize) -> usize {
  let mut field = init_field(&seed, ndim);
  for _ in 0..NUM_ITER {
    field = step(&field);
  }
  field.len()
}

pub(crate) fn solution() {
  let seed = common::read_lines(&common::data_file(17)).unwrap();
  println!("Answer 1: {}", run(&seed, 3));
  println!("Answer 2: {}", run(&seed, 4));
}
