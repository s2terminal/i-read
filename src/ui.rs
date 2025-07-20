use crate::cmds;
use cursive::Cursive;
use cursive::CursiveRunnable;
use cursive::theme::{BaseColor, Color, PaletteColor, Theme};
use cursive::traits::*;
use cursive::views::{CircularFocus, Dialog, SelectView};

pub fn select_command(commands: Vec<cmds::Command>) -> cmds::Command {
    let select = create_select_view(commands);
    let mut siv = create_cursive_runnable(select);

    siv.run();

    siv.take_user_data().unwrap()
}

fn create_select_view(commands: Vec<cmds::Command>) -> SelectView<cmds::Command> {
    let mut select = SelectView::new().autojump();

    for command in commands {
        select.add_item(command.to_string(), command)
    }

    select.set_on_submit(|siv: &mut Cursive, cmd: &cmds::Command| {
        siv.quit();
        siv.set_user_data(cmd.clone());
    });

    select
}

fn create_cursive_runnable(select: SelectView<cmds::Command>) -> CursiveRunnable {
    let mut siv = cursive::default();
    let theme = custom_theme_from_cursive(&siv);
    siv.set_theme(theme);

    siv.add_layer(
        CircularFocus::new(Dialog::around(select.scrollable()).title("choice command"))
            .wrap_arrows(),
    );

    siv
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
