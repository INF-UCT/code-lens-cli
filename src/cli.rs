use crate::commands;
use crate::ui::{prompt_main_menu, MainMenuOption};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    print_welcome_header();

    let menu_option = prompt_main_menu()?;

    match menu_option {
        MainMenuOption::Generate => commands::generate::execute()?,
        MainMenuOption::Refresh => commands::refresh::execute()?,
    }

    Ok(())
}

fn print_welcome_header() {
    println!("CodeLens - Token Generator\n");
}
