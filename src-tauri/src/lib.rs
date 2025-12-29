use std::vec::Vec;

use tauri_plugin_sql::{Migration, MigrationKind};


// Source for setting up sqlite to reference later
// https://github.com/FocusCookie/tauri-sqlite-example

// Look into drizzle ORM to use an object oriented database





// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Me hahahha!", name)
// }

// #[tauri::command]
// fn logdata(name: &str) {
//     let lines: Vec<&str> = name.lines().collect();

//     for (index, &value) in lines.iter().enumerate() {
//         println!("#{}, Value: {}", index + 1, value);
//     }
//     println!("{}", lines.len());
// }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let migrations = vec![Migration {
        version: 1,
        description: "create users table",
        sql: "CREATE TABLE IF NOT EXISTS users (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL,
                    email TEXT
                )",
        kind: MigrationKind::Up,
    }];

    tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:test.db", migrations)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
