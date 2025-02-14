use reqwest::Client;
use tauri::Manager;
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use tauri_plugin_opener::OpenerExt;

use crate::mapping::mod_pattern_map::ModPatternMap;
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
    Client::builder().user_agent("chiseled-price-checker").build()
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

    let mod_pattern_map = ModPatternMap::new(stats_api_response)
        .map_err(|e| format!("Failed to create mod pattern map: {}", e))?;
    let base_type_map = BaseTypeMap::new(items_api_response)
        .map_err(|e| format!("Failed to create base type map: {}", e))?;

    let item_text_to_trade_filters = move |text: &str| -> Result<TradeFilters, String> {
        TradeFilters::from_text(
            |text, prefix| mod_pattern_map.mod_pattern_to_trade_stat(text, prefix),
            |text| base_type_map.item_text_to_base_type(text),
            text,
        )
    };

    let ctrl_d = Shortcut::new(Some(Modifiers::CONTROL), Code::KeyD);
    let shortcut_plugin = tauri_plugin_global_shortcut::Builder::new()
        .with_handler(move |_, shortcut, event| {
            if shortcut == &ctrl_d && event.state() == ShortcutState::Pressed {
                if let Err(e) = hotkey::handle_shortcut(&app_handle, &item_text_to_trade_filters) {
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
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            if let Err(e) = setup(app) {
                show_error(app.handle(), e);
                std::process::exit(1);
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![search_trade, minimize_window, open_trade_website])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn search_trade(filters: String, page: u32) -> Result<String, String> {
    let filters: TradeFilters =
        serde_json::from_str(&filters).map_err(|e| format!("Failed to parse filters: {}", e))?;

    let query = TradeQuery::from_trade_filters(&filters);
    let client = create_client().map_err(|e| format!("Failed to create HTTP client: {}", e))?;
    trade_api::search_trade(&client, &query, page).await
}

#[tauri::command]
async fn minimize_window(app_handle: tauri::AppHandle) {
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.hide();
    }
}

#[tauri::command]
async fn open_trade_website(filters: String, app_handle: tauri::AppHandle) -> Result<(), String> {
    let filters: TradeFilters =
        serde_json::from_str(&filters).map_err(|e| format!("Failed to parse filters: {}", e))?;

    let query = TradeQuery::from_trade_filters(&filters);

    let client = create_client().map_err(|e| format!("Failed to create HTTP client: {}", e))?;
    let query_id = trade_api::get_query_id(&client, &query).await.map_err(|e| format!("Failed to get query ID: {}", e))?;

    log::info!("Query ID: {}", query_id);

    let url = format!("https://www.pathofexile.com/trade2/search/poe2/Standard/{}", query_id);

    app_handle
        .opener()
        .open_url(url, None::<&str>)
        .map_err(|e| format!("Failed to open URL: {}", e))?;

    Ok(())
}
