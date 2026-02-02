use clap::{arg, command, Command};
use std::{fs, env, path::Path};
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    name: String,
}

fn main() {
    // Test code
    // let data = r#"[
    //     {"name": "吃饭"},
    //     {"name": "睡觉"}
    // ]"#;
    // let tasks: Vec<Task> = serde_json::from_str(data).expect("JSON was not well-formatted");
    // println!("{:?}", tasks);


    let path = "./task_list.json";
    if !Path::new(path).exists() {
        fs::write(path, "[]").expect("Write failed!");
    }

    let contents = fs::read_to_string(path);
    let mut tasks: Vec<String> = serde_json::from_str(contents.unwrap().as_str()).unwrap();

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
            let result = serde_json::to_string(&tasks);
            println!("{:?}", result);
            fs::write("task_list.json", result.unwrap()).unwrap();
        },
        Some(("list", _)) => {
            for task_id in tasks {
                println!("{}", task_id);
            }
        },
        Some(("done", sub_matches)) => {
            let task_no: String = sub_matches.get_one::<String>("TaskNo").unwrap().to_string();
            let task_no = task_no.parse::<i32>().unwrap();
            println!("{}", task_no)
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}