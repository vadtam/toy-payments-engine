use serde::Deserialize;
use rust_decimal::Decimal;

#[derive(PartialEq)]
#[derive(Copy, Clone)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(Copy, Clone)]
#[derive(Debug, Deserialize)]
pub struct Transaction {
    #[serde(rename = "type")]
    pub tx_type: TransactionType,
    pub client: u16,
    pub tx: u32,
    pub amount: Option<Decimal>,
}

impl Transaction {
    pub fn validate(&mut self) -> bool {
        // input validation
        if self.tx_type == TransactionType::Deposit ||
                self.tx_type == TransactionType::Withdrawal {
            if self.amount.is_some() {
                let amount = self.amount.unwrap().round_dp(4);
                if amount > Decimal::from(0) {
                    self.amount = Some(amount);
                    return true;
                } else {
                    // amount must be positive
                    return false;
                }
            } else {
                // amoust must be positive
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;
    use rust_decimal::prelude::FromPrimitive;
    use crate::tx::{Transaction, TransactionType};

    #[test]
    fn transaction_max4digits_functionality() {
        let mut tx = Transaction{
            tx_type: TransactionType::Deposit,
            client: 1,
            tx: 1,
            amount: Some(Decimal::from_f64(1.123456).unwrap()),
        };
        assert_eq!(tx.validate(), true);
        assert_eq!(tx.amount.unwrap(), Decimal::from_f64(1.1235).unwrap());
    }

    #[test]
    fn transaction_positive_amount_functionality() {
        let mut tx = Transaction{
            tx_type: TransactionType::Deposit,
            client: 1,
            tx: 1,
            amount: Some(Decimal::from(1)),
        };
        assert_eq!(tx.validate(), true);
        tx.amount = Some(Decimal::from(0));
        assert_eq!(tx.validate(), false);
    }
}
