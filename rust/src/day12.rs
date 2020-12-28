use crate::common;
use std::str::FromStr;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Action {
  North(i32),
  South(i32),
  East(i32),
  West(i32),
  Forward(i32),
  Left(i32),
  Right(i32),
}

impl FromStr for Action {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let c = s.as_bytes()[0] as char;
    let val = s[1..].parse::<i32>().map_err(|_| ())?;
    match c {
      'N' => Ok(Action::North(val)),
      'S' => Ok(Action::South(val)),
      'E' => Ok(Action::East(val)),
      'W' => Ok(Action::West(val)),
      'F' => Ok(Action::Forward(val)),
      'L' => Ok(Action::Left(val)),
      'R' => Ok(Action::Right(val)),
      _ => Err(()),
    }
  }
}

fn manhattan((x, y): (i32, i32)) -> i32 {
  x.abs() + y.abs()
}

fn move_dir((x, y): (i32, i32), dir: i32, val: i32) -> (i32, i32) {
  let ang = (dir as f32).to_radians();
  let ca = ang.cos().round() as i32;
  let sa = ang.sin().round() as i32;
  (x + val * ca, y + val * sa)
}

fn step1((x, y, dir): (i32, i32, i32), cmd: &Action) -> (i32, i32, i32) {
  match cmd {
    Action::North(val) => (x, y - val, dir),
    Action::South(val) => (x, y + val, dir),
    Action::West(val) => (x - val, y, dir),
    Action::East(val) => (x + val, y, dir),
    Action::Right(val) => (x, y, (dir + val) % 360),
    Action::Left(val) => (x, y, (dir - val + 360) % 360),
    Action::Forward(val) => {
      let (x1, y1) = move_dir((x, y), dir, *val);
      (x1, y1, dir)
    }
  }
}

fn rotate((x, y): (i32, i32), angle: i32) -> (i32, i32) {
  let ar = (angle as f32).to_radians();
  let (ca, sa) = (ar.cos(), ar.sin());
  let (xf, yf) = (x as f32, y as f32);
  let x1 = (ca * xf - sa * yf).round() as i32;
  let y1 = (sa * xf + ca * yf).round() as i32;
  (x1, y1)
}

fn step2((x, y, wx, wy): (i32, i32, i32, i32), cmd: &Action) -> (i32, i32, i32, i32) {
  match cmd {
    Action::North(val) => (x, y, wx, wy - val),
    Action::South(val) => (x, y, wx, wy + val),
    Action::West(val) => (x, y, wx - val, wy),
    Action::East(val) => (x, y, wx + val, wy),
    Action::Right(val) => {
      let (wx1, wy1) = rotate((wx, wy), *val);
      (x, y, wx1, wy1)
    }
    Action::Left(val) => {
      let (wx1, wy1) = rotate((wx, wy), -*val);
      (x, y, wx1, wy1)
    }
    Action::Forward(val) => (x + wx * val, y + wy * val, wx, wy),
  }
}

pub(crate) fn solution() {
  let commands: Vec<Action> = common::read_lines(&common::data_file(12))
    .unwrap()
    .iter()
    .map(|line| line.parse().unwrap())
    .collect();

  let (x1, y1, _) = commands.iter().fold((0, 0, 0), |acc, cmd| step1(acc, cmd));
  println!("Answer 1: {}", manhattan((x1, y1)));

  let (x2, y2, _, _) = commands
    .iter()
    .fold((0, 0, 10, -1), |acc, cmd| step2(acc, cmd));
  println!("Answer 2: {}", manhattan((x2, y2)));
}
