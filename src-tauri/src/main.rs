
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use control::{opener, writer};
#[tauri::command]
fn greet(name: &str) -> String {
    format!("{}", name)
}
#[tauri::command]
fn srchange(name: &str) -> String {
    format!("{}", name)
 //   println!(name)
}
#[tauri::command]
fn log(logs: &str){
    println!("{}",logs)
}
#[tauri::command]
fn gener(name: &str) -> String{
    format!("{}", name)
}
#[tauri::command]
fn openfile(filename: &str) -> String{
    let prefix = "/home/james/Documents/Notes/";
    let filename = format!("{}{}", prefix, filename);
    let xer = opener(&filename);
    let mut a = String::new(); // Change `a` to be a mutable String
    for i in 0..xer.len() { // Iterate over indices
        let b = &xer[i]; // Access each element using index
        a.push_str(b);// Append `b` to `a`
        a.push_str(" ")
    }
    let c = a.to_string();
    return c
}
#[tauri::command]
fn writefile(filename: &str, content: &str) {
    let prefix = "/home/james/Documents/Notes/";
    let filename = format!("{}{}", prefix, filename);
    writer(&filename, content);
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![log])
        .invoke_handler(tauri::generate_handler![gener])
        .invoke_handler(tauri::generate_handler![srchange, openfile])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
