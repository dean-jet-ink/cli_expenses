use super::command::{
    category_command::CategoryCommand, date_command::DateCommand, name_command::NameCommand,
    price_command::PriceCommand, register_command::RegisterCommand,
};
use crate::{infrastructures::ItemRepository, models::Item};

pub struct RegisterService {
    item_repository: ItemRepository,
}

impl RegisterService {
    pub fn new() -> Self {
        RegisterService {
            item_repository: ItemRepository::new(),
        }
    }

    pub fn run(&self) {
        let register_type = RegisterCommand::input();

        let category = CategoryCommand::input(register_type);
        let name = NameCommand::input();
        let price = PriceCommand::input();
        let date = DateCommand::input();

        let item = Item::new(name, category, price, date);

        self.item_repository.save(item);
        println!("登録が完了しました");
    }
}
