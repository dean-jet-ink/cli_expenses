use crate::{config, models::Item};

use std::{
    fs::{File, OpenOptions},
    io::{BufReader, Write},
    path::Path,
};

#[derive(Debug)]
pub struct ItemRepository {
    file_path: String,
}

impl ItemRepository {
    pub fn new() -> Self {
        let file_path = String::from(config::constants::FILE_PATH);
        let path = Path::new(&file_path);
        if path.exists() {
            Self { file_path }
        } else {
            let mut fs = File::create(&file_path).expect("ファイルの作成に失敗しました");

            // JSONファイルの初期化
            // 初期化を行わない場合、findメソッドのデシリアライズでエラーとなる
            write!(fs, "[]").expect("ファイルの初期化に失敗しました");

            println!("データ用ファイルを新規作成しました");
            Self { file_path }
        }
    }

    pub fn find_all(&self) -> Vec<Item> {
        let fs = File::open(&self.file_path).unwrap();

        let buf_reader = BufReader::new(fs);

        serde_json::from_reader(buf_reader).expect("デシリアライズに失敗しました")
    }

    pub fn save(&self, item: Item) {
        let mut items = self.find_all();
        items.push(item);

        let json =
            serde_json::to_string_pretty(&items).expect("JSONへのシリアライズに失敗しました");

        let mut fs = OpenOptions::new()
            .write(true)
            .open(&self.file_path)
            .expect("ファイルを開くことができませんでした");

        writeln!(fs, "{}", json).expect("ファイルへの書き込みに失敗しました");
    }
}
