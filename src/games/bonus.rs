use std::{cmp, collections::HashMap};

use itertools::Itertools;
use lazy_static::lazy_static;

use crate::reel::{HitTable, ReelSnapshot};
use indicatif::{ProgressBar, ProgressStyle};

use super::feature::{SituationKey, SituationValue};

lazy_static! {
  pub static ref BONUS: Vec<i32> = vec![0, 0, 0, 1, 2, 4];
  static ref CANS: Vec<usize> = vec![1, 1, 2, 2, 2, 3, 3, 3, 4, 5, 5, 5];
}

fn game_pay_once(cnt: usize) -> (f64, usize, usize) {
  let mut sum: usize = 0;
  let mut total_cnt: usize = 0;
  let mut emit_feature_game: usize = 0;

  for permutation in (*CANS).iter().combinations(cnt) {
    let mut gold_count = 0;

    for icon in permutation {
      sum += cmp::min(*icon, 4);

      if *icon == 5 {
        gold_count += 1;
      }
    }

    if gold_count >= 2 {
      // feature game
      emit_feature_game += 1;
    }

    total_cnt += 1;
  }

  let v = (sum as f64) / (total_cnt as f64);

  println!("BONUS PAY E: {}", v);
  (v, emit_feature_game, total_cnt)
}

lazy_static! {
  pub static ref BONUS_PAY_E: (f64, usize, usize) = game_pay_once(5);
}

pub fn bonus_game_pay(
  snapshot: &Vec<ReelSnapshot>,
  FEATURE_PAY_E: f64,
) -> (f64, f64) {
  let mut cnt = 0;
  for reel in snapshot {
    for x in &reel.icons {
      if *x == 101 {
        cnt += 1;
      }
    }
  }

  if cnt > 5 {
    cnt = 5;
  }
  let bonus = BONUS[cnt];
  let enter_bonus_game: f64 = if cnt >= 3 { 1. } else { 0. };

  (
    (bonus as f64) * (BONUS_PAY_E.0),
    enter_bonus_game * (BONUS_PAY_E.1 as f64) / (BONUS_PAY_E.2 as f64)
      * FEATURE_PAY_E,
  )
}
