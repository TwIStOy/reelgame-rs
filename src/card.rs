use lazy_static::lazy_static;
use std::collections::HashMap;
use super::reel::ReelSnapshot;

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

pub fn possible_same(left: &ReelSnapshot, right: &ReelSnapshot) -> bool {
  for l in &left.icons {
    for r in &right.icons {
      if same_card(*l, *r) {
        return true;
      }
    }
  }

  false
}
