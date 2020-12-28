use crate::common;

use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::iter::FromIterator;

#[derive(Debug)]
struct Node {
  val: usize,
  next: usize,
}

#[derive(Debug)]
struct CircularList {
  nodes: Vec<Node>,
  current: usize,
  lookup: HashMap<usize, usize>,
  min_val: usize,
  max_val: usize,
}

impl FromIterator<usize> for CircularList {
  fn from_iter<I: IntoIterator<Item = usize>>(iter: I) -> Self {
    let mut lookup = HashMap::new();
    let mut nodes = Vec::new();
    let mut i = 0;
    let (mut min_val, mut max_val) = (usize::MAX, 0);
    for val in iter {
      nodes.push(Node {
        val: val,
        next: i + 1,
      });
      lookup.insert(val, i);
      i += 1;
      min_val = val.min(min_val);
      max_val = val.max(max_val);
    }
    let n = nodes.len();
    nodes[n - 1].next = 0;
    CircularList {
      nodes: nodes,
      current: 0,
      lookup: lookup,
      min_val: min_val,
      max_val: max_val,
    }
  }
}

impl fmt::Display for CircularList {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let start_node = self.current;
    write!(f, "({}) ", self.nodes[start_node].val)?;
    let mut node = self.nodes[start_node].next;
    while node != start_node {
      write!(f, "{} ", self.nodes[node].val)?;
      node = self.nodes[node].next;
    }
    Ok(())
  }
}

impl CircularList {
  fn get_values(&self, num_values: usize, start_val: usize) -> Vec<usize> {
    let mut i = 0;
    let start_node = self.lookup[&start_val];
    let mut res = Vec::new();
    let mut node = self.nodes[start_node].next;
    loop {
      if node == start_node || i == num_values {
        return res;
      }
      res.push(self.nodes[node].val);
      node = self.nodes[node].next;
      i += 1;
    }
  }

  fn step(&mut self) {
    let mut to_move = vec![0; 3];
    let mut to_move_values = vec![0; 3];
    let mut node = self.nodes[self.current].next;
    for i in 0..3 {
      to_move[i] = node;
      to_move_values[i] = self.nodes[node].val;
      node = self.nodes[node].next;
    }
    self.nodes[self.current].next = node;

    let mut dest_val = self.nodes[self.current].val - 1;
    while to_move_values.contains(&dest_val) || !self.lookup.contains_key(&dest_val) {
      if dest_val < self.min_val {
        dest_val = self.max_val;
      } else {
        dest_val -= 1;
      }
    }
    let dest_node = self.lookup[&dest_val];

    let dnext = self.nodes[dest_node].next;
    self.nodes[dest_node].next = to_move[0];
    self.nodes[to_move[to_move.len() - 1]].next = dnext;
    self.current = self.nodes[self.current].next;
  }
}

fn eval_game(labels: &mut CircularList, steps: usize) {
  for i in 0..steps {
    if (i + 1) % 1000000 == 0 {
      println!("Step {}", i);
    }
    labels.step();
  }
}

pub(crate) fn solution() {
  let seed: Vec<usize> = fs::read_to_string(&common::data_file(23))
    .unwrap()
    .split("")
    .filter(|s| !s.is_empty())
    .map(|x| x.parse::<usize>().unwrap())
    .collect();

  let mut labels1 = CircularList::from_iter(seed.clone().into_iter());
  eval_game(&mut labels1, 100);
  let res1 = labels1
    .get_values(usize::MAX, 1)
    .iter()
    .map(|x| x.to_string())
    .collect::<Vec<String>>()
    .join("");
  println!("Answer 1: {}", res1);

  let max_val = seed.iter().max().unwrap();
  let extend_range = (max_val + 1)..(1000000 + max_val + 1 - seed.len());
  let mut labels2 = CircularList::from_iter(seed.into_iter().chain(extend_range));
  eval_game(&mut labels2, 10000000);
  let res2: usize = labels2.get_values(2, 1).iter().product();
  println!("Answer 2: {}", res2);
}
