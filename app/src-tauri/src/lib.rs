use reqwest::Client;
use tauri::Manager;
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

use crate::mapping::affix_map::AffixMap;
use crate::mapping::base_type_map::BaseTypeMap;
use crate::model::trade_filter::TradeFilters;
use crate::model::trade_query::TradeQuery;
use crate::service::trade_api;

mod hotkey;
mod mapping;
mod model;
mod service;
mod tray;

fn create_client() -> Result<Client, reqwest::Error> {
    Client::builder().user_agent("chiseled-poe-trade").build()
}

fn show_error(app_handle: &tauri::AppHandle, e: String) {
    log::error!("Error: {}", e);
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window
            .dialog()
            .message(format!("Error: {}", e))
            .title("Error")
            .kind(tauri_plugin_dialog::MessageDialogKind::Error)
            .blocking_show();
    }
}

fn setup(app: &mut tauri::App) -> Result<(), String> {
    let app_handle = app.handle().clone();
    let _tray = tray::create_tray_menu(&app_handle);

    let client = create_client().map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let (stats_api_response, items_api_response) = tauri::async_runtime::block_on(
        trade_api::fetch_mappings(&client)
    ).map_err(|e| format!("Failed to fetch trade site mappings. Are you logged in to the official trade site?\n\nError: {}", e))?;

    let affix_map = AffixMap::new(stats_api_response)
        .map_err(|e| format!("Failed to create affix map: {}", e))?;
    let base_type_map = BaseTypeMap::new(items_api_response)
        .map_err(|e| format!("Failed to create base type map: {}", e))?;

    let parse_item_text = move |text: &str| -> Result<TradeFilters, String> {
        TradeFilters::from_text(
            |text, prefix| affix_map.affix_to_trade_stat(text, prefix),
            |text| base_type_map.item_text_to_base_type(text),
            text,
        )
    };

    let ctrl_d = Shortcut::new(Some(Modifiers::CONTROL), Code::KeyD);
    let shortcut_plugin = tauri_plugin_global_shortcut::Builder::new()
        .with_handler(move |_, shortcut, event| {
            if shortcut == &ctrl_d && event.state() == ShortcutState::Pressed {
                if let Err(e) = hotkey::handle_shortcut(&app_handle, &parse_item_text) {
                    show_error(&app_handle, e);
                }
            }
        })
        .build();

    app.handle()
        .plugin(shortcut_plugin)
        .map_err(|e| format!("Failed to register shortcut plugin: {}", e))?;
    app.global_shortcut()
        .register(ctrl_d)
        .map_err(|e| format!("Failed to register global shortcut: {}", e))?;

    Ok(())
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_log::Builder::default().build())
        .setup(|app| {
            if let Err(e) = setup(app) {
                show_error(app.handle(), e);
                std::process::exit(1);
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![search_trade, minimize_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn search_trade(filters: String) -> Result<String, String> {
    let filters: TradeFilters =
        serde_json::from_str(&filters).map_err(|e| format!("Failed to parse filters: {}", e))?;

    let query = TradeQuery::from_trade_filters(&filters);
    let client = create_client().map_err(|e| format!("Failed to create HTTP client: {}", e))?;
    trade_api::search_trade(&client, &query).await
}

#[tauri::command]
async fn minimize_window(app_handle: tauri::AppHandle) {
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.minimize();
    }
}
