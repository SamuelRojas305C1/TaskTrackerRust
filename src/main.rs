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

    let result = match command.as_str() {
        "add"              => handle_add(cmd_args, &mut tasks),
        "list"             => handle_list(cmd_args, &tasks),
        "update"           => handle_update(cmd_args, &mut tasks),
        "delete"           => handle_delete(cmd_args, &mut tasks),
        "mark-in-progress" => handle_mark_status(cmd_args, &mut tasks, TaskStatus::EnCurso),
        "mark-done"        => handle_mark_status(cmd_args, &mut tasks, TaskStatus::Hecho),
        _ => Err(format!("Comando desconocido: '{}'", command)),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}


/// Parsea un string a u32 (ID de tarea)
/// Retorna Err con mensaje descriptivo si no es un entero positivo
fn parse_id(s: &str) -> Result<u32, String> {
    s.parse::<u32>()
        .map_err(|_| format!("ID inválido '{}' — debe ser un número entero positivo.", s))
}

fn handle_add(args: &[String], tasks: &mut Vec<Task>) -> Result<(), String> {
    if args.is_empty() {
        return Err("Debes proporcionar una descripción. Ejemplo: add \"Mi tarea\"".to_string());
    }
    let description = args.join(" ");
    let new_id = tasks.iter().map(|t| t.id).max().map_or(1, |max| max + 1);
    let new_task = Task::new(new_id, description);
    tasks.push(new_task);
    save_tasks(tasks)?;
    println!("Tarea agregada exitosamente (ID {})", new_id);
    Ok(())
}

fn handle_list(args: &[String], tasks: &[Task]) -> Result<(), String> {
    // Parsear y validar el filtro de forma case-insensitive
    let filter: Option<TaskStatus> = match args.first() {
        None => None,
        Some(s) if s.is_empty() => None,
        Some(s) => {
            let parsed = TaskStatus::from_str_ci(s);
            if parsed.is_none() {
                return Err(format!(
                    "Filtro inválido '{}'. Valores válidos: pendiente, en-curso, hecho",
                    s
                ));
            }
            parsed
        }
    };

    let shown: Vec<&Task> = tasks
        .iter()
        .filter(|t| filter.is_none() || filter.as_ref() == Some(&t.status))
        .collect();

    if shown.is_empty() {
        println!("No hay tareas{}", filter.map_or(String::new(), |f| format!(" con estado '{}'", f)));
        return Ok(());
    }

    println!("{:<4} | {:<12} | {}", "ID", "Estado", "Descripción");
    println!("{:-<4}---{:-<12}---{:-<20}", "", "", "");
    for task in shown {
        println!("{:<4} | {:<12} | {}", task.id, task.status, task.description);
    }
    Ok(())
}

fn handle_update(args: &[String], tasks: &mut Vec<Task>) -> Result<(), String> {
    if args.len() < 2 {
        return Err("Uso: update <id> <nueva descripción>".to_string());
    }
    let id = parse_id(&args[0])?;
    let new_desc = args[1..].join(" ");
    let task = tasks.iter_mut().find(|t| t.id == id);
    match task {
        Some(t) => {
            t.update_description(new_desc);
            save_tasks(tasks)?;
            println!("Tarea {} actualizada", id);
            Ok(())
        }
        None => Err(format!("No se encontró tarea con ID {}", id)),
    }
}

fn handle_delete(args: &[String], tasks: &mut Vec<Task>) -> Result<(), String> {
    if args.is_empty() {
        return Err("Uso: delete <id>".to_string());
    }
    let id = parse_id(&args[0])?;
    let pos = tasks.iter().position(|t| t.id == id);
    match pos {
        Some(idx) => {
            tasks.remove(idx);
            save_tasks(tasks)?;
            println!("Tarea {} eliminada", id);
            Ok(())
        }
        None => Err(format!("No se encontró tarea con ID {}", id)),
    }
}

fn handle_mark_status(
    args: &[String],
    tasks: &mut Vec<Task>,
    new_status: TaskStatus,
) -> Result<(), String> {
    if args.is_empty() {
        return Err("Uso: mark-in-progress|mark-done <id>".to_string());
    }
    let id = parse_id(&args[0])?;
    let task = tasks.iter_mut().find(|t| t.id == id);
    match task {
        Some(t) => {
            let label = new_status.to_string();
            t.set_status(new_status);
            save_tasks(tasks)?;
            println!("Tarea {} marcada como {}", id, label);
            Ok(())
        }
        None => Err(format!("No se encontró tarea con ID {}", id)),
    }
}