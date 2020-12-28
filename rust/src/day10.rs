use crate::common;

pub(crate) fn solution() {
  let mut nums: Vec<i64> = common::read_integers(&common::data_file(10)).unwrap();
  nums.sort();
  nums.insert(0, 0);
  nums.push(nums.iter().max().unwrap() + 3);
  let n = nums.len();
  let mut diffs: Vec<i64> = Vec::new();
  for i in 1..n {
    diffs.push(nums[i] - nums[i - 1]);
  }
  let diffs1: i64 = diffs.iter().map(|x| (*x == 1) as i64).sum();
  let diffs3: i64 = diffs.iter().map(|x| (*x == 3) as i64).sum();
  println!("Sum 1/3 diffs: {}", diffs1 * diffs3);
  fn count_arrangements(nums: &Vec<i64>) -> i64 {
    fn rec(nums: &Vec<i64>, idx: usize, counts: &mut Vec<i64>) -> i64 {
      let n = nums.len();
      if idx == n - 1 {
        return 1;
      }
      if counts[idx] >= 0 {
        return counts[idx];
      }
      let mut res = 0;
      let mut i = idx;
      while i < n - 1 {
        i += 1;
        let diff = nums[i] - nums[idx];
        if diff <= 3 {
          res += rec(nums, i, counts);
        } else {
          break;
        }
      }
      counts[idx] = res;
      res
    }
    let mut counts: Vec<i64> = Vec::new();
    counts.resize(nums.len(), -1);
    rec(nums, 0, &mut counts)
  }
  println!("Number of arrangements: {}", count_arrangements(&nums));
}
