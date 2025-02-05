use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIcon, TrayIconBuilder},
    AppHandle, Manager,
};

pub fn create_tray_menu(app: &AppHandle) -> TrayIcon {
    let show_item = MenuItem::with_id(app, "show", "Show", true, None::<&str>).unwrap();
    let update_item =
        MenuItem::with_id(app, "update", "Check for Updates", true, None::<&str>).unwrap();
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
                    // TODO: Implement update check
                }
                _ => {}
            }
        })
        .build(app)
        .unwrap();

    tray
}
