use crate::common;

const TREE: u8 = '#' as u8;

fn count_trees(rows: &Vec<String>, dx: usize, dy: usize) -> usize {
  let h = rows.len();
  if h == 0 {
    return 0;
  }
  let w = rows[0].len();
  let mut x: usize = 0;
  let mut res: usize = 0;
  for y in (0..h).step_by(dy) {
    res += (rows[y].as_bytes()[x % w] == TREE) as usize;
    x += dx;
  }
  res
}

pub(crate) fn solution() {
  let rows = common::read_lines(&common::data_file(3)).unwrap();
  let res1 = count_trees(&rows, 3, 1);
  println!("Num trees 1: {}", res1);

  let res2: usize = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
    .iter()
    .map(|(dx, dy)| count_trees(&rows, *dx, *dy))
    .product();
  println!("Num trees 2: {}", res2);
}
