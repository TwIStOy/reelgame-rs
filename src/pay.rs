use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{
  collections::HashMap,
  fs::File,
  io::{BufRead, BufReader},
};

lazy_static! {
  pub static ref INT_ID: HashMap<&'static str, i32> = HashMap::from([
    ("WILD", 0),
    ("PIC01", 01),
    ("PIC02", 02),
    ("PIC03", 03),
    ("PIC04", 04),
    ("PIC05", 05),
    ("PIC06", 06),
    ("PIC07", 07),
    ("PIC08", 08),
    ("PIC09", 09),
    ("PIC10", 10),
    ("PIC11", 11),
    ("PIC12", 12),
    ("PIC13", 13),
    ("PIC14", 14),
    ("PIC15", 15),
    ("SCAT1", 100),
    ("SCAT2", 101),
  ]);
}

#[derive(Debug, Serialize, Deserialize)]
struct PayTableInterface(HashMap<String, Vec<i32>>);

#[derive(Debug)]
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
    self.data[index][count]
  }
}
