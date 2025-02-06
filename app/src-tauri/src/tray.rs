use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIcon, TrayIconBuilder},
    AppHandle, Manager,
};
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_updater::UpdaterExt;

async fn handle_update(app: &AppHandle) {
    let updater = match app.updater() {
        Ok(updater) => updater,
        Err(e) => {
            show_dialog(app, "Error", &format!("Failed to initialize updater: {}", e));
            return;
        }
    };

    match updater.check().await {
        Ok(Some(update)) => {
            show_dialog(app, "Update Available", "Installing new version...");
            if let Err(e) = update.download_and_install(|_, _| {}, || {}).await {
                show_dialog(app, "Error", &format!("Failed to install update: {}", e));
            }
        }
        Ok(None) => {
            show_dialog(app, "Up to Date", "You're running the latest version!");
        }
        Err(e) => {
            show_dialog(app, "Error", &format!("Failed to check for updates: {}", e));
        }
    }
}

fn show_dialog(app: &AppHandle, title: &str, message: &str) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window
            .dialog()
            .message(message)
            .title(title)
            .blocking_show();
    }
}

pub fn create_tray_menu(app: &AppHandle) -> TrayIcon {
    let show_item = MenuItem::with_id(app, "show", "Show", true, None::<&str>).unwrap();
    let update_item = MenuItem::with_id(app, "update", "Update", true, None::<&str>).unwrap();
    let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>).unwrap();
    let menu = Menu::with_items(app, &[&show_item, &update_item, &quit_item]).unwrap();

    let tray = TrayIconBuilder::new()
        .menu(&menu)
        .icon(app.default_window_icon().unwrap().clone())
        .on_menu_event(|app, event| {
            match event.id.as_ref() {
                "quit" => {
                    app.exit(0);
                }
                "show" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
                "update" => {
                    tauri::async_runtime::block_on(handle_update(app));
                }
                _ => {}
            }
        })
        .build(app)
        .unwrap();

    tray
}
