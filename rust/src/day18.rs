use crate::common;

#[derive(Debug)]
enum ArithOp {
  Add,
  Mul,
}

fn apply_op(op: Option<ArithOp>, a: Option<i64>, b: Option<i64>) -> Option<i64> {
  match op {
    Some(ArithOp::Add) => Some(a? + b?),
    Some(ArithOp::Mul) => Some(a? * b?),
    _ => {
      if a == None {
        b
      } else {
        a
      }
    }
  }
}

fn parse_num(exp: &str, pos: usize) -> (Option<i64>, usize) {
  let mut cpos = pos;
  let mut res = None;
  while cpos < exp.len() {
    let c = exp.as_bytes()[cpos] as char;
    if c < '0' || c > '9' {
      break;
    }
    let d = c as i64 - ('0' as i64);
    res = match res {
      Some(x) => Some(x * 10 + d),
      _ => Some(d),
    };
    cpos += 1;
  }
  (res, cpos)
}

fn eval_exp(exp: &str, pos: usize, add_precedence: bool) -> (Option<i64>, usize) {
  let mut res = None;
  let mut op = None;
  let mut cpos = pos;
  let mut muls = Vec::new();
  while cpos < exp.len() {
    match exp.as_bytes()[cpos] as char {
      ')' => {
        cpos += 1;
        break;
      }
      '(' => {
        let (val, pos1) = eval_exp(exp, cpos + 1, add_precedence);
        res = apply_op(op, res, val);
        cpos = pos1;
        op = None;
      }
      ' ' => cpos += 1,
      '*' => {
        if add_precedence {
          muls.push(res);
          res = None;
        } else {
          op = Some(ArithOp::Mul);
        }
        cpos += 1;
      }
      '+' => {
        op = Some(ArithOp::Add);
        cpos += 1;
      }
      _ => {
        let (val, pos1) = parse_num(exp, cpos);
        res = apply_op(op, res, val);
        op = None;
        cpos = pos1;
      }
    }
  }
  for mul in muls {
    res = Some(mul.unwrap() * res.unwrap());
  }
  (res, cpos)
}

pub(crate) fn solution() {
  let lines = common::read_lines(&common::data_file(18)).unwrap();
  let res1: i64 = lines
    .iter()
    .map(|line| eval_exp(line, 0, false).0.unwrap())
    .sum();
  println!("Answer 1: {}", res1);
  let res2: i64 = lines
    .iter()
    .map(|line| eval_exp(line, 0, true).0.unwrap())
    .sum();
  println!("Answer 2: {}", res2);
}
