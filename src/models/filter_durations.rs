pub enum FilterDuration {
    Date,
    Month,
    Year,
}

impl FilterDuration {
    pub fn to_value(&self) -> (u8, String) {
        match self {
            Self::Date => (0, String::from("日")),
            Self::Month => (1, String::from("月")),
            Self::Year => (2, String::from("年")),
        }
    }

    pub fn array() -> Vec<(u8, String)> {
        vec![
            Self::Date.to_value(),
            Self::Month.to_value(),
            Self::Year.to_value(),
        ]
    }
}

impl TryFrom<u8> for FilterDuration {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(FilterDuration::Date),
            1 => Ok(FilterDuration::Month),
            2 => Ok(FilterDuration::Year),
            _ => Err(String::from("無効な集計種別です")),
        }
    }
}
