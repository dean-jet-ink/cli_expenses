use crate::{infrastructures::ItemRepository, models::Item};

use super::{aggregation::ItemAggregator, command::filter_duration_command::FilterDurationCommand};

pub struct AggregationService {
    item_repository: ItemRepository,
}

impl AggregationService {
    pub fn new() -> Self {
        AggregationService {
            item_repository: ItemRepository::new(),
        }
    }

    pub fn run(&self) {
        let filter_duration = FilterDurationCommand::input();

        let items = self.item_repository.find_all();
        let item_aggregator = ItemAggregator::new(&items);

        let result_table = item_aggregator.aggregate(filter_duration);

        for result in result_table {
            print!("{}収支：", result.0);

            if result.1 > 0 {
                println!("+{}", result.1);
            } else {
                println!("{}", result.1);
            }
        }
    }
}
