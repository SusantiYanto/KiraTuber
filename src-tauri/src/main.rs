// Sembunyikan jendela konsol di Windows pada build release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Mutex,
};
use tauri::{
    LogicalSize, Manager, PhysicalPosition, PhysicalRect, PhysicalSize, Position, Size,
    WebviewWindow, WindowEvent,
};

const DEFAULT_WIDTH: f64 = 1140.0;
const DEFAULT_HEIGHT: f64 = 760.0;

#[derive(Default)]
struct WindowBoundsState {
    inner: Mutex<SavedBounds>,
    resizing: AtomicBool,
}

#[derive(Default, Clone, Copy)]
struct SavedBounds {
    work_maximized: bool,
    restore_pos: Option<PhysicalPosition<i32>>,
    restore_size: Option<PhysicalSize<u32>>,
}

struct ResizeGuard<'a>(&'a AtomicBool);

impl<'a> ResizeGuard<'a> {
    fn acquire(flag: &'a AtomicBool) -> Self {
        flag.store(true, Ordering::Release);
        Self(flag)
    }
}

impl Drop for ResizeGuard<'_> {
    fn drop(&mut self) {
        self.0.store(false, Ordering::Release);
    }
}

#[derive(serde::Serialize)]
struct WindowStateInfo {
    fullscreen: bool,
    maximized: bool,
    minimized: bool,
}

fn apply_win32_borderless(window: &WebviewWindow, enabled: bool) -> Result<(), String> {
    #[cfg(windows)]
    {
        use windows::Win32::Graphics::Dwm::{
            DwmSetWindowAttribute, DWMWA_BORDER_COLOR, DWMWA_COLOR_DEFAULT, DWMWA_COLOR_NONE,
            DWMWA_WINDOW_CORNER_PREFERENCE, DWMWCP_DEFAULT, DWMWCP_DONOTROUND,
        };

        let hwnd = window.hwnd().map_err(|e| e.to_string())?;
        let border_color = if enabled {
            DWMWA_COLOR_NONE
        } else {
            DWMWA_COLOR_DEFAULT
        };
        let corner_preference = if enabled {
            DWMWCP_DONOTROUND
        } else {
            DWMWCP_DEFAULT
        };

        // Pada Windows 10 (sebelum Build 22000), atribut DWM 33 & 34 belum tersedia dan
        // mengembalikan E_INVALIDARG — abaikan per atribut agar kompatibel di Win10 & Win11.
        unsafe {
            let _ = DwmSetWindowAttribute(
                hwnd,
                DWMWA_BORDER_COLOR,
                (&raw const border_color).cast(),
                std::mem::size_of_val(&border_color) as u32,
            );
            let _ = DwmSetWindowAttribute(
                hwnd,
                DWMWA_WINDOW_CORNER_PREFERENCE,
                (&raw const corner_preference).cast(),
                std::mem::size_of_val(&corner_preference) as u32,
            );
        }
    }

    #[cfg(not(windows))]
    let _ = (window, enabled);

    Ok(())
}

fn current_work_area(window: &WebviewWindow) -> Result<Option<PhysicalRect<i32, u32>>, String> {
    let monitor = window
        .current_monitor()
        .map_err(|e| e.to_string())?
        .or(window.primary_monitor().map_err(|e| e.to_string())?);
    Ok(monitor.map(|m| *m.work_area()))
}

fn matches_work_area(
    pos: PhysicalPosition<i32>,
    size: PhysicalSize<u32>,
    wa: PhysicalRect<i32, u32>,
) -> bool {
    pos.x.abs_diff(wa.position.x) <= 12
        && pos.y.abs_diff(wa.position.y) <= 12
        && size.width.abs_diff(wa.size.width) <= 12
        && size.height.abs_diff(wa.size.height) <= 12
}

fn sync_external_bounds(window: &WebviewWindow, state: &WindowBoundsState) {
    if state.resizing.load(Ordering::Acquire) {
        return;
    }
    let was_work_max = state
        .inner
        .try_lock()
        .map(|g| g.work_maximized)
        .unwrap_or(false);
    if !was_work_max || window.is_minimized().unwrap_or(false) {
        return;
    }
    if let (Ok(pos), Ok(size), Ok(Some(wa))) = (
        window.outer_position(),
        window.outer_size(),
        current_work_area(window),
    ) {
        if !matches_work_area(pos, size, wa) {
            if let Ok(mut guard) = state.inner.try_lock() {
                guard.work_maximized = false;
            }
            let _ = window.set_resizable(true);
        }
    }
}

#[tauri::command]
fn set_window_borderless(app: tauri::AppHandle, enabled: bool) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or_else(|| "Jendela utama tidak ditemukan.".to_string())?;
    apply_win32_borderless(&window, enabled)
}

