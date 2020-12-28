use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

pub fn read_lines(path: &str) -> Result<Vec<String>, Error> {
  let file = File::open(path)?;
  BufReader::new(file).lines().collect()
}

pub fn read_integers(path: &str) -> Result<Vec<i64>, Error> {
  let mut v = Vec::new();
  for line in read_lines(path)? {
    let n = line
      .trim()
      .parse()
      .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
    v.push(n);
  }
  Ok(v)
}

const DATA_ROOT: &str = "../data/";

pub fn data_file(problem_id: u32) -> String {
  format!("{}/{:02}.txt", DATA_ROOT, problem_id)
}

pub(crate) fn extra_file(problem_id: u32, suffix: &str) -> String {
  format!("{}/{:02}.{}.txt", DATA_ROOT, problem_id, suffix)
}
