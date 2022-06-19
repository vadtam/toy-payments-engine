use std::env;
use std::fs::File;

pub fn get_transaction_reader() -> csv::Reader<File> {
    let csv_reader: csv::Reader<File>;
    {
        let fpath: String;
        {
            let args: Vec<String> = env::args().collect();
            if args.len() != 2 {
                panic!("User error. Run toy payment engine as: $ cargo run -- transactions.csv");
            }
            fpath = args[1].to_string();
        }

        let csv_reader_res = csv::ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b',')
            .flexible(true)
            .trim(csv::Trim::All)
            .from_path(&fpath);

        if csv_reader_res.is_ok() {
            csv_reader = csv_reader_res.unwrap();
        } else {
            panic!("file {} not found", &fpath);
        }
    }
    csv_reader
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;
    use crate::tx::{Transaction, TransactionType};

    #[test]
    fn basic_reader_functionality() {
        let data = "type,   client,   tx,   amount\n
          deposit,     1,    2,      3.0\n
          withdrawal,4,5,6.0\n
          some malformed line\n,
          chargeback, 7,8";

        let mut tx_reader: csv::Reader<&[u8]> = csv::ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b',')
            .flexible(true)
            .trim(csv::Trim::All)
            .from_reader(data.as_bytes());
        let mut tx_vec: Vec<Transaction> = Vec::new();
        for row in tx_reader.deserialize::<Transaction>() {
            if row.is_ok() {
                let mut tx = row.unwrap();
                if tx.validate() {
                    tx_vec.push(tx);
                }
            }
        }
        let expected_tx_0 = Transaction {
          tx_type: TransactionType::Deposit,
          client: 1,
          tx: 2,
          amount: Some(Decimal::from(3))
        };
        let expected_tx_1 = Transaction {
          tx_type: TransactionType::Withdrawal,
          client: 4,
          tx: 5,
          amount: Some(Decimal::from(6))
        };
        let expected_tx_2 = Transaction {
          tx_type: TransactionType::Chargeback,
          client: 7,
          tx: 8,
          amount: None
        };
        assert_eq!(tx_vec.len(), 3);
        assert_eq!(tx_vec[0], expected_tx_0);
        assert_eq!(tx_vec[1], expected_tx_1);
        assert_eq!(tx_vec[2], expected_tx_2);
    }
}

