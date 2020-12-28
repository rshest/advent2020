use crate::common;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct PasswordDesc {
  cnt1: usize,
  cnt2: usize,
  ch: char,
  pwd: String,
}

impl FromStr for PasswordDesc {
  type Err = std::num::ParseIntError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let parts: Vec<&str> = s.split(' ').collect();
    let counts: Vec<usize> = parts[0].split('-').map(|p| p.parse().unwrap()).collect();
    Ok(PasswordDesc {
      cnt1: counts[0],
      cnt2: counts[1],
      ch: parts[1].as_bytes()[0] as char,
      pwd: parts[2].to_owned(),
    })
  }
}

fn is_valid1(p: &PasswordDesc) -> bool {
  let cnt = p.pwd.matches(p.ch).count();
  p.cnt1 <= cnt && cnt <= p.cnt2
}

fn is_valid2(p: &PasswordDesc) -> bool {
  let pwd = p.pwd.as_bytes();
  let ch = p.ch as u8;
  (pwd[p.cnt1 - 1] == ch) ^ (pwd[p.cnt2 - 1] == ch)
}

pub(crate) fn solution() {
  let passwords: Vec<PasswordDesc> = common::read_lines(&common::data_file(2))
    .unwrap()
    .iter()
    .filter_map(|line| line.parse().ok())
    .collect();

  println!(
    "Number of valid passwords: {}, {}",
    passwords.iter().filter(|x| is_valid1(x)).count(),
    passwords.iter().filter(|x| is_valid2(x)).count()
  );
}
