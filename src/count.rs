#[derive(Debug, Clone)]
pub struct CountResult {
  pub icon: usize,
  pub count: usize,
}

impl CountResult {
  pub fn base(v: &[i32]) -> CountResult {
    let mut icon = v[0];
    for i in 0..5 {
      if v[i] > 0 {
        icon = v[i];
        break;
      }
    }

    if icon >= 11 || icon == 0 {
      CountResult { icon: 0, count: 0 }
    } else {
      let mut count = 0;
      for i in v {
        if *i == icon || (icon < 100 && *i == 0) {
          count += 1;
        } else {
          break;
        }
      }
      let res = CountResult {
        icon: icon as usize,
        count,
      };

      res
    }
  }

  pub fn feature_mode_test_pic01_as_wild(v: &[i32]) -> CountResult {
    // select another icon
    let mut icon = 0;
    for i in 0..5 {
      if v[i] > 1 {
        icon = v[i];
        break;
      }
    }

    if icon >= 11 || icon == 0 {
      CountResult { icon: 0, count: 0 }
    } else {
      let mut count = 0;
      for i in v {
        if *i == icon || (icon < 100 && (*i == 0 || *i == 1)) {
          count += 1;
        } else {
          break;
        }
      }
      CountResult {
        icon: icon as usize,
        count,
      }
    }
  }

  pub fn feature_mod_test_pic01(v: &[i32]) -> CountResult {
    let mut count = 0;
    let mut has_one = false;
    for i in v {
      if *i == 1 || *i == 0 {
        count += 1;
        if *i == 1 {
          has_one = true;
        }
      } else {
        break;
      }
    }
    if has_one {
      CountResult { icon: 1, count }
    } else {
      CountResult { icon: 1, count: 0 }
    }
  }

  pub fn feature(v: &[i32]) -> (CountResult, CountResult) {
    let pic01_as_wild = CountResult::feature_mode_test_pic01_as_wild(&v);
    let pic01 = CountResult::feature_mod_test_pic01(&v);

    (pic01, pic01_as_wild)
  }
}
