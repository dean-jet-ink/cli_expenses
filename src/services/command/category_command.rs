use std::io;

use crate::models::{Category, Expense, Income, RegisterType};

pub struct CategoryCommand {}

impl CategoryCommand {
    pub fn input(register_type: RegisterType) -> Category {
        print!("カテゴリーを入力してください");

        match register_type {
            RegisterType::Income => {
                print!("(");
                let mut vec = vec![];
                for i in Income::array() {
                    vec.push(format!("{}:{}", i.0, i.1));
                }
                println!("{})", vec.join(", "));
            }
            RegisterType::Expense => {
                print!("(");
                let mut vec = vec![];
                for i in Expense::array() {
                    vec.push(format!("{}:{}", i.0, i.1));
                }
                println!("{})", vec.join(", "));
            }
        }

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("カテゴリーの入力に失敗しました");

        let value: u8 = input
            .trim()
            .parse()
            .expect("カテゴリーの数値を入力してください");

        match register_type {
            RegisterType::Income => {
                let income = Income::try_from(value);
                match income {
                    Ok(i) => Category::Income(i),
                    Err(e) => panic!("{}", e),
                }
            }
            RegisterType::Expense => {
                let expense = Expense::try_from(value);
                match expense {
                    Ok(e) => Category::Expense(e),
                    Err(e) => panic!("{}", e),
                }
            }
        }
    }
}
