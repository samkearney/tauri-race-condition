// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{thread, time::Duration};

use serde::Serialize;
use tauri::{EventTarget, Manager};

#[derive(Clone, Debug, Serialize)]
struct EventPayload {
    number: i32,
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .on_page_load(|page, _| {
            let app_handle = page.app_handle().clone();
            thread::spawn(move || {
                let mut number: i32 = 1;

                loop {
                    app_handle
                        .emit_to(
                            EventTarget::webview_window("main"),
                            "myEvent",
                            EventPayload { number },
                        )
                        .unwrap();
                    number += 1;
                    thread::sleep(Duration::from_millis(50));
                }
            });
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
