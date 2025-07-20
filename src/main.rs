mod args;
mod cmds;
mod exec;
mod ui;

fn main() {
    let content = args::get_content();
    let commands = cmds::Command::get_commands(content);
    let selected_command = ui::select_command(commands);

    println!("execute command: {selected_command}");
    exec::execute(&selected_command.executable());
}
