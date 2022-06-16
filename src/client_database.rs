use std::collections::BTreeMap;

use rust_decimal::Decimal;
use crate::client::Client;

pub struct ClientDatabase {
    pub db: BTreeMap<u16, Client>
}

impl ClientDatabase {
    pub fn get_client(&mut self, client_id: u16) -> Client {
        let client_maybe = self.db.get(&client_id);
        if client_maybe.is_some() {
            client_maybe.unwrap().clone()
        } else {
            let new_client = Client {
                id: client_id,
                available: Decimal::from(0),
                held: Decimal::from(0),
                total: Decimal::from(0),
                locked: false,
            };
            self.db.insert(client_id, new_client);
            new_client
        }
    }

    pub fn update_client(&mut self, client: &Client) {
        self.db.insert(client.id, *client);
    }
    
    pub fn print_all(&mut self) {
        println!("client,available,held,total,locked");
        for (_, client) in self.db.iter() {
            let row = format!("{},{},{},{},{}", client.id, client.available, client.held,
                client.total, client.locked);
            println!("{}",row);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic_database_functionality() {
        use std::collections::BTreeMap;
        use rust_decimal::Decimal;
        use crate::client::Client;
        use crate::client_database::ClientDatabase;

        let mut client_db = ClientDatabase{db: BTreeMap::new()};
        let mut client = Client{
            id: 1,
            available: Decimal::from(0),
            held: Decimal::from(0),
            total: Decimal::from(0),
            locked: false,
        };
        assert_eq!(client_db.get_client(client.id), client);
        client.locked = true;
        client_db.update_client(&client);
        assert_eq!(client_db.get_client(client.id), client);
    }
}

