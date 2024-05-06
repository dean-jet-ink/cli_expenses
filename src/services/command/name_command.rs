use std::io;

pub struct NameCommand {}

impl NameCommand {
    pub fn input() -> String {
        println!("品目名を入力してください");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("品目名の入力に失敗しました");

        input.trim().to_string()
    }
}
