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
#[warn(non_snake_case)]
fn send_msg(inputText: String) -> String {
  let mut package = json!({
    "userId": "0",
    "sendUserId": "0",
    "client": true,
    "password": "hui_penis",
    "msg": inputText
  });

  let package_str = serde_json::to_string(&package).unwrap();
  let result = send_msg_to_server(package_str.as_str());

  if result.is_err() {
    format!("ERR: can't send message to server")
  } else {
    format!("LOG: preparing msg package to server: '{}'", package_str)
  }
}

fn send_msg_to_server(package_str: &str) -> io::Result<()> {
  loop {
    let mut stream = TcpStream::connect("localhost:1201")?;

    stream.write_all(package_str.as_bytes())?;


    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;

    println!("responce from server: '{}'", String::from_utf8_lossy(&buffer[..bytes_read]));
  }
}