use pomoru_core::{config, notification, status, task::TaskList, timer::TimerState};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::{Manager, State};

// Shared application state managed by Tauri.
pub struct AppState {
    pub timer: Mutex<TimerState>,
    pub tasks: Mutex<TaskList>,
}

// Serializable snapshot of the full app state sent to the frontend.
#[derive(Serialize)]
pub struct AppSnapshot {
    pub timer: TimerState,
    pub tasks: Vec<pomoru_core::task::Task>,
    pub selected_task: Option<usize>,
    pub active_task_title: Option<String>,
}

#[derive(Serialize)]
pub struct TickResult {
    pub timer: TimerState,
    pub event: Option<pomoru_core::timer::TimerEvent>,
}

// --- Timer Commands ---

#[tauri::command]
fn get_state(state: State<AppState>) -> AppSnapshot {
    let timer = state.timer.lock().unwrap();
    let tasks = state.tasks.lock().unwrap();
    AppSnapshot {
        timer: timer.clone(),
        tasks: tasks.items().to_vec(),
        selected_task: tasks.selected(),
        active_task_title: tasks.active_task().map(|t| t.title.clone()),
    }
}

#[tauri::command]
fn tick(state: State<AppState>) -> TickResult {
    let mut timer = state.timer.lock().unwrap();
    let event = timer.tick();

    // Export status for Waybar compatibility
    let _ = status::export_status(timer.mode, timer.time_remaining_secs);

    // Send notification if session completed
    if let Some(ref ev) = event {
        notification::send_notification(&ev.title, &ev.message);
    }

    TickResult {
        timer: timer.clone(),
        event,
    }
}

#[tauri::command]
fn start_timer(state: State<AppState>) {
    let mut timer = state.timer.lock().unwrap();
    timer.start();
    let _ = status::export_status(timer.mode, timer.time_remaining_secs);
}

#[tauri::command]
fn pause_timer(state: State<AppState>) {
    let mut timer = state.timer.lock().unwrap();
    timer.pause();
    let _ = status::export_status(timer.mode, timer.time_remaining_secs);
}

#[tauri::command]
fn toggle_timer(state: State<AppState>) {
    let mut timer = state.timer.lock().unwrap();
    timer.toggle_running();
    let _ = status::export_status(timer.mode, timer.time_remaining_secs);
}

#[tauri::command]
fn reset_timer(state: State<AppState>) {
    let mut timer = state.timer.lock().unwrap();
    timer.reset();
    let _ = status::export_status(timer.mode, timer.time_remaining_secs);
}

#[tauri::command]
fn skip_session(state: State<AppState>) {
    let mut timer = state.timer.lock().unwrap();
    timer.skip_session();
    let _ = status::export_status(timer.mode, timer.time_remaining_secs);
}

#[tauri::command]
fn cycle_mode(state: State<AppState>) {
    let mut timer = state.timer.lock().unwrap();
    timer.cycle_mode();
    let _ = status::export_status(timer.mode, timer.time_remaining_secs);
}

#[tauri::command]
fn toggle_auto_switch(state: State<AppState>) {
    let mut timer = state.timer.lock().unwrap();
    timer.toggle_auto_switch();
}

#[tauri::command]
fn set_duration(state: State<AppState>, mins: u64) {
    let mut timer = state.timer.lock().unwrap();
    timer.set_current_duration_mins(mins);
    let _ = status::export_status(timer.mode, timer.time_remaining_secs);
}

// --- Task Commands ---

#[tauri::command]
fn get_tasks(state: State<AppState>) -> Vec<pomoru_core::task::Task> {
    let tasks = state.tasks.lock().unwrap();
    tasks.items().to_vec()
}

#[tauri::command]
fn add_task(state: State<AppState>, title: String) {
    let mut tasks = state.tasks.lock().unwrap();
    tasks.add(title);
}

#[tauri::command]
fn edit_task(state: State<AppState>, index: usize, title: String) {
    let mut tasks = state.tasks.lock().unwrap();
    tasks.edit(index, title);
}

#[tauri::command]
fn delete_task(state: State<AppState>, index: usize) {
    let mut tasks = state.tasks.lock().unwrap();
    tasks.delete(index);
}

#[tauri::command]
fn toggle_task(state: State<AppState>, index: usize) {
    let mut tasks = state.tasks.lock().unwrap();
    tasks.toggle(index);
}

