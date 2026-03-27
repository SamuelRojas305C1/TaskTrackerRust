use crate::user::{self, UserMap};
use std::io::{self, Write};


pub fn authenticate_or_register() -> Result<String, Box<dyn std::error::Error>> {
    let mut users = user::load_users()?;

    println!("Bienvenido");
    println!("Para Iniciar sesion marque 1");
    println!("Para Registrarse marque 2");
    print!("Elige una opción: ");
    io::stdout().flush()?;

    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;
    match choice.trim() {
        "1" => login(&mut users),
        "2" => register(&mut users),
        _ => {
            eprintln!("Opción inválida");
            std::process::exit(1);
        }
    }
}

fn login(users: &mut UserMap) -> Result<String, Box<dyn std::error::Error>> {
    print!("Usuario: ");
    io::stdout().flush()?;
    let mut username = String::new();
    io::stdin().read_line(&mut username)?;
    let username = username.trim().to_string();

    print!("Contraseña: ");
    io::stdout().flush()?;
    let mut password = String::new();
    io::stdin().read_line(&mut password)?;
    let password = password.trim();

    if user::authenticate(&username, password, users) {
        println!("Sesion iniciada correctamente");
        Ok(username)
    } else {
        eprintln!("Credenciales incorrectas");
        std::process::exit(1);
    }
}

fn register(users: &mut UserMap) -> Result<String, Box<dyn std::error::Error>> {
    print!("Nuevo usuario: ");
    io::stdout().flush()?;
    let mut username = String::new();
    io::stdin().read_line(&mut username)?;
    let username = username.trim().to_string();

    print!("Contraseña: ");
    io::stdout().flush()?;
    let mut password = String::new();
    io::stdin().read_line(&mut password)?;
    let password = password.trim();

    if let Err(e) = user::register(&username, password, users) {
        eprintln!("Error al registrar: {}", e);
        std::process::exit(1);
    }

    user::save_users(users)?;
    println!("Usuario registrado correctamente. Ya puedes iniciar sesion");

    login(users)
}