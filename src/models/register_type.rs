pub enum RegisterType {
    Income,
    Expense,
}

impl RegisterType {
    pub fn to_value(&self) -> (u8, String) {
        match self {
            Self::Income => (0, String::from("収入")),
            Self::Expense => (1, String::from("支出")),
        }
    }
}

impl TryFrom<u8> for RegisterType {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Income),
            1 => Ok(Self::Expense),
            _ => Err(String::from(format!("無効な登録種別です {}", value))),
        }
    }
}
