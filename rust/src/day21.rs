use crate::common;
use std::collections::{HashMap, HashSet};

type Rule = (HashSet<String>, HashSet<String>);
type Mapping = HashMap<String, String>;

fn parse_rule(line: &str) -> Rule {
  let mut parts = line.split(" (contains ");
  let c = parts
    .next()
    .unwrap()
    .split(" ")
    .map(|s| s.to_string())
    .collect();
  let a = parts
    .next()
    .unwrap()
    .trim_matches(')')
    .split(", ")
    .map(|s| s.to_string())
    .collect();
  (c, a)
}

fn get_mapping(rules: &Vec<Rule>) -> Mapping {
  fn first(set: &HashSet<String>) -> String {
    set.iter().next().unwrap().clone()
  }
  let mut res = Mapping::new();
  for (i, (c, a)) in rules.iter().enumerate() {
    if c.len() == 1 && a.len() == 1 {
      res.insert(first(c), first(a));
      continue;
    }
    let mut cxc = c.clone();
    let mut cxa = a.clone();
    for j in (i + 1)..rules.len() {
      let (c1, a1) = &rules[j];
      let xa: HashSet<String> = cxa.intersection(&a1).map(|s| s.to_string()).collect();
      if !xa.is_empty() {
        cxc = cxc.intersection(&c1).map(|s| s.to_string()).collect();
        cxa = xa;
      }
      if cxc.len() == 1 && cxa.len() == 1 {
        res.insert(first(&cxc), first(&cxa));
        break;
      }
    }
  }
  res
}

fn trim_rules(rules: &mut Vec<Rule>) -> Mapping {
  let mut res = Mapping::new();
  loop {
    let mapping = get_mapping(rules);
    if mapping.is_empty() {
      return res;
    }
    for (c, a) in mapping {
      res.insert(c.clone(), a.clone());
      for i in 0..rules.len() {
        rules[i].0.remove(&c);
        rules[i].1.remove(&a);
      }
    }
  }
}

pub(crate) fn solution() {
  let mut rules: Vec<Rule> = common::read_lines(&common::data_file(21))
    .unwrap()
    .iter()
    .map(|s| parse_rule(s))
    .collect();

  let mapping = trim_rules(&mut rules);
  let res1: usize = rules.iter().map(|r| r.0.len()).sum();
  println!("Answer 1: {}", res1);

  let mut m: Vec<(&String, &String)> = mapping.iter().collect();
  m.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());
  println!(
    "Answer 2: {}",
    m.iter()
      .map(|x| x.0.clone())
      .collect::<Vec<String>>()
      .join(",")
  );
}
