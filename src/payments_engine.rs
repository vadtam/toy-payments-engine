use std::collections::{BTreeMap, BTreeSet};

use crate::tx::*;
use crate::client_database::*;
use crate::tx_database::TransactionDatabase;


pub struct PaymentsEngine {
    pub client_db: ClientDatabase,
    pub tx_db: TransactionDatabase,
}

pub fn get_payments_engine() -> PaymentsEngine {
    let engine = PaymentsEngine{
        client_db: ClientDatabase{db: BTreeMap::new()},
        tx_db: TransactionDatabase{db: BTreeMap::new(), disputes: BTreeSet::new()},
    };
    engine
}

impl PaymentsEngine {
    fn process_deposit(&mut self, tx: &Transaction) {
        let mut client = self.client_db.get_client(tx.client);
        if self.tx_db.is_tx_exists(&tx.tx) {
            return;
        }
        client.available += tx.amount.unwrap();
        client.total += tx.amount.unwrap();
        self.tx_db.add_tx(tx);
        self.client_db.update_client(&client);
    }

    fn process_withdrawal(&mut self, tx: &Transaction) {
        let mut client = self.client_db.get_client(tx.client);
        if self.tx_db.is_tx_exists(&tx.tx) {
            return;
        }
        let amount = tx.amount.unwrap();
        if amount <= client.available {
            client.available -= amount;
            client.total -= amount;
            self.tx_db.add_tx(tx);
        } else {
            return;
        }
        self.client_db.update_client(&client);
    }

    fn process_dispute(&mut self, tx: &Transaction) {
        let mut client = self.client_db.get_client(tx.client);

        if self.tx_db.is_under_dispute(&tx.tx) {
            return;
        }
        let disputed_tx = match self.tx_db.get_tx(&tx.tx) {
            Some(val) => {
                self.tx_db.create_dispute(&tx.tx);
                val
            },
            None => return,
        };
        if disputed_tx.client != tx.client {
            // only client's own transactions are disputable
            self.tx_db.remove_dispute(&tx.tx);
            return;
        }
        if disputed_tx.tx_type == TransactionType::Deposit {
            let amount = disputed_tx.amount.unwrap();
            client.available -= amount;
            client.held += amount;
        } else if disputed_tx.tx_type == TransactionType::Withdrawal {
            let amount = disputed_tx.amount.unwrap();
            client.held += amount;
            client.total += amount;
        } else {
            self.tx_db.remove_dispute(&tx.tx);
            return;
        }
        self.client_db.update_client(&client);
    }

    fn process_resolve(&mut self, tx: &Transaction) {
        let mut client = self.client_db.get_client(tx.client);

        if !self.tx_db.is_under_dispute(&tx.tx) {
            return;
        }
        let disputed_tx = self.tx_db.get_tx(&tx.tx).unwrap();
        if disputed_tx.tx_type == TransactionType::Deposit {
            let amount = disputed_tx.amount.unwrap();
            client.available += amount;
            client.held -= amount;
        } else if disputed_tx.tx_type == TransactionType::Withdrawal {
            let amount = disputed_tx.amount.unwrap();
            client.held -= amount;
            client.total -= amount;
        }
        self.tx_db.remove_dispute(&tx.tx);
        self.client_db.update_client(&client);
    }

    fn process_chargeback(&mut self, tx: &Transaction) {
        let mut client = self.client_db.get_client(tx.client);

        if !self.tx_db.is_under_dispute(&tx.tx) {
            return;
        }
        let disputed_tx = self.tx_db.get_tx(&tx.tx).unwrap();
        if disputed_tx.tx_type == TransactionType::Deposit {
            let amount = disputed_tx.amount.unwrap();
            client.held -= amount;
            client.total -= amount;
            client.locked = true;
        } else if disputed_tx.tx_type == TransactionType::Withdrawal {
            let amount = disputed_tx.amount.unwrap();
            client.held -= amount;
            client.available += amount;
            client.locked = true;
        }
        self.tx_db.remove_dispute(&tx.tx);
        self.tx_db.remove_tx(&tx.tx);
        self.client_db.update_client(&client);
    }

