use std::{
  sync::{
    atomic::AtomicUsize,
    mpsc::{self, channel},
    Arc,
  },
  thread,
};

use indicatif::ProgressStyle;

mod pay;
mod reel;

struct HitResult {
  icon: usize,
  count: usize,
}

impl HitResult {
  pub fn new(v: &Vec<i32>) -> HitResult {
    let icon = v[0];
    let mut count = 0;
    for i in v {
      if *i == icon || (icon < 100 && *i == 0) {
        count += 1;
      } else {
        break;
      }
    }
    HitResult {
      icon: icon as usize,
      count,
    }
  }
}

fn main() {
  let pay: pay::PayTable = pay::PayTable::new("data/paytable.json");
  println!("{:?}", pay);

  let reels: Vec<reel::Reel> = vec![
    reel::Reel::new("data/reel1.json"),
    reel::Reel::new("data/reel2.json"),
    reel::Reel::new("data/reel3.json"),
    reel::Reel::new("data/reel4.json"),
    reel::Reel::new("data/reel5.json"),
  ];
  println!("{:?}", reels);
  println!("{:?}", reels[0].roll(-1));

  let hit = reel::HitTable::new("data/hits.json");
  println!("{:?}", hit);

  let bar = Arc::new(indicatif::ProgressBar::new(
    reels[0].len() as u64
      * reels[1].len() as u64
      * reels[2].len() as u64
      * reels[3].len() as u64
      * reels[4].len() as u64,
  ));
  bar.set_style(
    ProgressStyle::with_template(
      "[{wide_bar:.cyan/blue}] {human_pos}/{human_len} {per_sec} ({eta_precise})",
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
                  for res in hh.hit(&snapshot).iter().map(|x| HitResult::new(x))
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

  println!("total: {}", total);
}
