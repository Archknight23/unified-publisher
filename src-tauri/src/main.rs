// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bsky_sdk::BskyAgent;
use reqwest::Client;
use tokio;
use tauri::{Manager, State};
use std::sync::Mutex;

pub struct AppState {
  pub bsky_agent: Mutex<Option<BskyAgent>>,
}

impl AppState {
  pub fn new() -> Self {
    AppState {
      bsky_agent: Mutex::new(None),

    }
  }
}

#[tauri::command]
//login logic, basically takes in username and password as strings, and stores them for login in Tauri
async fn bsky_login(
  username:String,
  password:String,
  app_state: State<'_,AppState>,) -> Result<String, String> {
    let agent = BskyAgent::builder().build().await.map_err(|e| e.to_string())?;
    let session = agent.login(&username, &password).await.map_err(|e| e.to_string())?;
    //Stores agent with login in creds
    *app_state.bsky_agent.lock().unwrap() = Some(agent);
      Ok(format!("User {} has been authenticated.", username))
  }
#[tauri::command]
  fn greet() -> String {
  format!("Hello, you've been selected for beta testing - no there is no giftcard.")
}
  fn main() {
    tauri::Builder::default()
      .manage(AppState::new())
    //starts the show
      .invoke_handler(tauri::generate_handler![greet, bsky_login,])
      .run(tauri::generate_context!())
      .expect("Babe, it's broken.");
  }