use std::io;

pub struct PriceCommand {}

impl PriceCommand {
    pub fn input() -> u32 {
        println!("金額を入力してください");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("金額の入力に失敗しました");
        input.trim().parse().expect("金額は数値で入力してください")
    }
}