#[tauri::command]
fn reorder_task(state: State<AppState>, index: usize, direction: String) {
    let mut tasks = state.tasks.lock().unwrap();
    tasks.select(Some(index));
    match direction.as_str() {
        "up" => tasks.move_up(),
        "down" => tasks.move_down(),
        _ => {}
    }
}

#[tauri::command]
fn move_task(state: State<AppState>, from: usize, to: usize) {
    let mut tasks = state.tasks.lock().unwrap();
    tasks.move_task(from, to);
}

// --- Config Commands ---

#[tauri::command]
fn save_config(state: State<AppState>) -> Result<(), String> {
    let timer = state.timer.lock().unwrap();
    let tasks = state.tasks.lock().unwrap();
    let cfg = config::Config {
        work_time_mins: timer.work_time_secs / 60,
        short_break_mins: timer.short_break_secs / 60,
        long_break_mins: timer.long_break_secs / 60,
        tasks: tasks.items().to_vec(),
        auto_switch_sessions: timer.auto_switch_sessions,
    };
    config::save(&cfg).map_err(|e| e.to_string())
}

// --- Theme Commands ---

#[derive(Serialize)]
pub struct ThemeColors {
    pub primary: String,
    pub overlay0: String,
    pub surface0: String,
    pub text: String,
}

#[tauri::command]
fn get_theme() -> ThemeColors {
    use directories::BaseDirs;

    // Default Catppuccin Mocha-inspired colors
    let mut colors = ThemeColors {
        primary: "#b4befe".to_string(),
        overlay0: "#6c7086".to_string(),
        surface0: "#313244".to_string(),
        text: "#cdd6f4".to_string(),
    };

    // Try loading Noctalia overrides
    if let Some(base_dirs) = BaseDirs::new() {
        let path = base_dirs.home_dir().join(".config/noctalia/colors.json");
        if let Ok(content) = std::fs::read_to_string(&path) {
            #[derive(Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct NoctaliaColors {
                m_primary: String,
                m_on_surface_variant: String,
                m_surface_variant: String,
                m_on_surface: String,
            }

            if let Ok(noctalia) = serde_json::from_str::<NoctaliaColors>(&content) {
                colors.primary = noctalia.m_primary;
                colors.overlay0 = noctalia.m_on_surface_variant;
                colors.surface0 = noctalia.m_surface_variant;
                colors.text = noctalia.m_on_surface;
            }
        }
    }

    colors
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Load config on startup
    let cfg = config::load();
    let mut timer = TimerState::new();
    timer.work_time_secs = cfg.work_time_mins * 60;
    timer.short_break_secs = cfg.short_break_mins * 60;
    timer.long_break_secs = cfg.long_break_mins * 60;
    timer.auto_switch_sessions = cfg.auto_switch_sessions;
    timer.reset_timer_to_mode();

    let tasks = TaskList::from_tasks(cfg.tasks);

    // Initial status export
    let _ = status::export_status(timer.mode, timer.time_remaining_secs);

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            timer: Mutex::new(timer),
            tasks: Mutex::new(tasks),
        })
        .invoke_handler(tauri::generate_handler![
            get_state,
            tick,
            start_timer,
            pause_timer,
            toggle_timer,
            reset_timer,
            skip_session,
            cycle_mode,
            toggle_auto_switch,
            set_duration,
            get_tasks,
            add_task,
            edit_task,
            delete_task,
            toggle_task,
            reorder_task,
            move_task,
            save_config,
            get_theme,
        ])
        .setup(|app| {
            let quit_i = tauri::menu::MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = tauri::menu::Menu::with_items(app, &[&quit_i])?;

            tauri::tray::TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(|app, event| {
                    if event.id.as_ref() == "quit" {
                        app.exit(0);
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    if let tauri::tray::TrayIconEvent::Click {
                        button: tauri::tray::MouseButton::Left,
                        button_state: tauri::tray::MouseButtonState::Up,
                        ..
                    } = event {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            // Save config when window closes
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                if let Some(state) = window.try_state::<AppState>() {
                    let timer = state.timer.lock().unwrap();
                    let tasks = state.tasks.lock().unwrap();
                    let cfg = config::Config {
                        work_time_mins: timer.work_time_secs / 60,
                        short_break_mins: timer.short_break_secs / 60,
                        long_break_mins: timer.long_break_secs / 60,
                        tasks: tasks.items().to_vec(),
                        auto_switch_sessions: timer.auto_switch_sessions,
                    };
                    let _ = config::save(&cfg);
                    let _ = status::clear_status();
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
