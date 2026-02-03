use clap::{arg, command, Command};
use std::{fs, env, path::Path};
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    name: String,
    done: bool,
}

fn main() {
    let path = "./task_list.json";
    if !Path::new(path).exists() {
        fs::write(path, "[]").expect("Write failed!");
    }

    let contents = fs::read_to_string(path);
    let mut tasks: Vec<Task> = serde_json::from_str(contents.unwrap().as_str()).unwrap();

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
            tasks.push(Task {name:String::from(task_id), done:false});
            let result = serde_json::to_string(&tasks);
            fs::write("task_list.json", result.unwrap()).unwrap();
        },
        Some(("list", _)) => {
            for num in 0..tasks.len() {
                let have_done: bool = tasks[num].done;
                let have_done = if have_done { "已完成" } else { "未完成" };
                println!("[{}] - {} ({})",  num, tasks[num].name, have_done);
            }
        },
        Some(("done", sub_matches)) => {
            let task_no: String = sub_matches.get_one::<String>("TaskNo").unwrap().to_string();
            let task_no = task_no.parse::<usize>().unwrap();
            if task_no < tasks.len() {
                tasks[task_no].done = true;
                println!("[{}] - {} ({})",  task_no, tasks[task_no].name, "已完成");
                let result = serde_json::to_string(&tasks);
                fs::write("task_list.json", result.unwrap()).unwrap();
            } else {
                eprintln!("Task {} not found", task_no);
            }
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}