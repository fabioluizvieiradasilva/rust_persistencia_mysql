use mysql::{Result};

use crate::repository::client_repository;

pub fn create_client() {
    clear_screen();
    let mut name = String::new();
    let mut email = String::new();

    println!("Nome do cliente:");
    std::io::stdin().read_line(&mut name).expect("Falha ao ler o nome");
    println!("Email do cliente:");
    std::io::stdin().read_line(&mut email).expect("Falha ao ler o email");

    client_repository::create_client(name.trim(), email.trim()).expect("Falha ao criar o cliente");

    println!("Cliente criado com sucesso");

    clear_screen();
}

pub fn read_client() ->Result<()> {
    clear_screen();
    let clients = client_repository::read_clients()?;
    for client in clients {
        println!("------------------------------");
        println!("ID: {} ", client.id);
        println!("Nome: {}", client.name);
        println!("Email: {}", client.email);
    }
    println!("------------------------------");
    
    break_to_enter();
    clear_screen();

    Ok(())
}

fn break_to_enter() {
    println!("Pressione Enter para continuar...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Falha ao ler a entrada");
    clear_screen();
}

fn clear_screen() {
    // Limpa a tela usando um comando específico para o sistema operacional
    if cfg!(target_os = "windows") {
        std::process::Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
    } else {
        std::process::Command::new("clear").status().unwrap();
    }
}

fn break_by_second(seconds: u64) {
    std::thread::sleep(std::time::Duration::from_secs(seconds));    
}

pub fn update_client() -> Result<(), std::io::Error> {
    clear_screen();
    let mut id = String::new();
    let mut name = String::new();
    let mut email = String::new();

    println!("ID do cliente a atualizar:");
    std::io::stdin().read_line(&mut id).expect("Falha ao ler o ID");
    println!("Novo nome do cliente:");
    std::io::stdin().read_line(&mut name).expect("Falha ao ler o nome");
    println!("Novo email do cliente:");
    std::io::stdin().read_line(&mut email).expect("Falha ao ler o email");
    
    let id = id.trim().parse::<u32>().expect("ID inválido");
    client_repository::update_client(id, name.trim(), email.trim()).expect("Falha ao atualizar o cliente");

    println!("Cliente atualizado com sucesso");
    break_by_second(2);
    clear_screen();

    Ok(())
}

pub fn delete_client() -> Result<()> {
    clear_screen();
    let mut id = String::new();

    println!("ID do cliente a deletar:");
    std::io::stdin().read_line(&mut id).expect("Falha ao ler o ID");
    let id = id.trim().parse::<u32>().expect("ID inválido");
    client_repository::delete_client(id)?;
    println!("Cliente deletado com sucesso");
    break_by_second(2);
    clear_screen();

    Ok(())
}