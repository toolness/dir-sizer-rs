use std::fmt::Display;

pub fn strip_commas(string: &str) -> String {
  let mut result = String::new();

  for c in string.chars() {
    if c != ',' {
      result.push(c);
    }
  }

  return result;
}

#[test]
fn strip_commas_works() {
  assert_eq!(strip_commas("100"), "100");
  assert_eq!(strip_commas("1,000"), "1000");
}

pub trait WithCommas {
  fn with_commas(self) -> String;
}

impl<T: Display> WithCommas for T {
  fn with_commas(self) -> String {
    let mut result = String::new();
    let num_str = format!("{}", self);
    let num_digits = num_str.len();
    let mut comma_counter = 0;

    if num_str.len() > 3 {
      comma_counter = ((num_digits / 3 + 1) * 3 - num_digits) % 3;
    }

    for (i, c) in num_str.chars().enumerate() {
      let is_last_digit = i == num_digits - 1;
      result.push(c);
      comma_counter += 1;
      if comma_counter == 3 && !is_last_digit {
        result.push(',');
        comma_counter = 0;
      }
    }

    result
  }
}

#[test]
fn with_commas_works() {
  assert_eq!(1.with_commas(), "1");
  assert_eq!(15.with_commas(), "15");
  assert_eq!(153.with_commas(), "153");
  assert_eq!(1534.with_commas(), "1,534");
  assert_eq!(51534.with_commas(), "51,534");
  assert_eq!(651534.with_commas(), "651,534");
  assert_eq!(7651534.with_commas(), "7,651,534");
  assert_eq!(87651534.with_commas(), "87,651,534");
  assert_eq!(987651534.with_commas(), "987,651,534");
  assert_eq!(1987651534.with_commas(), "1,987,651,534");
}
