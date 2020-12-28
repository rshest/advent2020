use crate::common;
use std::str::FromStr;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Seat {
  Floor,
  Vacant,
  Busy,
}

type SeatPlan = Vec<Vec<Seat>>;

impl FromStr for Seat {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "." => Ok(Seat::Floor),
      "L" => Ok(Seat::Vacant),
      "#" => Ok(Seat::Busy),
      _ => Err(()),
    }
  }
}

const OFFS: [(i32, i32); 8] = [
  (-1, -1),
  (0, -1),
  (1, -1),
  (-1, 0),
  (1, 0),
  (-1, 1),
  (0, 1),
  (1, 1),
];

fn get_seat(seats: &SeatPlan, (x, y): (i32, i32)) -> Option<Seat> {
  let (h, w) = (seats.len() as i32, seats[0].len() as i32);
  if x >= 0 && x < w && y >= 0 && y < h {
    Some(seats[y as usize][x as usize])
  } else {
    None
  }
}

fn get_neighbor_counts(seats: &SeatPlan) -> Vec<Vec<u32>> {
  let (h, w) = (seats.len() as i32, seats[0].len() as i32);
  let mut res = vec![vec![0; w as usize]; h as usize];
  for y in 0..h {
    for x in 0..w {
      res[y as usize][x as usize] = OFFS
        .iter()
        .filter_map(|(dx, dy)| get_seat(&seats, (x + dx, y + dy)))
        .map(|s| (s == Seat::Busy) as u32)
        .sum();
    }
  }
  res
}

fn get_visible_neighbor_counts(seats: &SeatPlan) -> Vec<Vec<u32>> {
  let (h, w) = (seats.len() as usize, seats[0].len() as usize);
  let mut res = vec![vec![0; w]; h];
  let mut can_see = vec![vec![false; w]; h];

  for (dx, dy) in OFFS.iter() {
    for j in 0..h {
      for i in 0..w {
        let x: usize = if *dx <= 0 { i } else { w - i - 1 };
        let y: usize = if *dy <= 0 { j } else { h - j - 1 };
        let pos = (x as i32 + dx, y as i32 + dy);
        let visible = match get_seat(&seats, pos) {
          Some(Seat::Floor) => can_see[pos.1 as usize][pos.0 as usize],
          Some(Seat::Busy) => true,
          Some(Seat::Vacant) => false,
          _ => false,
        };
        can_see[y][x] = visible;
        res[y][x] += visible as u32;
      }
    }
  }
  res
}

fn step(
  seats: &SeatPlan,
  new_seats: &mut SeatPlan,
  neighbors_fn: fn(&SeatPlan) -> Vec<Vec<u32>>,
  max_occupied: u32,
) -> (u32, u32) {
  let (h, w) = (seats.len() as usize, seats[0].len() as usize);
  let neighbor_counts = neighbors_fn(seats);
  let mut num_changed = 0;
  let mut total_occupied = 0;
  for y in 0..h {
    for x in 0..w {
      let n = neighbor_counts[y][x];
      let status = match seats[y][x] {
        Seat::Vacant if n == 0 => Seat::Busy,
        Seat::Busy if n >= max_occupied => Seat::Vacant,
        val => val,
      };
      new_seats[y][x] = status;
      num_changed += (status != seats[y][x]) as u32;
      total_occupied += (status == Seat::Busy) as u32;
    }
  }
  (num_changed, total_occupied)
}

fn iter_seats(
  seats: &SeatPlan,
  neighbors_fn: fn(&SeatPlan) -> Vec<Vec<u32>>,
  max_occupied: u32,
) -> u32 {
  let mut s1 = seats.clone();
  let mut s2 = seats.clone();
  let mut flip = true;
  loop {
    let (r1, r2) = if flip { (&s1, &mut s2) } else { (&s2, &mut s1) };
    let (changed, occupied) = step(r1, r2, neighbors_fn, max_occupied);
    if changed == 0 {
      return occupied;
    }
    flip = !flip;
  }
}

pub(crate) fn solution() {
  let seats: SeatPlan = common::read_lines(&common::data_file(11))
    .unwrap()
    .iter()
    .map(|line| {
      line
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect()
    })
    .collect();
  println!("Answer 1: {}", iter_seats(&seats, get_neighbor_counts, 4));
  println!(
    "Answer 2: {}",
    iter_seats(&seats, get_visible_neighbor_counts, 5)
  );
}
