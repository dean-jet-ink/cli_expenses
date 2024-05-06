use std::{io, str::FromStr};

use chrono::NaiveDate;

pub struct DateCommand {}

impl DateCommand {
  pub fn input() -> NaiveDate {
    println!("日付を入力してください(yyyy-mm-dd)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("日付の入力に失敗しました");
    NaiveDate::from_str(&input).expect("日付はyyyy-mm-ddの形式で入力してください")
  }
}