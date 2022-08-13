use crate::{count::CountResult, pay::PayTable, reel::ReelSnapshot};

pub fn base_game_pay(
  snapshot: &Vec<ReelSnapshot>,
  pay_table: &PayTable,
) -> usize {
  let first_line = snapshot.iter().map(|x| x.icons[0]).collect::<Vec<i32>>();
  let res = CountResult::base(&first_line);
  pay_table.pay(res.icon, res.count) as usize
}
