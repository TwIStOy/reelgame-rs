use std::{sync::Arc, thread};

use indicatif::ProgressStyle;
use itertools::Itertools;
use reel::ReelSnapshot;

mod pay;
mod reel;

#[derive(Debug, Clone)]
struct HitResult {
  pub icon: usize,
  pub count: usize,
}

impl HitResult {
  pub fn new(v: &[i32]) -> HitResult {
    let mut icon = v[0];
    for i in 0..5 {
      if v[i] > 0 {
        icon = v[i];
        break;
      }
    }

    if icon >= 11 || icon == 0 {
      HitResult { icon: 0, count: 0 }
    } else {
      let mut count = 0;
      for i in v {
        if *i == icon || (icon < 100 && *i == 0) {
          count += 1;
        } else {
          break;
        }
      }
      let res = HitResult {
        icon: icon as usize,
        count,
      };

      // println!("check: {:?}, res: {:?}", v, res);
      res
    }
  }
}

fn same_card(left: i32, right: i32) -> bool {
  if left == right {
    true
  } else if left == 0 {
    right < 100
  } else if right == 0 {
    left < 100
  } else {
    false
  }
}

fn no_same(left: &ReelSnapshot, right: &ReelSnapshot) -> bool {
  for l in &left.icons {
    for r in &right.icons {
      if same_card(*l, *r) {
        return true;
      }
    }
  }

  false
}

fn main() {
  let pay: pay::PayTable = pay::PayTable::new("data/paytable.json");
  let reels: Vec<reel::Reel> = vec![
    reel::Reel::new("data/reel1.json"),
    reel::Reel::new("data/reel2.json"),
    reel::Reel::new("data/reel3.json"),
    reel::Reel::new("data/reel4.json"),
    reel::Reel::new("data/reel5.json"),
  ];
  let hit = reel::HitTable::new("data/hits.json");

  if false {
    println!("== test ==");
    let snapshot = vec![
      reels[0].roll(34),
      reels[1].roll(5),
      reels[2].roll(67),
      reels[3].roll(61),
      reels[4].roll(38),
    ];
    println!("{:?}", snapshot);
    let hits = hit.hit(&snapshot);
    let results = hits.iter().map(|x| HitResult::new(x)).collect_vec();
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
    let bar = Arc::new(indicatif::ProgressBar::new(total_cost));
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

      let handler = thread::Builder::new()
        .name(format!("worker-{i}"))
        .spawn(move || {
          let mut result: usize = 0;
          for a in 0..rr[0].len() {
            if a % thread_count != i {
              continue;
            }
            let s0 = rr[0].roll(a as i32);
            for b in 0..rr[1].len() {
              let s1 = rr[1].roll(b as i32);

              if no_same(&s0, &s1) {
                bb.inc((rr[4].len() * rr[3].len() * rr[2].len()) as u64);
                continue;
              }

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

                    for res in hh
                      .hit(&snapshot)
                      .iter()
                      .map(|x| HitResult::new(x))
                      .filter(|x| x.count >= 2)
                    {
                      result += pp.pay(res.icon, res.count) as usize;
                    }
                  }
                }
                bb.inc((rr[4].len() * rr[3].len()) as u64);
              }
            }
          }
          result
        })
        .unwrap();

      pool.push(handler);
    }

    let mut total = 0;
    for t in pool {
      total += t.join().unwrap();
    }

    bar.finish();

    println!(
      "total: {}, RTP: {}",
      total,
      (total as f64) / (total_cost as f64)
    );
  }
}
