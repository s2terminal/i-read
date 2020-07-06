use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;

pub fn select_commands(commands: &Vec<String>) -> usize {
    Select::with_theme(&ColorfulTheme::default())
        .with_prompt("choice command")
        .default(0)
        .items(&commands)
        .interact()
        .unwrap()
}
