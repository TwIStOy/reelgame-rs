use std::sync::{atomic::AtomicUsize, mpsc::channel, Arc};

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
  let pay: Arc<pay::PayTable> =
    Arc::new(pay::PayTable::new("data/paytable.json"));
  println!("{:?}", pay);

  let reels: Arc<Vec<reel::Reel>> = Arc::new(vec![
    reel::Reel::new("data/reel1.json"),
    reel::Reel::new("data/reel2.json"),
    reel::Reel::new("data/reel3.json"),
    reel::Reel::new("data/reel4.json"),
    reel::Reel::new("data/reel5.json"),
  ]);
  println!("{:?}", reels);
  println!("{:?}", reels[0].roll(-1));

  let hit = Arc::new(reel::HitTable::new("data/hits.json"));
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
  let pool = threadpool::ThreadPool::new(8);
  let total: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
  for (a, b, c, d, e) in itertools::iproduct!(
    0..reels[0].len(),
    0..reels[1].len(),
    0..reels[2].len(),
    0..reels[3].len(),
    0..reels[4].len()
  ) {
    let tt = total.clone();
    let rr = reels.clone();
    let hh = hit.clone();
    let pp = pay.clone();
    let bb = bar.clone();

    pool.execute(move || {
      let snapshot = vec![
        rr[0].roll(a as i32),
        rr[1].roll(b as i32),
        rr[2].roll(c as i32),
        rr[3].roll(d as i32),
        rr[4].roll(e as i32),
      ];
      for res in hh.hit(&snapshot).iter().map(|x| HitResult::new(x)) {
        tt.fetch_add(
          pp.pay(res.icon, res.count) as usize,
          std::sync::atomic::Ordering::Relaxed,
        );
      }
      bb.inc(1);
    });
  }

  pool.join();
  bar.finish();

  println!("total: {}", total.load(std::sync::atomic::Ordering::SeqCst));
}
