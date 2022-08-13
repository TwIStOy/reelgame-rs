use std::{
  collections::HashMap,
  sync::Arc,
  thread::{self, JoinHandle},
};

mod card;
mod count;
mod games;
mod pay;
mod reel;

use card::possible_same;
use count::CountResult;
use games::{base::base_game_pay, bonus};
use games::{
  bonus::bonus_game_pay,
  feature::{self, SituationKey, SituationValue},
};
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use pay::PayTable;
use reel::HitTable;

fn new_base_worker_thread(
  pp: PayTable,
  hh: HitTable,
  rr: Vec<reel::Reel>,
  bb: ProgressBar,
  FEATURE_PAY_E: f64,
  i: usize,
  thread_count: usize,
) -> JoinHandle<(f64, f64, f64)> {
  thread::Builder::new()
    .name(format!("worker-{i}"))
    .spawn(move || {
      let mut base_result: f64 = 0.;
      let mut bonus_result: f64 = 0.;
      let mut feature_result: f64 = 0.;
      for a in 0..rr[0].len() {
        if a % thread_count != i {
          continue;
        }
        for b in 0..rr[1].len() {
          for c in 0..rr[2].len() {
            for d in 0..rr[3].len() {
              for e in 0..rr[4].len() {
                let snapshot = vec![
                  rr[0].roll(a as i32),
                  rr[1].roll(b as i32),
                  rr[2].roll(c as i32),
                  rr[3].roll(d as i32),
                  rr[4].roll(e as i32),
                ];

                base_result += base_game_pay(&snapshot, &&pp) as f64;
                let (bonus_e, feature_e) =
                  bonus_game_pay(&snapshot, FEATURE_PAY_E);
                bonus_result += bonus_e;
                feature_result += feature_e;
              }
            }
            // bb.inc((rr[4].len() * rr[3].len()) as u64);
          }
        }
      }
      (base_result, bonus_result, feature_result)
    })
    .unwrap()
}

fn main() {
  let pay: pay::PayTable = pay::PayTable::new("data/paytable.json");
  let reels: Vec<reel::Reel> = vec![
    reel::Reel::new("data/base/reel1.json"),
    reel::Reel::new("data/base/reel2.json"),
    reel::Reel::new("data/base/reel3.json"),
    reel::Reel::new("data/base/reel4.json"),
    reel::Reel::new("data/base/reel5.json"),
  ];
  let hit = reel::HitTable::new("data/hits.json");
  let feature_reels: Vec<reel::Reel> = vec![
    reel::Reel::new("data/feature/reel1.json"),
    reel::Reel::new("data/feature/reel2.json"),
    reel::Reel::new("data/feature/reel3.json"),
    reel::Reel::new("data/feature/reel4.json"),
    reel::Reel::new("data/feature/reel5.json"),
  ];
  let mut feature_pay = feature::FeatureGamePayCalc::new(feature_reels, &pay);

  // println!("---- test: {:?}", CountResult::feature(&vec![0, 0, 3, 1, 3]));

  println!("== BONUS_PAY_E: {:?} ==", *games::bonus::BONUS_PAY_E);

  let FEATURE_PAY_E = feature_pay.get(0, 5).pay_expect;
  println!("=== FEATURE_PAY_E: {} ===", FEATURE_PAY_E);

  if false {
    println!("== test ==");
    // println!("== BONUS PAY E: {} ==", games::bonus::BONUS_PAY_E.0);
    let snapshot = vec![
      reels[0].roll(34),
      reels[1].roll(5),
      reels[2].roll(67),
      reels[3].roll(61),
      reels[4].roll(38),
    ];
    println!("{:?}", snapshot);
    let hits = hit.hit(&snapshot);
    let results = hits.iter().map(|x| CountResult::base(x)).collect_vec();
    println!("{:?}", hits);
    println!("{:?}", results);
    println!(
      "{:?}",
      results
        .iter()
        .map(|x| pay.pay(x.icon, x.count))
        .collect_vec()
    );
  } else {
    let total_cost = reels[0].len() as u64
      * reels[1].len() as u64
      * reels[2].len() as u64
      * reels[3].len() as u64
      * reels[4].len() as u64;
    let bar = indicatif::ProgressBar::new(total_cost);
    bar.set_style(
    ProgressStyle::with_template(
      "[{wide_bar:.cyan/blue}] {pos}/{len} {percent}% {per_sec} ({eta_precise})",
    )
    .unwrap()
    .progress_chars("#>-"),
  );

    let thread_count = 20;
    let mut pool = vec![];
    for i in 0..thread_count {
      // make threads
      let pp = pay.clone();
      let hh = hit.clone();
      let rr = reels.clone();
      let bb = bar.clone();

      pool.push(new_base_worker_thread(
        pp,
        hh,
        rr,
        bb,
        FEATURE_PAY_E,
        i,
        thread_count,
      ));
    }

    let mut base_total = 0.;
    let mut bonus_total = 0.;
    let mut feature_total = 0.;
    for t in pool {
      let (base, bonus, feature) = t.join().unwrap();
      base_total += base;
      bonus_total += bonus;
      feature_total += feature;
    }

    bar.finish();

    println!(
      "RTP: {:.4}%, base RTP: {:.4}%, bonus RTP: {:.4}%, feature RTP: {:.4}%",
      (base_total + bonus_total + feature_total) / (total_cost as f64) * 100f64,
      (base_total) / (total_cost as f64) * 100f64,
      (bonus_total) / (total_cost as f64) * 100f64,
      (feature_total) / (total_cost as f64) * 100f64
    );
  }
}
