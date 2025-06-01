use std::{
    error::Error,
    sync::{Mutex, MutexGuard},
};

use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use directories::ProjectDirs;
use tauri::Manager;

pub mod commands;
pub mod error;
pub mod models;
pub mod schema;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn run_migrations(
    conn: &mut impl MigrationHarness<diesel::sqlite::Sqlite>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    conn.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}

pub struct App {
    pub db: Mutex<SqliteConnection>,
}

impl App {
    pub fn db<'a>(&'a self) -> MutexGuard<'a, SqliteConnection> {
        self.db.lock().unwrap()
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let database_path = if let Some(proj_dirs) = ProjectDirs::from("com", "task-tracker", "app") {
        let data_dir = proj_dirs.data_dir();
        std::fs::create_dir_all(data_dir).unwrap();
        data_dir.join("data.db").to_str().map(|v| v.to_string())
    } else {
        None
    };

    tauri::Builder::default()
        .setup(move |app| {
            let Some(database_path) = database_path else {
                return Err("could not find database path".into());
            };

            let mut db_connection = SqliteConnection::establish(&database_path)?;

            match db_connection.run_pending_migrations(MIGRATIONS) {
                Ok(_) => {}
                Err(_) => return Err("could not run migrations".into()),
            }

            app.manage(App {
                db: Mutex::new(db_connection),
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::list_projects,
            commands::create_project,
            commands::update_project,
            commands::delete_project,
            commands::start_task,
            commands::stop_task,
            commands::current_task,
            commands::update_task,
            commands::delete_task,
            commands::list_tasks_between,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
