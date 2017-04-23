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

pub fn nice_num<T: Display>(number: T) -> String {
  let mut result = String::new();
  let num_str = format!("{}", number);
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

  return result;
}

#[test]
fn nice_num_works() {
  assert_eq!(nice_num(1), "1");
  assert_eq!(nice_num(15), "15");
  assert_eq!(nice_num(153), "153");
  assert_eq!(nice_num(1534), "1,534");
  assert_eq!(nice_num(51534), "51,534");
  assert_eq!(nice_num(651534), "651,534");
  assert_eq!(nice_num(7651534), "7,651,534");
  assert_eq!(nice_num(87651534), "87,651,534");
  assert_eq!(nice_num(987651534), "987,651,534");
  assert_eq!(nice_num(1987651534), "1,987,651,534");
}
