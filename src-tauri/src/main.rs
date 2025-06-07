// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use tauri::{Manager, State};
use tauri_plugin_sqlite::{SqlitePlugin, Migration, MigrationKind};

// Define the default path for POS database:
fn get_db_path(app: &tauri::AppHandle) -> String {
  let mut path = app.path_resolver().app_data_dir().unwrap();
  path.push("pos.db");
  path.to_string_lossy().to_string()
}

#[derive(Serialize, Deserialize)]
struct Customer {
  id: i64,
  name: String,
  card_number: Option<String>,
  email: Option<String>,
  phone: Option<String>,
  balance: f64,
  total_debt: f64,
  created_at: String,
  updated_at: String,
}

#[tauri::command]
async fn get_all_customers(
  app_handle: tauri::AppHandle,
) -> Result<Vec<Customer>, String> {
  // Get the database path
  let db_path = get_db_path(&app_handle);

  // Open the DB (it auto‐creates the file if not exist)
  let conn = tauri_plugin_sqlite::init_schema(
    &app_handle,
    &db_path,
    "pos",

    vec![Migration {
      kind: MigrationKind::MigrationFolder,
      migrations_dir: "./prisma/migrations".into(),
    }],
  )
  .await
  .map_err(|e| format!("Failed to init schema: {:?}", e))?;

  let sql = "SELECT id, name, card_number, email, phone, balance, total_debt, created_at, updated_at FROM Customer;";
  let results = conn
    .select(tauri_plugin_sqlite::Value::QuestionMark, sql, None)
    .map_err(|e| format!("Query failed: {:?}", e))?;

  // Convert SQLite rows into Customer struct
  let customers = results
    .iter()
    .map(|row| {
      let cols = row.to_owned();
      Customer {
        id: cols[0].as_integer().unwrap(),
        name: cols[1].as_str().unwrap().to_string(),
        card_number: cols[2].as_str().map(|s| s.to_string()),
        email: cols[3].as_str().map(|s| s.to_string()),
        phone: cols[4].as_str().map(|s| s.to_string()),
        balance: cols[5].as_double().unwrap_or(0.0),
        total_debt: cols[6].as_double().unwrap_or(0.0),
        created_at: cols[7].as_str().unwrap().to_string(),
        updated_at: cols[8].as_str().unwrap().to_string(),
      }
    })
    .collect();

  Ok(customers)
}

fn main() {
  tauri::Builder::default()
    // Register the sqlite plugin, giving it a namespace "sqlite_plugin"
    .plugin(SqlitePlugin::default())
    // Register Tauri command so React can call get_all_customers
    .invoke_handler(tauri::generate_handler![get_all_customers])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
