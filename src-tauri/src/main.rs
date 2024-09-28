// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serde_json::json;
use std::io::{self, Write, Read};
use std::net::TcpStream;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![send_msg])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn send_msg(inputText: String) -> String {
  let mut package = json!({});

  if let Some(obj) = package.as_object_mut() {
    obj.insert("msg".to_string(), serde_json::Value::String(inputText));
    obj.insert("user".to_string(), serde_json::Value::String("Adisteyf".to_string()));
  }
  let package_str = serde_json::to_string(&package).unwrap();

  format!("preparing msg package to server: '{}'", package_str)
}

fn send_msg_to_server(package_str: String) -> io::Result<()> {
  let mut stream = TcpStream::connect("localhost:1200")?;

  stream.write_all(package_str.as_bytes())?;


  let mut buffer = [0; 1024];
  let bytes_read = stream.read(&mut buffer)?;

  println!("responce from server: '{}'", String::from_utf8_lossy(&buffer[..bytes_read]));
  Ok(())
}