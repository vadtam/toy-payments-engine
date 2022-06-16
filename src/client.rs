use rust_decimal::Decimal;

#[derive(PartialEq)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct Client {
    pub id: u16,
    pub available: Decimal,
    pub held: Decimal,
    pub total: Decimal,
    pub locked: bool,
}

