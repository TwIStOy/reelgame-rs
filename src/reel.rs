use std::{fs::File, io::BufReader};

use super::pay::INT_ID;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Reel {
  pub icons: Vec<i32>,
}

#[derive(Debug)]
pub struct ReelSnapshot {
  pub icons: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ReelInterface(Vec<String>);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HitTable(Vec<Vec<usize>>);

impl Reel {
  pub fn new(filename: &str) -> Reel {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let interface: ReelInterface = serde_json::from_reader(reader).unwrap();

    Reel {
      icons: interface
        .0
        .iter()
        .map(|x| *INT_ID.get(x.as_str()).unwrap())
        .collect(),
    }
  }

  pub fn roll(&self, index: i32) -> ReelSnapshot {
    ReelSnapshot {
      icons: vec![
        self.icons
          [(index - 1 + self.icons.len() as i32) as usize % self.icons.len()],
        self.icons[(index) as usize % self.icons.len()],
        self.icons
          [(index + 1 + self.icons.len() as i32) as usize % self.icons.len()],
      ],
    }
  }

  pub fn len(&self) -> usize {
    self.icons.len()
  }
}

impl HitTable {
  pub fn new(filename: &str) -> HitTable {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let interface: HitTable = serde_json::from_reader(reader).unwrap();

    interface
  }

  pub fn hit(&self, snapshots: &Vec<ReelSnapshot>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = Vec::new();
    for line in &self.0 {
      res.push(
        line
          .iter()
          .zip(snapshots)
          .map(|(index, snap)| snap.icons[*index])
          .collect(),
      );
    }
    res
  }
}
