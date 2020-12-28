use crate::common;
use std::collections::BTreeMap;

fn get_first_non_sum(nums: &Vec<i64>, k: usize) -> Option<i64> {
    let mut window = BTreeMap::new();
    let n = nums.len();
    for i in 0..k {
        *window.entry(nums[i]).or_insert(1) += 1;
    }
    for i in k..n {
        let s = nums[i];
        let (mut it_l, mut it_r) = (window.iter(), window.iter().rev());
        let (mut l, mut r) = (it_l.next().unwrap(), it_r.next().unwrap());
        while l.0 != r.0 {
            let cur_sum = l.0 + r.0;
            if cur_sum < s {
                l = it_l.next().unwrap();
            } else if cur_sum > s {
                r = it_r.next().unwrap();
            } else {
                break;
            }
        }
        if l.0 == r.0 && *l.1 == 1 {
            return Some(s);
        }
        let to_remove = nums[i - k];
        window.entry(to_remove).and_modify(|e| *e -= 1);
        if window[&to_remove] == 0 {
            window.remove(&to_remove);
        }
        *window.entry(s).or_insert(1) += 1;
    }
    None
}

fn find_sum_range(nums: &Vec<i64>, s: i64) -> Option<(usize, usize)> {
    let (mut l, mut r) = (0, 0);
    let mut cur_sum = nums[0];
    let n = nums.len();
    while r < n - 1 && l < n - 1 {
        if cur_sum < s {
            r += 1;
            cur_sum += nums[r];
        } else if cur_sum > s {
            cur_sum -= nums[l];
            l += 1;
        } else {
            break;
        }
    }
    if l != r {
        Some((l, r))
    } else {
        None
    }
}

fn find_sum_range_agg(nums: &Vec<i64>, s: i64) -> Option<i64> {
    let sum_range = find_sum_range(&nums, s)?;
    let range_it = &nums[sum_range.0..=sum_range.1];
    let range_max = range_it.iter().max()?;
    let range_min = range_it.iter().min()?;
    Some(range_min + range_max)
}

pub(crate) fn solution() {
    const WINDOW_SIZE: usize = 25;
    let nums: Vec<i64> = common::read_integers(&common::data_file(9)).unwrap();
    let non_sum = get_first_non_sum(&nums, WINDOW_SIZE).unwrap();
    println!("First non-sum: {}", non_sum);
    println!("Sum range: {}", find_sum_range_agg(&nums, non_sum).unwrap());
}
