mod config;
mod models;

use std::result::Result;
use models::client::Client;
use mysql::prelude::Queryable;

fn main() -> Result<(), mysql::Error> {
    let mut cnn = config::cnn::get_connection()?;
    
    let clients = cnn.query_map(
        "SELECT id, name, email FROM client",
        |(id, name, email)| {
            Client { id, name, email }
        },
    )?;

    for client in clients {
        println!("ID: {}, Name: {}, Email: {}", client.id, client.name, client.email);
        println!("-----------------------------");        
    }

    Ok(())
}
