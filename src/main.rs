mod args;
mod cmds;
mod exec;

use cursive::traits::*;
use cursive::views::{Dialog, SelectView};
use cursive::Cursive;

fn main() {
    let content = args::get_content();
    let commands = cmds::Command::get_commands(content);
    exec_command(commands)
}

fn exec_command(commands: Vec<cmds::Command>) {
    let mut select = SelectView::new().autojump();

    for command in commands {
        select.add_item(command.to_string(), command)
    }

    select.set_on_submit(|siv: &mut Cursive, cmd: &cmds::Command| {
        siv.quit();
        siv.set_user_data(cmd.clone());
    });

    let mut siv = cursive::default();

    siv.add_layer(
        Dialog::around(select.scrollable()).title("choice command"),
    );

    siv.set_user_data(exec_command);

    siv.run();

    let take_command: cmds::Command = siv.take_user_data().unwrap();
    println!("execute command: {}", take_command);
    exec::execute(&take_command.executable());
}
