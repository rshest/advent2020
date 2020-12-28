use crate::common;
use std::collections::HashSet;

#[derive(Debug)]
enum HexDir {
  E,
  W,
  NW,
  SW,
  NE,
  SE,
}
type HexCoord = (i64, i64);

fn parse_path(line: &str) -> Vec<HexDir> {
  let mut it = line.chars();
  let mut res = Vec::new();
  loop {
    let dir = match it.next() {
      Some('e') => HexDir::E,
      Some('w') => HexDir::W,
      Some(x) => match (x, it.next()) {
        ('n', Some('w')) => HexDir::NW,
        ('n', Some('e')) => HexDir::NE,
        ('s', Some('w')) => HexDir::SW,
        ('s', Some('e')) => HexDir::SE,
        _ => return res,
      },
      _ => return res,
    };
    res.push(dir);
  }
}

fn eval_step((x, y): &HexCoord, step: &HexDir) -> HexCoord {
  let dy = match step {
    HexDir::SW | HexDir::SE => 1,
    HexDir::NW | HexDir::NE => -1,
    _ => 0,
  };
  let dx = match step {
    HexDir::E => 1,
    HexDir::W => -1,
    HexDir::SW | HexDir::NW if y % 2 == 0 => -1,
    HexDir::SE | HexDir::NE if y % 2 != 0 => 1,
    _ => 0,
  };
  (x + dx, y + dy)
}

fn eval_path(steps: &Vec<HexDir>) -> HexCoord {
  steps.iter().fold((0, 0), |pos, step| eval_step(&pos, step))
}

fn eval_paths(paths: &Vec<Vec<HexDir>>) -> HashSet<HexCoord> {
  let mut res = HashSet::new();
  for path in paths {
    let pos = eval_path(path);
    if res.contains(&pos) {
      res.remove(&pos);
    } else {
      res.insert(pos);
    }
  }
  res
}

fn step_day(tiles: &HashSet<HexCoord>) -> HashSet<HexCoord> {
  let minx = tiles.iter().map(|t| t.0).min().unwrap();
  let maxx = tiles.iter().map(|t| t.0).max().unwrap();
  let miny = tiles.iter().map(|t| t.1).min().unwrap();
  let maxy = tiles.iter().map(|t| t.1).max().unwrap();

  let mut res = HashSet::new();
  for x in (minx - 1)..=(maxx + 1) {
    for y in (miny - 1)..=(maxy + 1) {
      let pos = (x, y);
      let num_neighbors: u32 = [
        HexDir::E,
        HexDir::W,
        HexDir::NW,
        HexDir::SW,
        HexDir::NE,
        HexDir::SE,
      ]
      .iter()
      .map(|dir| tiles.contains(&eval_step(&pos, dir)) as u32)
      .sum();
      if tiles.contains(&pos) {
        if num_neighbors == 0 || num_neighbors > 2 {
        } else {
          res.insert(pos);
        }
      } else if num_neighbors == 2 {
        res.insert(pos);
      }
    }
  }
  res
}

pub(crate) fn solution() {
  let paths: Vec<Vec<HexDir>> = common::read_lines(&common::data_file(24))
    .unwrap()
    .iter()
    .map(|s| parse_path(s))
    .collect();

  let mut tiles = eval_paths(&paths);
  println!("Answer 1: {}", tiles.len());

  for _ in 0..100 {
    tiles = step_day(&tiles);
  }
  println!("Answer 2: {}", tiles.len());
}
