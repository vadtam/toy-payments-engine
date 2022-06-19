use std::collections::{BTreeMap, BTreeSet};

use crate::tx::Transaction;

pub struct TransactionDatabase {
    pub db: BTreeMap<u32, Transaction>,
    pub disputes: BTreeSet<u32>,
}

impl TransactionDatabase {
    pub fn get_tx(&mut self, tx_id: &u32) -> Option<Transaction> {
        self.db.get(tx_id).cloned()
    }

    pub fn add_tx(&mut self, tx: &Transaction) {
        self.db.insert(tx.tx, *tx);
    }

    pub fn remove_tx(&mut self, tx_id: &u32) {
        self.db.remove(tx_id);
    }

    pub fn is_tx_exists(&mut self, tx_id: &u32) -> bool {
        let tx_maybe = self.get_tx(tx_id);
        if tx_maybe.is_some() {
            true
        } else {
            false
        }
    }

    pub fn is_under_dispute(&mut self, tx_id: &u32) -> bool {
        let val_maybe = self.disputes.get(tx_id);
        if val_maybe.is_some() {
            true
        } else {
            false
        }
    }

    pub fn create_dispute(&mut self, tx_id: &u32) {
        self.disputes.insert(*tx_id);
    }

    pub fn remove_dispute(&mut self, tx_id: &u32) {
        self.disputes.remove(tx_id);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};
    use rust_decimal::Decimal;
    use crate::tx_database::TransactionDatabase;
    use crate::tx::{Transaction, TransactionType};

    #[test]
    fn basic_database_functionality() {
        let mut tx_db = TransactionDatabase{db: BTreeMap::new(), disputes: BTreeSet::new()};
        let tx = Transaction{
            tx_type: TransactionType::Deposit,
            client: 1,
            tx: 1,
            amount: Some(Decimal::from(1)),
        };
        assert_eq!(tx_db.is_tx_exists(&tx.tx), false);
        tx_db.add_tx(&tx);
        assert_eq!(tx_db.is_tx_exists(&tx.tx), true);
        tx_db.remove_tx(&tx.tx);
        assert_eq!(tx_db.is_tx_exists(&tx.tx), false);
    }

    #[test]
    fn basic_disputes_functionality() {
        let mut tx_db = TransactionDatabase{db: BTreeMap::new(), disputes: BTreeSet::new()};
        let tx = Transaction{
            tx_type: TransactionType::Deposit,
            client: 1,
            tx: 1,
            amount: Some(Decimal::from(1)),
        };
        assert_eq!(tx_db.is_under_dispute(&tx.tx), false);
        tx_db.create_dispute(&tx.tx);
        assert_eq!(tx_db.is_under_dispute(&tx.tx), true);
        tx_db.remove_dispute(&tx.tx);
        assert_eq!(tx_db.is_under_dispute(&tx.tx), false);
    }
}


