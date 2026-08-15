// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::fs;
use std::io::Write;
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::{Command, Stdio};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![find_servers, start_server])
        .run(tauri::generate_context!())
        .expect("error");
}

#[tauri::command]
fn find_servers() -> Vec<(String, String)> {
    let mut servers = vec![];

    let working_path = env::current_dir().unwrap();
    println!("The current directory is {}", working_path.display());
    let paths = fs::read_dir(working_path).unwrap();

    for path in paths {
        let pathstr = format!("{}", path.unwrap().path().display());
        let path3 = Path::new(&pathstr);
        let server_name_temp = path3.file_name().unwrap();
        if path3.is_dir() {
            let paths2 = fs::read_dir(path3).unwrap();
            for path2 in paths2 {
                let filestr = format!("{}", path2.unwrap().path().display());
                let file = Path::new(&filestr);
                if file.file_name().unwrap() == "server.jar" {
                    //println!("Name: {:?}", server_name);
                    let server_name = server_name_temp
                        .to_string_lossy()
                        .to_string()
                        .to_lowercase();

                    let server = (filestr, server_name);
                    servers.push(server);
                } else if file.file_name().unwrap() == "velocity.jar" {
                    //println!("Name: {:?}", server_name);
                    let server_name = "Velocity".to_string();
                    let server = (filestr, server_name);
                    servers.push(server);
                }
            }
        }
    }
    // for server in servers{
    //     println!("{:?}", server.0);
    //     println!("{:?}", server.1);
    // }
    if servers.len() == 0 {
        servers.push(("error".to_string(), "no Servers".to_string()))
    }
    servers
}

#[tauri::command]
fn start_server(strpath: &str, name: &str) {
    let jar_path = Path::new(strpath);
    println!("The current filepatgh is {}", jar_path.display());

    let server_folder = jar_path.parent().unwrap();
    println!("The current server folder is {}", server_folder.display());
    const CREATE_NO_WINDOW: u32 = 0x08000000;
    let mut server = Command::new("java")
        .arg("-jar")
        .arg(jar_path)
        .arg("nogui")
        .current_dir(server_folder)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        //.creation_flags(CREATE_NO_WINDOW)
        .spawn()
        .expect("ERROR: Failed to start the child process");

    // println!("Server started with PID: {}", server.id());

    // // Get access to the server console input
    // let mut stdin = server.stdin.take().unwrap();

    // // Send a Minecraft command
    // stdin.write(b"say Hello from Rust!\n").unwrap();
}
