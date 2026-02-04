use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize };
use std::fs;
use std::path::PathBuf;
use colored::*;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    name: String,
    done: bool,
}

const APP_NAME: &str = "todo";

/// 获取跨平台文件路径
fn get_data_path() -> PathBuf {
    let mut path = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(APP_NAME);
    fs::create_dir_all(&path).expect("Could not create data directory");
    path.push("task_list.json");
    path
}

/// 安全加载任务列表
fn load_tasks(path: &PathBuf) -> Vec<Task> {
    if !path.exists() {
        return Vec::new();
    }

    let content = match fs::read_to_string(path) {
        Ok(c) if c.trim().is_empty() => {
            return Vec::new();
        },
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };

    serde_json::from_str::<Vec<Task>>(&content).unwrap_or_else(|e| {
        eprintln!("Could not parse task file: {}", e);
        Vec::new()
    })
}

/// 安全保存任务列表
fn save_tasks(path: &PathBuf, tasks: &Vec<Task>) {
    let json = serde_json::to_string_pretty(tasks).expect("Failed to serialize tasks");
    fs::write(path, json).expect("Failed to save tasks");
}

#[derive(Parser)]
#[command(name = APP_NAME, about = "A simple CLI todo manager", version, author)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    Add {name: String},
    /// List all tasks
    List ,
    /// Mark a task as done
    Done {id: usize},
}


fn main() {
    let cli = Cli::parse();
    let data_path = get_data_path();
    let mut tasks = load_tasks(&data_path);

    match cli.command {
        Commands::Add { name } => {
            println!("Added task: {}", name);
            tasks.push(Task {name, done: false});
            save_tasks(&data_path, &tasks);
        }
        Commands::List => {
            if tasks.is_empty() {
                println!("No tasks. Add one with ‘todo add \"...\"’");
                return;
            }
            for (i, task) in tasks.iter().enumerate() {
                let mut display_name = task.name.clone();
                if task.done {
                    display_name = display_name.strikethrough().to_string();
                }
                let status = if task.done { "√" } else { " " };
                println!("{} [{}] {}", i + 1, status, display_name);
            }
        }
        Commands::Done { id } => {
            let index = id - 1;
            if index < tasks.len() {
                tasks[index].done = true;
                save_tasks(&data_path, &tasks);
                println!("Marked task {} as done", id);
            } else {
                eprintln!("Error: Task {} not found", id);
            }
        }
    }
}