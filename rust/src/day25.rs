use crate::common;

const FACTOR: usize = 20201227;
const DEFAULT_SUBJ_NUMBER: usize = 7;

fn tm_step(val: usize, subj_number: usize) -> usize {
  (val * subj_number) % FACTOR
}

fn transform(subj_number: usize, loop_size: usize) -> usize {
  (0..loop_size).fold(1, |val, _| tm_step(val, subj_number))
}

fn find_loop_len(pub_key: usize, subj_number: usize) -> usize {
  let mut val = 1;
  let mut i = 0;
  while val != pub_key {
    val = tm_step(val, subj_number);
    i += 1
  }
  return i;
}

pub(crate) fn solution() {
  let pub_keys: Vec<usize> = common::read_lines(&common::data_file(25))
    .unwrap()
    .iter()
    .map(|s| s.parse().unwrap())
    .collect();

  let loop_sizes: Vec<usize> = pub_keys
    .iter()
    .map(|x| find_loop_len(*x, DEFAULT_SUBJ_NUMBER))
    .collect();
  let res = transform(pub_keys[0], loop_sizes[1]);
  println!("Answer: {}", res);
}
