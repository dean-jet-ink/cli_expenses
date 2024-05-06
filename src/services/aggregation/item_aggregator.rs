use std::{
    collections::{BTreeMap, BTreeSet},
    str::FromStr,
};

use chrono::NaiveDate;

use crate::models::{Category, FilterDuration, Item};

pub struct ItemAggregator<'a> {
    items: &'a Vec<Item>,
}

impl<'a> ItemAggregator<'a> {
    pub fn new(items: &'a Vec<Item>) -> Self {
        Self { items }
    }

    pub fn aggregate(&self, duration: FilterDuration) -> BTreeMap<String, i32> {
        match duration {
            FilterDuration::Date => self.aggregate_by_date(),
            FilterDuration::Month => self.aggregate_by_month(),
            FilterDuration::Year => self.aggregate_by_year(),
        }
    }

    fn aggregate_by_date(&self) -> BTreeMap<String, i32> {
        let target_dates = self.target_dates();

        let mut result_table = BTreeMap::new();

        for date in target_dates {
            let filtered_items = self.filter_by_date(&date);
            let sum = self.summation(filtered_items);
            result_table.insert(date.format("%Y-%m-%d").to_string(), sum);
        }

        result_table
    }

    fn aggregate_by_month(&self) -> BTreeMap<String, i32> {
        let target_months = self.target_months();

        let mut result_table = BTreeMap::new();

        for month in target_months {
            let filtered_items = self.filter_by_month(&month);
            let sum = self.summation(filtered_items);
            result_table.insert(month.format("%Y-%m").to_string(), sum);
        }

        result_table
    }

    fn aggregate_by_year(&self) -> BTreeMap<String, i32> {
        let target_years = self.target_years();

        let mut result_table = BTreeMap::new();

        for year in target_years {
            let filtered_items = self.filter_by_year(&year);
            let sum = self.summation(filtered_items);
            result_table.insert(year.format("%Y").to_string(), sum);
        }

        result_table
    }

    fn target_dates(&self) -> BTreeSet<NaiveDate> {
        let mut dates = BTreeSet::new();

        for item in self.items {
            dates.insert(item.date().clone());
        }

        dates
    }

    fn target_months(&self) -> BTreeSet<NaiveDate> {
        let mut months = BTreeSet::new();

        for item in self.items {
            let formated_date = item.date().format("%Y-%m-01").to_string();
            let month = NaiveDate::from_str(formated_date.as_str())
                .expect(format!("無効な日付です {}", formated_date.as_str()).as_str());
            months.insert(month);
        }

        months
    }

    fn target_years(&self) -> BTreeSet<NaiveDate> {
        let mut years = BTreeSet::new();

        for item in self.items {
            let date = item.date();
            let year = NaiveDate::from_str(date.format("%Y-01-01").to_string().as_str())
                .expect(format!("無効な日付です {:?}", date).as_str());
            years.insert(year);
        }

        years
    }

    fn filter_by_date(&self, date: &NaiveDate) -> Vec<&Item> {
        self.items
            .iter()
            .filter(|item| item.date() == date)
            .collect()
    }

    fn filter_by_month(&self, date: &NaiveDate) -> Vec<&Item> {
        self.items
            .iter()
            .filter(|item| {
                item.date().format("%Y-%m").to_string() == date.format("%Y-%m").to_string()
            })
            .collect()
    }

    fn filter_by_year(&self, date: &NaiveDate) -> Vec<&Item> {
        self.items
            .iter()
            .filter(|item| item.date().format("%Y").to_string() == date.format("%Y").to_string())
            .collect()
    }

    fn summation(&self, items: Vec<&Item>) -> i32 {
        let mut sum: i32 = 0;

        for item in items {
            match item.category() {
                Category::Income(_) => sum += item.price() as i32,
                Category::Expense(_) => sum -= item.price() as i32,
            }
        }

        sum
    }
}
