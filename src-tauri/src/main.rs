// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serde_json::json;
use std::io::{self, Write, Read};
use std::net::TcpStream;
use std::thread;

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
    "msg": inputText,
    "userId": "0",
    "password": "hui_penis",
    "sendUserId": "0",
    "client": true,
  });

  let package_str = serde_json::to_string(&package).unwrap();
  let result = send_msg_to_server(package_str.as_str());

  if result.is_err() {
    format!("ERR: can't send message to server")
  } else {
    format!("LOG: preparing msg package to server: '{}'", package_str.as_str())
  }
}

fn send_msg_to_server(package_str: &str) -> io::Result<()> {
  let package_str = String::from(package_str);
  let _handle = thread::spawn(move || {
    let mut stream = match TcpStream::connect("127.0.0.1:1201") {
      Ok(stream) => stream,
      Err(e) => {
        println!("LOG: failed to connect to server: {}", e);
        return;
      }
    };

    if let Err(e) = stream.write_all(package_str.as_bytes()) {
      println!("LOG: failed to send message to server: {}", e);
      return;
    }

    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
      Ok(bytes_read) => {
        println!("LOG: responce from server: '{}'", String::from_utf8_lossy(&buffer[..bytes_read]));
      },
      Err(e) => {
        println!("LOG: failed to read from server: {}", e);
        return;
      }
    }
  });
  Ok(())
}