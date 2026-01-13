mod api;
mod cli;
mod ui;
mod commands {
    pub mod generate;
    pub mod refresh;
}

use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "-h" | "--help" => {
                print_help();
                return;
            }
            _ => {
                eprintln!("[!] Argumento desconocido: {}", args[1]);
                eprintln!("Usa -h para ver la ayuda");
                std::process::exit(1);
            }
        }
    }

    if let Err(e) = cli::run() {
        eprintln!("\n[!] Error: {e}");
        std::process::exit(1);
    }
}

fn print_help() {
    println!(
        "Code Lens - Generador de tokens

USO:
    code-lens [OPCIONES]

OPCIONES:
    -h, --help    Muestra ayuda

DESCRIPCIÓN:
    Herramienta interactiva para generar y refrescar tokens JWT para 
    proyectos gestionados con code lens.
    
    Al ejecutar sin argumentos, se abrirá un menú interactivo donde puedes:
    1. Generar un nuevo token
    2. Refrescar un token existente
    
    
FLUJO DE USO:
    1. Inicia tu proyecto en github.
    2. Ejecuta esta herramienta para generar un token JWT asociado a tu proyecto.
    3. El token se registra automáticamente en el servidor de code lens.
    4. Configura el token como un secret en tu repositorio de github.
    5. Usa el github action de code lens en tu repositorio, proporcionando el token generado.
    6. Code lens analizará tú código en cada push a la rama principal y generará documentación actualizada.
    7. Para acceder a la documentación generada, reemplaza 'github.com' con codelens.inf.uct.cl en la URL de tu repositorio."
    );
}
