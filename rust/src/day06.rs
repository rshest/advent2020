use crate::common;
use std::collections::HashMap;
use std::fs;

pub(crate) fn solution() {
  let text = fs::read_to_string(&common::data_file(6)).unwrap();
  let groups: Vec<Vec<&str>> = text
    .split("\n\n")
    .map(|s| s.split_whitespace().collect())
    .collect();

  let (mut res1, mut res2) = (0, 0);
  for group in &groups {
    let mut counts: HashMap<char, usize> = HashMap::new();
    for answers in group {
      for ans in answers.chars() {
        *counts.entry(ans).or_insert(0) += 1;
      }
    }
    res1 += counts.len();
    res2 += counts.iter().filter(|&(_, v)| *v == group.len()).count()
  }
  println!("Answer 1: {}, Answer 2: {}", res1, res2);
}
