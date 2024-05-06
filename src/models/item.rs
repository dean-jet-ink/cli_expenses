use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::category::Category;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Item {
    name: String,
    category: Category,
    price: u32,
    date: NaiveDate,
}

impl Item {
    pub fn new(name: String, category: Category, price: u32, date: NaiveDate) -> Self {
        Self {
            name,
            category,
            price,
            date,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn category(&self) -> &Category {
        &self.category
    }

    pub fn price(&self) -> u32 {
        self.price
    }

    pub fn date(&self) -> &NaiveDate {
        &self.date
    }
}
