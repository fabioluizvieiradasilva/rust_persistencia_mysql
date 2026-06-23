mod config;
mod models;
mod screen;
mod repository;

use screen::display;
/*use std::result::Result;
use models::client::Client;
use mysql::prelude::Queryable;*/

fn main() {
    loop {
        println!("CRUD Clientes");
            println!("1. Criar cliente");
            println!("2. Ler cliente");
            println!("3. Atualizar cliente");
            println!("4. Deletar cliente");
            println!("5. Sair");
    
            let mut choice = String::new();
            std::io::stdin().read_line(&mut choice).expect("Falha ao ler a entrada");
    
            match choice.trim() {
                "1" => display::create_client(),
                "2" => {
                    if let Err(e) = display::read_client() {
                        println!("Erro ao ler clientes: {}", e);
                    }
                }
                "3" => {
                    if let Err(e) = display::update_client() {
                        println!("Erro ao atualizar cliente: {}", e);
                    }
                }
                "4" => {
                    if let Err(e) = display::delete_client() {
                        println!("Erro ao deletar cliente: {}", e);
                    }
                }
                "5" => {
                    println!("Saindo...");
                    break;
                },
                _ => println!("Opção inválida, tente novamente."),
            }
    }
}


/*fn main() -> Result<(), mysql::Error> {
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
}*/
