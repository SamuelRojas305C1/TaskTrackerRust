# TaskTrackerRust

TaskTrackerRust es una aplicación de línea de comandos escrita en Rust para gestionar tareas pendientes, con soporte para múltiples usuarios y autenticación.

## Características

- **Múltiples usuarios**: Cada usuario tiene su propio espacio de tareas independiente.
- **Autenticación segura**: Registro e inicio de sesión con contraseñas almacenadas mediante hash (bcrypt).
- **Gestión de tareas**: Agregar, listar, actualizar y eliminar tareas.
- **Estados**: Las tareas pueden estar en estado `Pendiente`, `in-progress` o `done`.
- **Persistencia por usuario**: Las tareas de cada usuario se guardan en un archivo JSON separado dentro del directorio `data/`.
- **Manejo de errores**: Validación robusta de argumentos y manejo de errores en operaciones de archivo.
- **Tests unitarios**: Cobertura completa de las funcionalidades principales.

## Instalación

1. Asegúrate de tener Rust instalado (versión 1.70 o superior).
2. Clona el repositorio:
   ```bash
   git clone <url-del-repositorio>
   cd TaskTrackerRust
3. Compila el proyecto:
   ```bash
   cargo build
   ```

## Uso

El ejecutable se encuentra en `target/debug/task_tracker_rust`.

## Autenticación

Al ejecutar cualquier comando, aparecera un menu interactivo:

Bienvenido
Para Iniciar sesion marque 1
Para Registrarse marque 2
Elige una opción:

Registrarse: Crea un nuevo usuario con contraseña (se guarda de forma segura).

Iniciar sesión: Accede con tus credenciales existentes.

Una vez autenticado, podras usar los comandos de gestion de tareas.

### Comandos disponibles

#### Agregar tarea
```bash
cargo run -- add "Descripción de la tarea"
```

#### Listar tareas
```bash
cargo run -- list
```

Para listar tareas por estado:
```bash
cargo run -- list pendiente
cargo run -- list en-curso
cargo run -- list hecho
```

#### Actualizar tarea
```bash
cargo run -- update <id> "Nueva descripción"
```

#### Eliminar tarea
```bash
cargo run -- delete <id>
```

#### Marcar como en curso
```bash
cargo run -- mark-in-progress <id>
```

#### Marcar como hecha
```bash
cargo run -- mark-done <id>
```

## Tests

Para ejecutar los tests unitarios:
```bash
cargo -- test
```

## Estructura del proyecto

- `src/main.rs`: Punto de entrada de la aplicación y lógica de línea de comandos.
- `src/task.rs`: Definición de la estructura `Task` y el enum `TaskStatus`.
- `src/storage.rs`: Lógica de persistencia de datos (lectura/escritura en JSON).