#[tauri::command]
fn control_window(
    app: tauri::AppHandle,
    state: tauri::State<'_, WindowBoundsState>,
    action: &str,
) -> Result<WindowStateInfo, String> {
    let window = app
        .get_webview_window("main")
        .ok_or_else(|| "Jendela utama tidak ditemukan.".to_string())?;

    if action == "close" {
        window.close().map_err(|e| e.to_string())?;
        return Ok(WindowStateInfo {
            fullscreen: false,
            maximized: false,
            minimized: false,
        });
    }

    // Sinkronkan status lebih dulu tanpa menahan lock lintas panggilan Win32
    sync_external_bounds(&window, &state);

    // Salin state ke variabel lokal lalu segera lepaskan Mutex agar bebas deadlock WM_SIZE
    let mut saved = *state
        .inner
        .lock()
        .map_err(|_| "Status jendela sedang terkunci.".to_string())?;

    if action == "status" {
        let fullscreen = window.is_fullscreen().unwrap_or(false);
        let minimized = window.is_minimized().unwrap_or(false);
        let maximized =
            !fullscreen && (saved.work_maximized || window.is_maximized().unwrap_or(false));
        return Ok(WindowStateInfo {
            fullscreen,
            maximized,
            minimized,
        });
    }

    let is_fs = window.is_fullscreen().map_err(|e| e.to_string())?;
    let is_os_max = window.is_maximized().map_err(|e| e.to_string())?;
    let _resize_guard = ResizeGuard::acquire(&state.resizing);

    let capture_restore_bounds = |saved: &mut SavedBounds| {
        if !saved.work_maximized
            && !is_os_max
            && !is_fs
            && !window.is_minimized().unwrap_or(false)
        {
            if let (Ok(pos), Ok(size)) = (window.outer_position(), window.outer_size()) {
                if size.width > 0 && size.height > 0 {
                    saved.restore_pos = Some(pos);
                    saved.restore_size = Some(size);
                }
            }
        }
    };

    let do_maximize = |saved: &mut SavedBounds| -> Result<(), String> {
        capture_restore_bounds(saved);
        if is_fs {
            window.set_fullscreen(false).map_err(|e| e.to_string())?;
            let _ = apply_win32_borderless(&window, true);
        }
        if let Some(wa) = current_work_area(&window)? {
            if is_os_max {
                let _ = window.unmaximize();
            }
            window.set_resizable(true).map_err(|e| e.to_string())?;
            window
                .set_position(Position::Physical(wa.position))
                .map_err(|e| e.to_string())?;
            window
                .set_size(Size::Physical(wa.size))
                .map_err(|e| e.to_string())?;
            let _ = window.set_resizable(false);
            saved.work_maximized = true;
        } else {
            window.set_resizable(true).map_err(|e| e.to_string())?;
            window.maximize().map_err(|e| e.to_string())?;
            saved.work_maximized = true;
        }
        Ok(())
    };

    let do_restore = |saved: &mut SavedBounds| -> Result<(), String> {
        let need_bounds_restore = saved.work_maximized || is_fs;
        if is_fs {
            window.set_fullscreen(false).map_err(|e| e.to_string())?;
            let _ = apply_win32_borderless(&window, true);
        }
        if window.is_maximized().unwrap_or(false) {
            window.unmaximize().map_err(|e| e.to_string())?;
        }
        window.set_resizable(true).map_err(|e| e.to_string())?;
        saved.work_maximized = false;
        if need_bounds_restore {
            if let (Some(pos), Some(size)) = (saved.restore_pos, saved.restore_size) {
                window
                    .set_size(Size::Physical(size))
                    .map_err(|e| e.to_string())?;
                window
                    .set_position(Position::Physical(pos))
                    .map_err(|e| e.to_string())?;
            } else {
                window
                    .set_size(Size::Logical(LogicalSize::new(DEFAULT_WIDTH, DEFAULT_HEIGHT)))
                    .map_err(|e| e.to_string())?;
                let _ = window.center();
            }
        }
        Ok(())
    };

    match action {
        "minimize" => {
            window.minimize().map_err(|e| e.to_string())?;
        }
        "maximize" => {
            do_maximize(&mut saved)?;
        }
        "restore" | "windowed" => {
            do_restore(&mut saved)?;
        }
        "toggle_maximize" => {
            if saved.work_maximized || is_os_max || is_fs {
                do_restore(&mut saved)?;
            } else {
                do_maximize(&mut saved)?;
            }
        }
        "fullscreen" => {
            capture_restore_bounds(&mut saved);
            saved.work_maximized = false;
            let _ = window.set_resizable(true);
            window.set_fullscreen(true).map_err(|e| e.to_string())?;
        }
        other => return Err(format!("Aksi jendela tidak dikenal: {other}")),
    }

    // Simpan kembali state yang sudah diperbarui
    if let Ok(mut guard) = state.inner.lock() {
        *guard = saved;
    }

    let fullscreen = window.is_fullscreen().unwrap_or(false);
    let minimized = window.is_minimized().unwrap_or(false);
    let maximized = !fullscreen && (saved.work_maximized || window.is_maximized().unwrap_or(false));

    Ok(WindowStateInfo {
        fullscreen,
        maximized,
        minimized,
    })
}

fn main() {
    tauri::Builder::default()
        .manage(WindowBoundsState::default())
        .setup(|app| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_decorations(false);
                let _ = window.set_shadow(false);
                let _ = apply_win32_borderless(&window, true);
            }
            Ok(())
        })
        .on_window_event(|window, event| match event {
            WindowEvent::Resized(new_size) => {
                if new_size.width == 0 || new_size.height == 0 {
                    return; // Diabaikan saat sedang di-minimize ke taskbar
                }
                if let Some(state) = window.try_state::<WindowBoundsState>() {
                    if let Some(webview_win) =
                        window.app_handle().get_webview_window(window.label())
                    {
                        sync_external_bounds(&webview_win, &state);
                    }
                }
            }
            WindowEvent::Moved(_) => {
                if let Some(state) = window.try_state::<WindowBoundsState>() {
                    if let Some(webview_win) =
                        window.app_handle().get_webview_window(window.label())
                    {
                        sync_external_bounds(&webview_win, &state);
                    }
                }
            }
            _ => {}
        })
        .on_permission_request(|_, permission| match permission {
            tauri::webview::PermissionKind::Microphone => {
                tauri::webview::PermissionResponse::Allow
            }
            _ => tauri::webview::PermissionResponse::Default,
        })
        .invoke_handler(tauri::generate_handler![
            set_window_borderless,
            control_window
        ])
        .run(tauri::generate_context!())
        .expect("gagal menjalankan KiraTuber");
}
