use crate::common;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::str::FromStr;

type Passport = HashMap<String, String>;

fn read_passport(line: &str) -> Passport {
  line
    .split_whitespace()
    .filter_map(|s| {
      let mut parts = s.split(':');
      Some((parts.next()?.to_owned(), parts.next()?.to_owned()))
    })
    .collect()
}

const REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn is_valid1(passport: &Passport) -> bool {
  REQUIRED_FIELDS.iter().all(|f| passport.contains_key(*f))
}

fn is_valid2(passport: &Passport) -> bool {
  lazy_static! {
    static ref PID_REGEX: Regex = Regex::new(r"^(\d{9})$").unwrap();
    static ref HCL_REGEX: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    static ref HGT_REGEX: Regex = Regex::new(r"^(?P<in>\d+)in|(?P<cm>\d+)cm$").unwrap();
    static ref EYE_COLORS: HashSet<String> = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
      .iter()
      .copied()
      .map(|x| x.to_owned())
      .collect();
  }

  fn in_range<T: Ord + FromStr>(int_str: &str, minv: T, maxv: T) -> bool {
    match int_str.parse::<T>() {
      Ok(n) => n >= minv && n <= maxv,
      _ => false,
    }
  }

  fn is_valid_height(height_str: &str) -> bool {
    match HGT_REGEX.captures(height_str) {
      Some(c) => match (c.name("in"), c.name("cm")) {
        (Some(x), _) => in_range(x.as_str(), 59, 76),
        (_, Some(x)) => in_range(x.as_str(), 150, 193),
        _ => false,
      },
      _ => false,
    }
  }

  is_valid1(passport)
    && in_range(&passport["byr"], 1920, 2002)
    && in_range(&passport["iyr"], 2010, 2020)
    && in_range(&passport["eyr"], 2020, 2030)
    && EYE_COLORS.contains(passport["ecl"].as_str())
    && PID_REGEX.is_match(&passport["pid"])
    && HCL_REGEX.is_match(&passport["hcl"])
    && is_valid_height(&passport["hgt"])
}

pub(crate) fn solution() {
  let passports: Vec<Passport> = fs::read_to_string(&common::data_file(4))
    .unwrap()
    .split("\n\n")
    .map(|s| read_passport(s))
    .collect();

  println!(
    "Valid passports 1: {}",
    passports.iter().filter(|x| is_valid1(x)).count()
  );
  println!(
    "Valid passports 2: {}",
    passports.iter().filter(|x| is_valid2(x)).count()
  );
}
