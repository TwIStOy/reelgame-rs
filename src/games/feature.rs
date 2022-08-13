use std::{
  cmp,
  collections::HashMap,
  ops::{AddAssign, BitAnd},
  sync::Arc,
  thread,
};

use indicatif::{ProgressBar, ProgressStyle};
use lazy_static::lazy_static;

use crate::{
  count::CountResult,
  pay::PayTable,
  reel::{self, HitTable, Reel},
};

#[derive(Hash, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SituationKey {
  pub active: u8,
  pub rest_cnt: u32,
}
#[derive(Debug, Clone, Copy)]
pub struct SituationValue {
  pub pay_expect: f64,
}

pub struct FeatureGamePayCalc<'a> {
  pub record: HashMap<SituationKey, SituationValue>,
  reels: Vec<Reel>,
  pay_table: &'a PayTable,
}

fn to_active_arr(active: u8) -> Vec<bool> {
  let mut res: Vec<bool> = vec![false; 5];
  for i in 0..5 {
    res[i] = if active.bitand(1 << i) > 0 {
      true
    } else {
      false
    };
  }
  res
}

fn to_active_value(current: &Vec<i32>) -> u8 {
  let mut res = 0;

  for i in 0..5 {
    if current[i] == 1 {
      res |= 1 << i;
    }
  }

  res
}

fn dfs(
  index: usize,
  reels: &Vec<Reel>,
  active: u8,
  current: &mut Vec<i32>,
  pay_table: &PayTable,
  total_result: &mut f64,
  total_count: &mut usize,
  next: &mut HashMap<u8, usize>,
  bar: &ProgressBar,
  has_multi_threaded: bool,
) {
  if index == 5 {
    // calc pay
    let (p0, p1) = CountResult::feature(&current);

    let value = cmp::max(
      pay_table.pay(p0.icon, p0.count),
      pay_table.pay(p1.icon, p1.count),
    );

    *total_result += value as f64;
    *total_count += 1;
    *next.entry(to_active_value(&current)).or_insert(0) += 1;

    if !has_multi_threaded {
      bar.inc(1);
    }

    return;
  } else if (active & (1 << index)) > 0 {
    current[index] = 1;
    dfs(
      index + 1,
      &reels,
      active,
      current,
      &pay_table,
      total_result,
      total_count,
      next,
      bar,
      has_multi_threaded,
    );
  } else {
    let mut expect_count: u64 = 1;
    for i in index..5 {
      if (active & (1 << i)) == 0 {
        expect_count *= reels[i].len() as u64;
      };
    }

    if expect_count <= 10000000 || has_multi_threaded {
      for icon in &reels[index].icons {
        current[index] = *icon;
        dfs(
          index + 1,
          &reels,
          active,
          current,
          &pay_table,
          total_result,
          total_count,
          next,
          bar,
          has_multi_threaded,
        );
      }
    } else {
      let thread_count: usize = if reels[index].icons.len() % 20 == 0 {
        20
      } else {
        15
      };
      println!("Launch Multithread, thread: {}", thread_count);
      let mut threads = vec![];
      for thread_id in 0..thread_count {
        let mut my_current = current.clone();
        let my_reels = reels.clone();
        let my_pay_table = pay_table.clone();
        let mut my_total_result: f64 = 0.;
        let mut my_total_count: usize = 0;
        let mut my_next: HashMap<u8, usize> = HashMap::new();
        let my_bar = bar.clone();

        let t = thread::Builder::new()
          .name(format!("feature-{thread_id}"))
          .spawn(move || {
            for i in 0..my_reels[index].icons.len() {
              if i % thread_count != thread_id {
                continue;
              }
              my_current[index] = my_reels[index].icons[i];
              dfs(
                index + 1,
                &my_reels,
                active,
                &mut my_current,
                &my_pay_table,
                &mut my_total_result,
                &mut my_total_count,
                &mut my_next,
                &my_bar,
                true,
              );
            }

            (my_total_result, my_total_count, my_next)
          })
          .unwrap();

        threads.push(t);
      }
      for t in threads {
        let (some_result, some_count, some_next) = t.join().unwrap();
        *total_result += some_result;
        *total_count += some_count;

        for (k, v) in some_next {
          *next.entry(k).or_insert(0) += v;
        }
      }
    }
  }
}

impl<'a> FeatureGamePayCalc<'a> {
  pub fn new(reels: Vec<Reel>, pay_table: &'a PayTable) -> FeatureGamePayCalc {
    FeatureGamePayCalc {
      record: HashMap::new(),
      reels,
      pay_table,
    }
  }

  pub fn get(&mut self, active: u8, rest_cnt: u32) -> SituationValue {
    if let Some(v) = self.record.get(&SituationKey::new(active, rest_cnt)) {
      return v.clone();
    }

    let mut pay_total: f64 = 0.;
    let mut total_count: usize = 0;
    let mut arr: Vec<i32> = vec![0; 5];
    let mut next_map = HashMap::new();

    let mut expect_count: u64 = 1;
    for i in 0..5 {
      if active.bitand(1 << i) == 0 {
        expect_count *= self.reels[i].len() as u64;
      };
    }

    let bar = indicatif::ProgressBar::new(expect_count);
    println!("Calculate ({}, {})", active, rest_cnt);
    bar.set_style(
      ProgressStyle::with_template(
        "[{wide_bar:.cyan/blue}] {pos}/{len} {percent}% {per_sec} ({eta_precise})",
      )
      .unwrap()
      .progress_chars("#>-"));

    dfs(
      0,
      &self.reels,
      active,
      &mut arr,
      self.pay_table,
      &mut pay_total,
      &mut total_count,
      &mut next_map,
      &bar,
      false,
    );

    bar.finish();

    if rest_cnt > 0 {
      for (a, times) in next_map {
        pay_total += (times as f64) * self.get(a, rest_cnt - 1).pay_expect;
      }
    }

    let sv = SituationValue {
      pay_expect: pay_total / (total_count as f64),
    };
    self
      .record
      .insert(SituationKey::new(active, rest_cnt), sv.clone());
    sv
  }
}

impl SituationKey {
  fn new(active: u8, rest_cnt: u32) -> SituationKey {
    SituationKey { active, rest_cnt }
  }
}
