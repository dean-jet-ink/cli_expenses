use crate::services::{AggregationService, RegisterService};
use std::io;

pub struct Router {}

impl Router {
    pub fn route() {
        println!("実行したい内容を入力してください(0: 登録, 1: 集計)");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let service_type: u8 = input.trim().parse().expect("数値を入力してください");

        match service_type {
            0 => {
                RegisterService::new().run();
            }
            1 => {
                AggregationService::new().run();
            }
            _ => {
                println!("無効な数値です {}", service_type);
            }
        }
    }
}
