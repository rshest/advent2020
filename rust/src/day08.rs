use crate::common;
use std::str::FromStr;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Op {
  Acc(i64),
  Nop(i64),
  Jmp(i64),
}

impl FromStr for Op {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut it = s.split(' ');
    let opstr: &str = it.next().ok_or(())?;
    let val: i64 = it.next().ok_or(())?.parse::<i64>().map_err(|_| ())?;
    match opstr {
      "acc" => Ok(Op::Acc(val)),
      "nop" => Ok(Op::Nop(val)),
      "jmp" => Ok(Op::Jmp(val)),
      _ => Err(()),
    }
  }
}

fn eval_ops(ops: &Vec<Op>) -> (bool, i64) {
  let n = ops.len() as i64;
  let mut visited: Vec<bool> = Vec::new();
  visited.resize(ops.len(), false);
  let mut ip: i64 = 0;
  let mut acc: i64 = 0;
  while ip >= 0 && ip < n {
    if visited[ip as usize] {
      return (false, acc);
    }
    visited[ip as usize] = true;
    match ops[ip as usize] {
      Op::Jmp(val) => ip += val - 1,
      Op::Acc(val) => acc += val,
      _ => (),
    }
    ip += 1;
  }
  (true, acc)
}

fn try_mutate_program(ops: &Vec<Op>) -> Option<i64> {
  let mut ops_copy = ops.clone();
  for (i, op) in ops.iter().enumerate() {
    let prev_op: Op = *op;
    let new_op = match op {
      Op::Jmp(val) => Op::Nop(*val),
      Op::Nop(val) => Op::Jmp(*val),
      _ => prev_op,
    };
    if prev_op != new_op {
      ops_copy[i] = new_op;
      let (terminated, acc) = eval_ops(&ops_copy);
      if terminated {
        return Some(acc);
      }
      ops_copy[i] = prev_op;
    }
  }
  None
}

pub(crate) fn solution() {
  let ops: Vec<Op> = common::read_lines(&common::data_file(8))
    .unwrap()
    .iter()
    .filter_map(|line| line.parse().ok())
    .collect();
  let (_, acc) = eval_ops(&ops);
  println!("Answer 1: {}", acc);
  println!("Answer 2: {}", try_mutate_program(&ops).unwrap());
}
