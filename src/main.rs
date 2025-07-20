mod args;
mod cmds;
mod exec;

use cursive::theme::{BaseColor, Color, PaletteColor, Theme};
use cursive::traits::*;
use cursive::views::{CircularFocus, Dialog, SelectView};
use cursive::Cursive;

fn main() {
    let content = args::get_content();
    let commands = cmds::Command::get_commands(content);
    select_and_exec_command(commands)
}

fn select_and_exec_command(commands: Vec<cmds::Command>) {
    let mut select = SelectView::new().autojump();

    for command in commands {
        select.add_item(command.to_string(), command)
    }

    select.set_on_submit(|siv: &mut Cursive, cmd: &cmds::Command| {
        siv.quit();
        siv.set_user_data(cmd.clone());
    });

    let mut siv = cursive::default();
    let theme = custom_theme_from_cursive(&siv);
    siv.set_theme(theme);

    siv.add_layer(
        CircularFocus::new(Dialog::around(select.scrollable()).title("choice command"))
            .wrap_arrows(),
    );

    siv.run();

    let take_command: cmds::Command = siv.take_user_data().unwrap();
    println!("execute command: {take_command}");
    exec::execute(&take_command.executable());
}

// https://github.com/gyscos/cursive/blob/cursive-v0.16.3/examples/src/bin/terminal_default.rs
fn custom_theme_from_cursive(siv: &Cursive) -> Theme {
    let mut theme = siv.current_theme().clone();
    theme.palette[PaletteColor::Background] = Color::TerminalDefault;
    theme.palette[PaletteColor::Shadow] = Color::TerminalDefault;
    theme.palette[PaletteColor::View] = Color::TerminalDefault;
    theme.palette[PaletteColor::Primary] = Color::TerminalDefault;
    theme.palette[PaletteColor::Tertiary] = Color::TerminalDefault;
    theme.palette[PaletteColor::TitlePrimary] = Color::TerminalDefault;
    theme.palette[PaletteColor::Highlight] = Color::Light(BaseColor::White);
    theme.palette[PaletteColor::HighlightText] = Color::Dark(BaseColor::Blue);

    theme
}
