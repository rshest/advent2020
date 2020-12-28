use crate::common;
use std::collections::HashMap;
extern crate regex;

use regex::Regex;

#[derive(Debug, PartialEq)]
struct MemOp {
  mask: String,
  ops: Vec<(i64, i64)>,
}

fn parse_ops(data: &str) -> Vec<MemOp> {
  lazy_static! {
    static ref MEM_REGEX: Regex = Regex::new(r"^mem\[(?P<addr>\d+)\] = (?P<value>\d+)$").unwrap();
  }

  fn get_match_val(cap: &regex::Captures, group_name: &str) -> Option<i64> {
    match cap.name(group_name)?.as_str().parse::<i64>() {
      Ok(val) => Some(val),
      _ => None,
    }
  }

  fn parse_op(lines: &Vec<&str>) -> Option<MemOp> {
    if lines.len() <= 1 {
      return None;
    }
    Some(MemOp {
      mask: lines[0].to_owned(),
      ops: lines[1..]
        .iter()
        .filter_map(|line| {
          let c = MEM_REGEX.captures(line)?;
          Some((get_match_val(&c, "addr")?, get_match_val(&c, "value")?))
        })
        .collect(),
    })
  }

  data
    .split("mask = ")
    .filter_map(|chunk| parse_op(&chunk.split("\n").collect()))
    .collect()
}

fn apply_mask1(val: &i64, mask: &str) -> i64 {
  let mut res = *val;
  let mut bit = 1;
  for i in (0..mask.len()).rev() {
    match mask.as_bytes()[i] as char {
      '0' => res = res & !bit,
      '1' => res = res | bit,
      _ => (),
    };
    bit = bit << 1;
  }
  res
}

fn eval1(mem_ops: &Vec<MemOp>) -> HashMap<i64, i64> {
  let mut mem = HashMap::new();
  for mem_op in mem_ops {
    for (addr, val) in &mem_op.ops {
      mem.insert(*addr, apply_mask1(val, &mem_op.mask));
    }
  }
  mem
}

fn apply_mask2(val: &i64, mask: &str) -> Vec<i64> {
  fn rec(val: i64, mask: &str, pos: usize, res: &mut Vec<i64>) {
    if pos == mask.len() {
      res.push(val);
      return;
    }
    let bit = 1 << (mask.len() - pos - 1);
    let c = mask.as_bytes()[pos] as char;
    if c == '0' {
      rec(val, mask, pos + 1, res);
    }
    if c == 'X' {
      rec(val & !bit, mask, pos + 1, res);
    }
    if c == '1' || c == 'X' {
      rec(val | bit, mask, pos + 1, res);
    }
  }
  let mut res: Vec<i64> = Vec::new();
  rec(*val, mask, 0, &mut res);
  res
}

fn eval2(mem_ops: &Vec<MemOp>) -> HashMap<i64, i64> {
  let mut mem = HashMap::new();
  for mem_op in mem_ops {
    for (addr, val) in &mem_op.ops {
      for masked_addr in apply_mask2(addr, &mem_op.mask) {
        mem.insert(masked_addr, *val);
      }
    }
  }
  mem
}

pub(crate) fn solution() {
  let data = std::fs::read_to_string(&common::data_file(14)).unwrap();
  let ops = parse_ops(&data);
  println!("Answer 1: {:?}", eval1(&ops).values().sum::<i64>());
  println!("Answer 2: {:?}", eval2(&ops).values().sum::<i64>());
}
