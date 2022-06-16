mod tx_reader;
use tx_reader::get_transaction_reader;
mod tx;
use tx::Transaction;
mod client;
mod client_database;
mod tx_database;
mod payments_engine;
use payments_engine::get_payments_engine;

fn main() {
    let mut payments_engine = get_payments_engine();
    // process
    let mut tx_reader = get_transaction_reader();
    for row in tx_reader.deserialize::<Transaction>() {
        match row {
            Ok(mut tx) => {
                if tx.validate() {
                    payments_engine.process_transaction(&tx);
                }
            },
            Err(_err) => {
                //println!("file row parsing error: {}", err);
            }
        }
    }
    payments_engine.client_db.print_all();
}

