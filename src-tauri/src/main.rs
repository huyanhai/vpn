// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod file_info;
mod runner;

use file_info::GitConfig;
use runner::Runner::{Running, VpnStatus, CLOSE_STATUS};

#[tauri::command]
fn git_config_ctx(handle: tauri::AppHandle, file_type: u8) -> String {
    let conf = GitConfig::new(handle);
    conf.read_file(file_type)
}

#[tauri::command]
fn write_config(handle: tauri::AppHandle, ctx: &str, file_type: u8) {
    let conf = GitConfig::new(handle);
    conf.write_file(ctx, file_type).unwrap();
}

#[tauri::command]
fn running(handle: tauri::AppHandle) -> u8 {
    unsafe {
        match CLOSE_STATUS {
            VpnStatus::CLOSE => {
                let running = Running::new(handle);
                running.run();
                1
            }
            VpnStatus::OPEN => 0,
        }
    }
}

#[tauri::command]
fn kill() {
    unsafe {
        CLOSE_STATUS = VpnStatus::CLOSE;
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            git_config_ctx,
            write_config,
            running,
            kill
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
