use crate::model::trade_filter::TradeFilters;
use tauri::AppHandle;
use tauri::Emitter;
use tauri::Manager;
use tauri_plugin_clipboard_manager::ClipboardExt;

#[cfg(target_os = "windows")]
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, VK_C, VK_CONTROL,
};

pub fn handle_shortcut(
    app_handle: &AppHandle,
    parse_item_text: impl Fn(&str) -> Result<TradeFilters, String>,
) -> Result<(), String> {
    if !is_poe_active() {
        return Ok(());
    }

    let window = app_handle
        .get_webview_window("main")
        .ok_or("Main window not found")?;

    // Save current clipboard content
    let prev_clipboard = window.clipboard().read_text().unwrap_or_default();

    // Simulate Ctrl+C to copy item data
    #[cfg(target_os = "windows")]
    simulate_ctrl_c();

    // Small delay to ensure clipboard is updated
    std::thread::sleep(std::time::Duration::from_millis(50));

    let item_text = window
        .clipboard()
        .read_text()
        .map_err(|e| format!("Failed to read clipboard: {}", e))?;

    // Quick validation - if less than 4 lines, it can't be a valid item
    if item_text.lines().count() < 4 {
        // Restore clipboard and return early
        let _ = window.clipboard().write_text(prev_clipboard);
        return Ok(());
    }

    let filters = parse_item_text(&item_text)
        .map_err(|e| format!("Failed to parse item text: {}", e))?;

    window
        .emit("parsed_filters", serde_json::to_string(&filters).unwrap())
        .map_err(|e| format!("Failed to emit parsed filters: {}", e))?;

    // Show window
    let _ = window.unminimize();
    std::thread::sleep(std::time::Duration::from_millis(10));
    let _ = window.show();
    std::thread::sleep(std::time::Duration::from_millis(10));
    let _ = window.set_focus();

    // Restore previous clipboard content
    window
        .clipboard()
        .write_text(prev_clipboard)
        .map_err(|e| format!("Failed to restore clipboard: {}", e))?;

    Ok(())
}

#[cfg(target_os = "windows")]
fn is_poe_active() -> bool {
    use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowTextW};

    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.0 == 0 {
            return false;
        }

        let mut title = [0u16; 512];
        let len = GetWindowTextW(hwnd, &mut title);
        if len > 0 {
            let title = String::from_utf16_lossy(&title[..len as usize]);
            return title.contains("Path of Exile 2");
        }
    }
    false
}

#[cfg(not(target_os = "windows"))]
fn is_poe_active() -> bool {
    // TODO: Implement for other platforms
    true
}

#[cfg(target_os = "windows")]
fn simulate_ctrl_c() {
    unsafe {
        let inputs = vec![
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VK_CONTROL,
                        dwFlags: windows::Win32::UI::Input::KeyboardAndMouse::KEYBD_EVENT_FLAGS(0),
                        ..Default::default()
                    },
                },
            },
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VK_C,
                        dwFlags: windows::Win32::UI::Input::KeyboardAndMouse::KEYBD_EVENT_FLAGS(0),
                        ..Default::default()
                    },
                },
            },
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VK_C,
                        dwFlags: KEYEVENTF_KEYUP,
                        ..Default::default()
                    },
                },
            },
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VK_CONTROL,
                        dwFlags: KEYEVENTF_KEYUP,
                        ..Default::default()
                    },
                },
            },
        ];
        SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
    }
}

#[cfg(not(target_os = "windows"))]
fn simulate_ctrl_c() {
    // TODO: Implement for other platforms
    unimplemented!("Ctrl+C simulation not implemented for this platform");
}
