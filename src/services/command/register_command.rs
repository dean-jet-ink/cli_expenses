use std::io;

use crate::models::RegisterType;

pub struct RegisterCommand {}

impl RegisterCommand {
    pub fn input() -> RegisterType {
        println!("登録種別を入力してください(0: 収入, 1: 支出)");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("登録種別の入力に失敗しました");

        let value: u8 = input
            .trim()
            .parse()
            .expect("登録種別の数値を入力してください");

        let register_type = RegisterType::try_from(value);
        match register_type {
            Ok(r) => r,
            Err(e) => panic!("{}", e),
        }
    }
}
