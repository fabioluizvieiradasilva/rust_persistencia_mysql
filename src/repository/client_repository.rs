use crate::{config::cnn::get_connection, models::client::Client};
use mysql::{Result, prelude::Queryable, *};



pub fn create_client(
    name: &str,
    email: &str,    
) -> Result<()> {
    let mut conn = get_connection()?;

    conn.exec_drop(
        "INSERT INTO client (name, email) VALUES (:name, :email)",
        params! {
            "name" => name,
            "email" => email,
        },
    )?;

    Ok(())
}

pub fn read_clients() -> Result<Vec<Client>> {
    let mut conn = get_connection()?;

    let clients = conn.query_map(
        "SELECT id, name, email FROM client",
        |(id, name, email)| {
            Client { id, name, email }
        },
    )?;

    Ok(clients)
}

pub fn update_client(
    id: u32,
    name: &str,
    email: &str,
) -> Result<()> {
    let mut conn = get_connection()?;

    conn.exec_drop(
        "UPDATE client SET name = :name, email = :email WHERE id = :id",
        params! {
            "id" => id,
            "name" => name,
            "email" => email,
        },
    )?;

    Ok(())
}

pub fn delete_client(id: u32) -> Result<()> {
    let mut conn = get_connection()?;

    conn.exec_drop(
        "DELETE FROM client WHERE id = :id",
        params! {
            "id" => id,
        },
    )?;

    Ok(())
}