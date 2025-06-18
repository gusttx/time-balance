mod commands;
mod database;
mod types;
mod utils;

use commands::{activity, entry};
use log::LevelFilter;
use tauri::Manager;
use tauri_plugin_log::{Target, TargetKind};

use crate::utils::DatabaseError;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle();

            let mut log_plugin = tauri_plugin_log::Builder::default().filter(|metadata| {
                metadata.target().starts_with("app_lib") || metadata.target().starts_with("webview")
            });

            if cfg!(debug_assertions) {
                log_plugin = log_plugin.level(LevelFilter::Debug)
            } else {
                log_plugin = log_plugin
                    .level(LevelFilter::Info)
                    .targets([Target::new(TargetKind::LogDir { file_name: None })])
            }

            handle.plugin(log_plugin.build())?;

            #[cfg(desktop)]
            handle.plugin(tauri_plugin_single_instance::init(|app, _, _| {
                let _ = app
                    .get_webview_window("main")
                    .expect("no main window")
                    .set_focus();
            }))?;

            tauri::async_runtime::block_on(async {
                let db = database::Database::new(&handle).await?.init().await?;
                handle.manage(db);

                Ok::<(), DatabaseError>(())
            })?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            activity::get_all_activities,
            activity::create_activity,
            activity::delete_activity,
            activity::edit_activity,
            entry::create_entry,
            entry::get_entry_page,
            entry::delete_entry
        ])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}
