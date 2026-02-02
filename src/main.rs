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
        .subcommand(
            Command::new("done")
                .about("Finish done task")
                .arg(arg!([TaskNo]).required(true)),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let task_id: String = sub_matches.get_one::<String>("TaskID").unwrap().to_string();
            tasks.push(String::from(task_id));
        },
        Some(("list", _)) => {
            for task_id in tasks {
                println!("{}", task_id);
            }
        },
        Some(("done", sub_matches)) => {
            let task_no: String = sub_matches.get_one::<String>("TaskNo").unwrap().to_string();
            let task_no = task_no.parse::<i32>().unwrap();
            println!("{}", task_no);
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }


}