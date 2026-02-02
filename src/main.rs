use clap::{arg, command, Command};

fn main() {
    let mut tasks: Vec<String> = Vec::new();

    let matches = command!() // requires `cargo` feature
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("Adds a todo task")
                .arg(arg!([TaskID]).required(true)),
        )
        .subcommand(
            Command::new("list")
                .about("List all todos"))
        .get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let task_id: &String = sub_matches.get_one::<String>("TaskID").unwrap();
            tasks.push(String::from(task_id));
        },
        Some(("list", _)) => {
            for task_id in tasks {
                println!("{}", task_id);
            }
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }


}