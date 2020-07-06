mod args;
mod cmds;
mod select;
mod exec;

fn main() {
    let content = args::get_content();
    let commands = cmds::get_commands(content);
    let select = select::select_commands(&commands);
    println!("execute command: {}", commands[select]);
    exec::execute(&commands[select]);
}
