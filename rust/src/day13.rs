use crate::common;

fn part1(periods: &Vec<(u64, u64)>, ts: u64) -> u64 {
  let mut min_diff = None;
  let mut res = 0;
  for (p, _) in periods {
    let d = p - ts % p;
    if min_diff == None || d < min_diff.unwrap() {
      min_diff = Some(d);
      res = d * p;
    }
  }
  res
}

/*
// The actual, initial version that was used for submission, this takes ~6min to execute
fn part2(periods: &Vec<(u64, u64)>) -> u64 {
  let (mut p0, i0) = periods[0];
  // Assuming here that the largest number has an offset
  // which is present as another number and they are also coprime.
  // This is something I noticed for my data, but should be safe for any input.
  p0 *= i0;
  let mut k = 1;
  loop {
      let ts = k * p0 - i0;
      let mut found = true;
      for j in 1..periods.len() {
          let (p, i) = periods[j];
          if (ts + i) % p != 0 {
              found = false;
              break;
          }
      }
      if found {
          return ts;
      }
      k += 1;
      if k % 1000000000 == 0 {
          println!("Processed timestamp: {}", ts);
      }
  }
}
*/

fn part2(periods: &Vec<(u64, u64)>) -> u64 {
  let (mut step, mut ts) = periods[0];
  let mut k = 0;
  loop {
    let (p, i) = periods[k + 1];
    if (ts + i) % p == 0 {
      step *= p;
      k += 1;
      if k == periods.len() - 1 {
        return ts;
      }
    }
    ts += step;
  }
}

pub(crate) fn solution() {
  let lines = common::read_lines(&common::data_file(13)).unwrap();
  let ts: u64 = lines[0].parse::<u64>().unwrap();
  let mut periods: Vec<(u64, u64)> = lines[1]
    .split(",")
    .map(|p| p.parse::<u64>())
    .enumerate()
    .filter_map(|(i, p)| match p {
      Ok(val) => Some((val, i as u64)),
      _ => None,
    })
    .collect();
  periods.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
  println!("Answer 1: {:?}", part1(&periods, ts));
  println!("Answer 2: {:?}", part2(&periods));
}
