use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;

pub fn select_commands<T: std::fmt::Display>(commands: &Vec<T>) -> &T {
    let selected = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("choice command")
        .default(0)
        .items(&commands)
        .interact()
        .unwrap();
    &commands[selected]
}
