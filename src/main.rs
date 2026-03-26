mod task;
mod storage;

use std::env;
use std::process;
use task::{Task, TaskStatus};
use storage::{load_tasks, save_tasks};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: Se requiere un comando. Ejemplos: add, list, update, delete, mark-in-progress, mark-done");
        process::exit(1);
    }

    let command = &args[1];
    let cmd_args = &args[2..];

    let mut tasks = match load_tasks() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error al cargar tareas: {}", e);
            process::exit(1);
        }
    };

    match command.as_str() {
        "add" => handle_add(cmd_args, &mut tasks),
        "list" => handle_list(cmd_args, &tasks),
        "update" => handle_update(cmd_args, &mut tasks),
        "delete" => handle_delete(cmd_args, &mut tasks),
        "mark-in-progress" => handle_mark_status(cmd_args, &mut tasks, TaskStatus::EnCurso),
        "mark-done" => handle_mark_status(cmd_args, &mut tasks, TaskStatus::Hecho),
        _ => {
            eprintln!("Comando desconocido: {}", command);
            process::exit(1);
        }
    }
}

/// Parsea un string a u32 (ID de tarea).
/// Si falla, imprime un error descriptivo y termina el proceso.
fn parse_id(s: &str) -> u32 {
    s.parse().unwrap_or_else(|_| {
        eprintln!("Error: ID inválido '{}' — debe ser un número entero positivo.", s);
        process::exit(1);
    })
}

fn handle_add(args: &[String], tasks: &mut Vec<Task>) {
    if args.is_empty() {
        eprintln!("Error: Debes proporcionar una descripción. Ejemplo: add \"Mi tarea\"");
        process::exit(1);
    }
    let description = args.join(" ");
    let new_id = tasks.iter().map(|t| t.id).max().map_or(1, |max| max + 1);
    let new_task = Task::new(new_id, description);
    tasks.push(new_task);
    if let Err(e) = save_tasks(tasks) {
        eprintln!("Error al guardar la tarea: {}", e);
        process::exit(1);
    }
    println!("Tarea agregada exitosamente (ID {})", new_id);
}

fn handle_list(args: &[String], tasks: &[Task]) {
    // Parsear el filtro de forma case-insensitive y validarlo
    let filter: Option<TaskStatus> = args.first().and_then(|s| {
        if s.is_empty() {
            return None;
        }
        let parsed = TaskStatus::from_str_ci(s);
        if parsed.is_none() {
            eprintln!(
                "Filtro inválido '{}'. Valores válidos: pendiente, en curso, hecho",
                s
            );
            process::exit(1);
        }
        parsed
    });

    println!("{:<4} | {:<12} | {}", "ID", "Estado", "Descripción");
    println!("{:-<4}---{:-<12}---{:-<20}", "", "", "");
    for task in tasks {
        if filter.is_none() || filter.as_ref() == Some(&task.status) {
            println!("{:<4} | {:<12} | {}", task.id, task.status, task.description);
        }
    }
}

fn handle_update(args: &[String], tasks: &mut Vec<Task>) {
    if args.len() < 2 {
        eprintln!("Uso: update <id> <nueva descripción>");
        process::exit(1);
    }
    let id = parse_id(&args[0]);
    let new_desc = args[1..].join(" ");
    let task = tasks.iter_mut().find(|t| t.id == id);
    match task {
        Some(t) => {
            t.update_description(new_desc);
            if let Err(e) = save_tasks(tasks) {
                eprintln!("Error al guardar: {}", e);
                process::exit(1);
            }
            println!("Tarea {} actualizada", id);
        }
        None => {
            eprintln!("No se encontró tarea con ID {}", id);
            process::exit(1);
        }
    }
}

fn handle_delete(args: &[String], tasks: &mut Vec<Task>) {
    if args.is_empty() {
        eprintln!("Uso: delete <id>");
        process::exit(1);
    }
    let id = parse_id(&args[0]);
    let pos = tasks.iter().position(|t| t.id == id);
    match pos {
        Some(idx) => {
            tasks.remove(idx);
            if let Err(e) = save_tasks(tasks) {
                eprintln!("Error al guardar: {}", e);
                process::exit(1);
            }
            println!("Tarea {} eliminada", id);
        }
        None => {
            eprintln!("No se encontró tarea con ID {}", id);
            process::exit(1);
        }
    }
}

fn handle_mark_status(args: &[String], tasks: &mut Vec<Task>, new_status: TaskStatus) {
    if args.is_empty() {
        eprintln!("Uso: mark-in-progress|mark-done <id>");
        process::exit(1);
    }
    let id = parse_id(&args[0]);
    let task = tasks.iter_mut().find(|t| t.id == id);
    match task {
        Some(t) => {
            let label = new_status.to_string();
            t.set_status(new_status);
            if let Err(e) = save_tasks(tasks) {
                eprintln!("Error al guardar: {}", e);
                process::exit(1);
            }
            println!("Tarea {} marcada como {}", id, label);
        }
        None => {
            eprintln!("No se encontró tarea con ID {}", id);
            process::exit(1);
        }
    }
}