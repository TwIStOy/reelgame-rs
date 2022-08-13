use super::card::INT_ID;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::BufReader};

#[derive(Debug, Serialize, Deserialize)]
struct PayTableInterface(HashMap<String, Vec<i32>>);

#[derive(Debug, Clone)]
pub struct PayTable {
  pub data: Vec<Vec<i32>>,
}

impl PayTable {
  pub fn new(filename: &str) -> PayTable {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut res = PayTable { data: Vec::new() };
    res.data.resize(200, vec![]);

    let data: PayTableInterface = serde_json::from_reader(reader).unwrap();

    for (k, v) in &data.0 {
      let id = INT_ID.get(k.as_str()).unwrap();
      res.data[*id as usize] = v.clone();
    }

    res
  }

  pub fn pay(&self, index: usize, count: usize) -> i32 {
    if count > 0 {
      self.data[index][count - 1]
    } else {
      0
    }
  }
}