    pub fn process_transaction(&mut self, tx: &Transaction) {
        match tx.tx_type {
            TransactionType::Deposit => self.process_deposit(tx),
            TransactionType::Withdrawal => self.process_withdrawal(tx),
            TransactionType::Dispute => self.process_dispute(tx),
            TransactionType::Resolve => self.process_resolve(tx),
            TransactionType::Chargeback => self.process_chargeback(tx),
        }
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;
    use crate::client::Client;
    use crate::tx::{Transaction, TransactionType};
    use crate::payments_engine::get_payments_engine;

    #[test]
    fn process_deposit_functionality() {
        let mut engine = get_payments_engine();
        let client = Client{
            id: 1,
            available: Decimal::from(0),
            held: Decimal::from(0),
            total: Decimal::from(0),
            locked: false,
        };
        engine.client_db.update_client(&client);

        let tx = Transaction{
            tx_type: TransactionType::Deposit,
            client: 1,
            tx: 1,
            amount: Some(Decimal::from(1)),
        };
        engine.process_deposit(&tx);
        let updated_client = engine.client_db.get_client(client.id);
        let expected_client = Client {
            id: 1,
            available: Decimal::from(1),
            held: Decimal::from(0),
            total: Decimal::from(1),
            locked: false,
        };
        assert_eq!(updated_client, expected_client);
    }

    #[test]
    fn process_withdrawal_functionality() {
        let mut engine = get_payments_engine();
        let client = Client{
            id: 1,
            available: Decimal::from(13),
            held: Decimal::from(0),
            total: Decimal::from(13),
            locked: false,
        };
        engine.client_db.update_client(&client);

        let tx = Transaction{
            tx_type: TransactionType::Withdrawal,
            client: 1,
            tx: 1,
            amount: Some(Decimal::from(4)),
        };

        engine.process_withdrawal(&tx);
        let updated_client = engine.client_db.get_client(client.id);
        let expected_client = Client {
            id: 1,
            available: Decimal::from(9),
            held: Decimal::from(0),
            total: Decimal::from(9),
            locked: false,
        };
        assert_eq!(updated_client, expected_client);
    }

    #[test]
    fn process_dispute_deposit_functionality() {
        let mut engine = get_payments_engine();
        let client = Client{
            id: 1,
            available: Decimal::from(0),
            held: Decimal::from(0),
            total: Decimal::from(0),
            locked: false,
        };
        engine.client_db.update_client(&client);

        let tx1 = Transaction{
            tx_type: TransactionType::Deposit,
            client: 1,
            tx: 1,
            amount: Some(Decimal::from(13)),
        };
        engine.process_deposit(&tx1);

        let tx2 = Transaction{
            tx_type: TransactionType::Withdrawal,
            client: 1,
            tx: 2,
            amount: Some(Decimal::from(4)),
        };
        engine.process_withdrawal(&tx2);

        let tx3 = Transaction{
            tx_type: TransactionType::Dispute,
            client: 1,
            tx: 2,
            amount: None,
        };
        engine.process_dispute(&tx3);
        let updated_client = engine.client_db.get_client(client.id);
        let expected_client = Client {
            id: 1,
            available: Decimal::from(9),
            held: Decimal::from(4),
            total: Decimal::from(13),
            locked: false,
        };
        assert_eq!(updated_client, expected_client);
    }

    #[test]
    fn process_dispute_withdrawal_functionality() {
        let mut engine = get_payments_engine();
        let client = Client{
            id: 1,
            available: Decimal::from(0),
            held: Decimal::from(0),
            total: Decimal::from(0),
            locked: false,
        };
        engine.client_db.update_client(&client);

        let tx1 = Transaction{
            tx_type: TransactionType::Deposit,
            client: 1,
            tx: 1,
            amount: Some(Decimal::from(13)),
        };
        engine.process_deposit(&tx1);

        let tx2 = Transaction{
            tx_type: TransactionType::Deposit,
            client: 1,
            tx: 2,
            amount: Some(Decimal::from(4)),
        };
        engine.process_deposit(&tx2);

        let tx3 = Transaction{
            tx_type: TransactionType::Dispute,
            client: 1,
            tx: 2,
            amount: None,
        };
        engine.process_dispute(&tx3);
        let updated_client = engine.client_db.get_client(client.id);
        let expected_client = Client {
            id: 1,
            available: Decimal::from(13),
            held: Decimal::from(4),
            total: Decimal::from(17),
            locked: false,
        };
        assert_eq!(updated_client, expected_client);
    }

    #[test]
    fn process_resolve_deposit_functionality() {
        let mut engine = get_payments_engine();
        let client = Client{
            id: 1,
            available: Decimal::from(0),
            held: Decimal::from(0),
            total: Decimal::from(0),
            locked: false,
        };
        engine.client_db.update_client(&client);

        let tx1 = Transaction{
            tx_type: TransactionType::Deposit,
            client: 1,
            tx: 1,
            amount: Some(Decimal::from(13)),
        };
        engine.process_deposit(&tx1);

        let tx2 = Transaction{
            tx_type: TransactionType::Deposit,
            client: 1,
            tx: 2,
            amount: Some(Decimal::from(4)),
        };
        engine.process_deposit(&tx2);

        let tx3 = Transaction{
            tx_type: TransactionType::Dispute,
            client: 1,
            tx: 2,
            amount: None,
        };
        engine.process_dispute(&tx3);

        let tx4 = Transaction{
            tx_type: TransactionType::Resolve,
            client: 1,
            tx: 2,  
            amount: None,
        };
        engine.process_resolve(&tx4);

        let updated_client = engine.client_db.get_client(client.id);
        let expected_client = Client {
            id: 1,
            available: Decimal::from(17),
            held: Decimal::from(0),
            total: Decimal::from(17),
            locked: false,
        };
        assert_eq!(updated_client, expected_client);
    }

    #[test]
    fn process_resolve_withdrawal_functionality() {
        let mut engine = get_payments_engine();
        let client = Client{
            id: 1,
            available: Decimal::from(0),
            held: Decimal::from(0),
            total: Decimal::from(0),
            locked: false,
        };
        engine.client_db.update_client(&client);

        let tx1 = Transaction{
            tx_type: TransactionType::Deposit,
            client: 1,
            tx: 1,
            amount: Some(Decimal::from(13)),
        };
        engine.process_deposit(&tx1);

        let tx2 = Transaction{
            tx_type: TransactionType::Withdrawal,
            client: 1,
            tx: 2,
            amount: Some(Decimal::from(4)),
        };
        engine.process_withdrawal(&tx2);

        let tx3 = Transaction{
            tx_type: TransactionType::Dispute,
            client: 1,
            tx: 2,
            amount: None,
        };
        engine.process_dispute(&tx3);

        let tx4 = Transaction{
            tx_type: TransactionType::Resolve,
            client: 1,
            tx: 2,
            amount: None,
        };
        engine.process_resolve(&tx4);

        let updated_client = engine.client_db.get_client(client.id);
        let expected_client = Client {
            id: 1,
            available: Decimal::from(9),
            held: Decimal::from(0),
            total: Decimal::from(9),
            locked: false,
        };
        assert_eq!(updated_client, expected_client);
    }

    #[test]
    fn process_chargeback_deposit_functionality() {
        let mut engine = get_payments_engine();
        let client = Client{
            id: 1,
            available: Decimal::from(0),
            held: Decimal::from(0),
            total: Decimal::from(0),
            locked: false,
        };
        engine.client_db.update_client(&client);

        let tx1 = Transaction{
            tx_type: TransactionType::Deposit,
            client: 1,
            tx: 1,
            amount: Some(Decimal::from(13)),
        };
        engine.process_deposit(&tx1);

        let tx2 = Transaction{
            tx_type: TransactionType::Deposit,
            client: 1,
            tx: 2,
            amount: Some(Decimal::from(4)),
        };
        engine.process_deposit(&tx2);

        let tx3 = Transaction{
            tx_type: TransactionType::Dispute,
            client: 1,
            tx: 2,
            amount: None,
        };
        engine.process_dispute(&tx3);

        let tx4 = Transaction{
            tx_type: TransactionType::Chargeback,
            client: 1,
            tx: 2,
            amount: None,
        };
        engine.process_chargeback(&tx4);

        let updated_client = engine.client_db.get_client(client.id);
        let expected_client = Client {
            id: 1,
            available: Decimal::from(13),
            held: Decimal::from(0),
            total: Decimal::from(13),
            locked: true,
        };
        assert_eq!(updated_client, expected_client);
    }

    #[test]
    fn process_chargeback_withdrawal_functionality() {
        let mut engine = get_payments_engine();
        let client = Client{
            id: 1,
            available: Decimal::from(0),
            held: Decimal::from(0),
            total: Decimal::from(0),
            locked: false,
        };
        engine.client_db.update_client(&client);

        let tx1 = Transaction{
            tx_type: TransactionType::Deposit,
            client: 1,
            tx: 1,
            amount: Some(Decimal::from(13)),
        };
        engine.process_deposit(&tx1);

        let tx2 = Transaction{
            tx_type: TransactionType::Withdrawal,
            client: 1,
            tx: 2,
            amount: Some(Decimal::from(4)),
        };
        engine.process_withdrawal(&tx2);

        let tx3 = Transaction{
            tx_type: TransactionType::Dispute,
            client: 1,
            tx: 2,
            amount: None,
        };
        engine.process_dispute(&tx3);

        let tx4 = Transaction{
            tx_type: TransactionType::Chargeback,
            client: 1,
            tx: 2,
            amount: None,
        };
        engine.process_chargeback(&tx4);

        let updated_client = engine.client_db.get_client(client.id);
        let expected_client = Client {
            id: 1,
            available: Decimal::from(13),
            held: Decimal::from(0),
            total: Decimal::from(13),
            locked: true,
        };
        assert_eq!(updated_client, expected_client);
    }
}

