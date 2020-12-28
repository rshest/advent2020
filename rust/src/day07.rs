use crate::common;
use std::collections::{HashMap, HashSet};

type Mapping = HashMap<String, HashMap<String, u32>>;
type BackMapping = HashMap<String, Vec<String>>;

fn parse_mapping(line: &str) -> (String, HashMap<String, u32>) {
  let parts: Vec<&str> = line.split_whitespace().collect();
  let bag_color = parts[0..=1].join(" ");
  let mut res: HashMap<String, u32> = HashMap::new();
  for i in (4..parts.len()).step_by(4) {
    if parts[i] != "no" {
      let child_bag_color = parts[i + 1..i + 3].join(" ");
      res.insert(child_bag_color, parts[i].parse::<u32>().unwrap());
    }
  }
  (bag_color, res)
}

fn build_back_mapping(mapping: &Mapping) -> BackMapping {
  let mut res: HashMap<_, _> = HashMap::new();
  for (key, children) in mapping {
    for (child, _) in children {
      res
        .entry(child.clone())
        .or_insert(Vec::new())
        .push(key.clone());
    }
  }
  res
}

fn count_reachable(root: &str, back_mapping: &BackMapping) -> u32 {
  fn rec(node: &str, back_mapping: &BackMapping, visited: &mut HashSet<String>) -> u32 {
    if visited.contains(node) {
      return 0;
    }
    let mut res = 1;
    if back_mapping.contains_key(node) {
      for child in &back_mapping[node] {
        res += rec(&child, back_mapping, visited);
      }
    }
    visited.insert(node.to_owned());
    res
  }
  let mut visited: HashSet<String> = HashSet::new();
  rec(root, back_mapping, &mut visited)
}

fn count_contains(root: &str, mapping: &Mapping) -> u32 {
  let mut res: u32 = 1;
  for (child, count) in &mapping[root] {
    res += count * count_contains(&child, mapping);
  }
  res
}

pub(crate) fn solution() {
  let mapping: Mapping = common::read_lines(&common::data_file(7))
    .unwrap()
    .iter()
    .map(|line| parse_mapping(line))
    .into_iter()
    .collect();
  let back_mapping = build_back_mapping(&mapping);
  const ROOT: &str = "shiny gold";
  println!("Reachable: {}", count_reachable(ROOT, &back_mapping) - 1);
  println!("Contains: {}", count_contains(ROOT, &mapping) - 1);
}
