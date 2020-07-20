mod args;
mod cmds;
mod select;
mod exec;

fn main() {
    let content = args::get_content();
    let commands = cmds::Command::get_commands(content);
    let command = select::select_commands(&commands);
    println!("execute command: {}", command);
    exec::execute(&command.executable());
}
