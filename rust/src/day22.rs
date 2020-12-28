use crate::common;

use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;
use std::hash::{Hash, Hasher};

type Deck = VecDeque<usize>;

fn parse_deck(text: &str) -> Deck {
  text
    .split("\n")
    .skip(1)
    .filter_map(|s| s.parse::<usize>().ok())
    .collect()
}

fn step(da: &mut Deck, db: &mut Deck, recurse: bool) -> Option<usize> {
  if da.len() == 0 {
    return Some(1);
  } else if db.len() == 0 {
    return Some(0);
  }
  let (a, b) = (da.pop_front().unwrap(), db.pop_front().unwrap());
  let a_wins = if recurse && a <= da.len() && b <= db.len() {
    let mut da1 = da.iter().take(a).map(|x| *x).collect();
    let mut db1 = db.iter().take(b).map(|x| *x).collect();
    game(&mut da1, &mut db1, true) == 0
  } else {
    a > b
  };

  if a_wins {
    da.push_back(a);
    da.push_back(b);
  } else {
    db.push_back(b);
    db.push_back(a);
  }
  None
}

fn game(da: &mut Deck, db: &mut Deck, recurse: bool) -> usize {
  let (mut visited_a, mut visited_b) = (HashSet::new(), HashSet::new());
  loop {
    let (mut hasher1, mut hasher2) = (DefaultHasher::new(), DefaultHasher::new());
    da.hash(&mut hasher1);
    db.hash(&mut hasher2);
    let (h1, h2) = (hasher1.finish(), hasher2.finish());
    if visited_a.contains(&h1) || visited_b.contains(&h2) {
      return 0;
    }
    visited_a.insert(h1);
    visited_b.insert(h2);

    match step(da, db, recurse) {
      Some(x) => return x,
      None => (),
    }
  }
}

fn get_score(deck: &Deck) -> usize {
  deck
    .iter()
    .enumerate()
    .map(|(i, x)| x * (deck.len() - i))
    .sum()
}

pub(crate) fn solution() {
  let decks: Vec<Deck> = fs::read_to_string(&common::data_file(22))
    .unwrap()
    .split("\n\n")
    .map(|s| parse_deck(s))
    .collect();

  let (mut da1, mut db1) = (decks[0].clone(), decks[1].clone());
  let winner1 = game(&mut da1, &mut db1, false);
  println!(
    "Answer 1: {}",
    get_score(if winner1 == 0 { &da1 } else { &db1 })
  );

  let (mut da2, mut db2) = (decks[0].clone(), decks[1].clone());
  let winner2 = game(&mut da2, &mut db2, true);
  println!(
    "Answer 2: {}",
    get_score(if winner2 == 0 { &da2 } else { &db2 })
  );
}
