use std::io;

use crate::models::FilterDuration;

pub struct FilterDurationCommand {}

impl FilterDurationCommand {
    pub fn input() -> FilterDuration {
        println!("集計種別を入力してください");

        let mut durations = Vec::new();
        for d in FilterDuration::array() {
            durations.push(format!("{}:{}", d.0, d.1));
        }
        println!("({})", durations.join(", "));

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("集計種別の入力に失敗しました");

        let value: u8 = input
            .trim()
            .parse()
            .expect("集計種別の数値を入力してください");
        let filter_duration = FilterDuration::try_from(value);

        match filter_duration {
            Ok(f) => f,
            Err(e) => panic!("{}", e),
        }
    }
}
