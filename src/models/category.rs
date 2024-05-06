use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Income {
    Salary,
    Bonus,
    Other,
}

impl Income {
    pub fn to_value(&self) -> (u8, String) {
        match self {
            Self::Salary => (0, String::from("給与")),
            Self::Bonus => (1, String::from("ボーナス")),
            Self::Other => (2, String::from("その他")),
        }
    }

    pub fn array() -> Vec<(u8, String)> {
        vec![
            Self::Salary.to_value(),
            Self::Bonus.to_value(),
            Self::Other.to_value(),
        ]
    }
}

impl TryFrom<u8> for Income {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Salary),
            1 => Ok(Self::Bonus),
            2 => Ok(Self::Other),
            _ => Err(String::from(format!("無効な収入種別です {}", value))),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Expense {
    Food,
    Hobby,
    Other,
}

impl Expense {
    pub fn to_value(&self) -> (u8, String) {
        match self {
            Self::Food => (0, String::from("食費")),
            Self::Hobby => (1, String::from("趣味")),
            Self::Other => (2, String::from("その他")),
        }
    }

    pub fn array() -> Vec<(u8, String)> {
        vec![
            Self::Food.to_value(),
            Self::Hobby.to_value(),
            Self::Other.to_value(),
        ]
    }
}

impl TryFrom<u8> for Expense {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Food),
            1 => Ok(Self::Hobby),
            2 => Ok(Self::Other),
            _ => Err(String::from(format!("無効な支出種別です {}", value))),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Category {
    Income(Income),
    Expense(Expense),
}