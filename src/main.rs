mod pay;
mod reel;

struct HitResult {
  icon: i32,
  count: i32,
}

fn main() {
  let pay = pay::PayTable::new("data/paytable.json");
  println!("{:?}", pay);

  let reels = vec![
    reel::Reel::new("data/reel1.json"),
    reel::Reel::new("data/reel2.json"),
    reel::Reel::new("data/reel3.json"),
    reel::Reel::new("data/reel4.json"),
    reel::Reel::new("data/reel5.json"),
  ];
  println!("{:?}", reels);
  println!("{:?}", reels[0].roll(-1));
}
