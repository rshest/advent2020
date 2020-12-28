use crate::common;
use std::collections::HashSet;
use std::fs;

#[derive(Debug, PartialEq)]
struct Category {
  name: String,
  ranges: Vec<(i64, i64)>,
}

#[derive(Debug, PartialEq)]
struct Notes {
  categories: Vec<Category>,
  own_ticket: Vec<i64>,
  tickets: Vec<Vec<i64>>,
}

type Err = std::num::ParseIntError;

fn parse_notes(data: &str) -> Notes {
  fn parse_range(s: &str) -> Result<(i64, i64), Err> {
    let parts: Vec<&str> = s.split('-').collect();
    Ok((parts[0].parse()?, parts[1].parse()?))
  }

  fn parse_category(line: &str) -> Result<Category, Err> {
    let parts: Vec<&str> = line.split(": ").collect();
    let ranges = parts[1]
      .split(" or ")
      .map(|s| parse_range(s).unwrap())
      .collect();
    Ok(Category {
      name: parts[0].to_owned(),
      ranges: ranges,
    })
  }

  fn parse_ticket(line: &str) -> Result<Vec<i64>, Err> {
    line.split(',').map(|x| x.parse::<i64>()).collect()
  }

  let parts: Vec<Vec<&str>> = data
    .split("\n\n")
    .map(|s| s.trim().split('\n').collect())
    .collect();
  Notes {
    categories: parts[0]
      .iter()
      .map(|line| parse_category(line).unwrap())
      .collect(),
    own_ticket: parse_ticket(parts[1][1]).unwrap(),
    tickets: parts[2][1..]
      .iter()
      .map(|line| parse_ticket(line).unwrap())
      .collect(),
  }
}

fn is_valid_category(category: &Category, val: &i64) -> bool {
  category.ranges.iter().any(|(a, b)| a <= val && val <= b)
}

fn get_invalid_entries(notes: &Notes) -> Vec<(usize, i64)> {
  let mut res = Vec::new();
  for (i, t) in notes.tickets.iter().enumerate() {
    let invalid = t
      .iter()
      .filter(|n| !notes.categories.iter().any(|cat| is_valid_category(cat, n)));
    for x in invalid {
      res.push((i, *x));
    }
  }
  res
}

fn get_valid_tickets(notes: &Notes, invalid_entries: &Vec<(usize, i64)>) -> Vec<Vec<i64>> {
  let invalid_tickets: HashSet<usize> = invalid_entries.into_iter().map(|x| x.0).collect();
  notes
    .tickets
    .iter()
    .enumerate()
    .filter_map(|(i, t)| {
      if invalid_tickets.contains(&i) {
        None
      } else {
        Some(t.clone())
      }
    })
    .collect()
}

fn get_category_ordering(notes: &Notes) -> Vec<usize> {
  let n = notes.categories.len();
  let mut candidates = vec![vec![true; n]; n];
  let mut cands_left = n * n;

  // sift out invalid tickets
  for ticket in &notes.tickets {
    for (i, x) in ticket.iter().enumerate() {
      for j in 0..n {
        if !is_valid_category(&notes.categories[j], x) {
          cands_left -= candidates[j][i] as usize;
          candidates[j][i] = false;
        }
      }
    }
  }

  // iteratively subtract single-element candidate sets from others
  while cands_left > n {
    for k in 0..n {
      let mut count = 0;
      let mut last = 0;
      for i in 0..n {
        if candidates[k][i] {
          count += 1;
          last = i;
          if count > 1 {
            break;
          }
        }
      }
      if count == 1 {
        for i in 0..n {
          if i != k {
            cands_left -= candidates[i][last] as usize;
            candidates[i][last] = false;
          }
        }
      }
    }
  }

  let mut res = Vec::new();
  for i in 0..n {
    for j in 0..n {
      if candidates[j][i] {
        res.push(j);
      }
    }
  }
  res
}

fn part2(notes: &Notes) -> i64 {
  get_category_ordering(&notes)
    .iter()
    .enumerate()
    .filter_map(|(i, idx)| {
      if notes.categories[*idx].name.starts_with("departure") {
        Some(notes.own_ticket[i])
      } else {
        None
      }
    })
    .product()
}

pub(crate) fn solution() {
  let notes = parse_notes(&fs::read_to_string(&common::data_file(16)).unwrap());
  let invalid_entries = get_invalid_entries(&notes);
  println!(
    "Answer 1: {:?}",
    invalid_entries.iter().map(|(_, x)| x).sum::<i64>()
  );
  let valid_tickets = get_valid_tickets(&notes, &invalid_entries);
  let valid_notes = Notes {
    categories: notes.categories,
    tickets: valid_tickets,
    own_ticket: notes.own_ticket,
  };
  println!("Answer 2: {:?}", part2(&valid_notes));
}
