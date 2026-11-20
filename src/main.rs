#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder, EventLoopProxy},
    window::WindowBuilder,
};
#[cfg(target_os = "windows")]
use tao::platform::windows::{WindowBuilderExtWindows, WindowExtWindows};

#[cfg(target_os = "windows")]
extern "system" {
    fn CreatePopupMenu() -> isize;
    fn AppendMenuW(hmenu: isize, uflags: u32, uid: usize, lptext: *const u16) -> i32;
    fn TrackPopupMenu(
        hmenu: isize,
        uflags: u32,
        x: i32,
        y: i32,
        nreserved: i32,
        hwnd: isize,
        prc: *const i32,
    ) -> i32;
    fn DestroyMenu(hmenu: isize) -> i32;
    fn GetWindowRect(hwnd: isize, rect: *mut i32) -> i32;
    fn SetForegroundWindow(hwnd: isize) -> i32;
    fn GetWindow(hwnd: isize, ucmd: u32) -> isize;
    fn BringWindowToTop(hwnd: isize) -> i32;
    fn SetWindowPos(
        hwnd: isize,
        hwnd_insert_after: isize,
        x: i32,
        y: i32,
        cx: i32,
        cy: i32,
        flags: u32,
    ) -> i32;
    fn LoadLibraryA(lpfilename: *const u8) -> isize;
    fn GetProcAddress(hmodule: isize, lpprocname: *const u8) -> usize;
    fn SetCurrentProcessExplicitAppUserModelID(app_id: *const u16) -> i32;
}

#[cfg(target_os = "windows")]
const GW_CHILD: u32 = 5;
#[cfg(target_os = "windows")]
const HWND_TOP: isize = 0;
#[cfg(target_os = "windows")]
const SWP_NOSIZE: u32 = 0x0001;
#[cfg(target_os = "windows")]
const SWP_NOMOVE: u32 = 0x0002;
#[cfg(target_os = "windows")]
const SWP_NOACTIVATE: u32 = 0x0010;

#[cfg(target_os = "windows")]
fn enable_windows_dark_mode() {
    unsafe {
        let uxtheme = LoadLibraryA(b"uxtheme.dll\0".as_ptr());
        if uxtheme != 0 {
            let proc = GetProcAddress(uxtheme, 135 as _);
            if proc != 0 {
                let set_preferred_app_mode: unsafe extern "system" fn(i32) -> i32 =
                    std::mem::transmute(proc);
                set_preferred_app_mode(2);
            }
        }
    }
}

#[cfg(target_os = "windows")]
fn set_app_user_model_id() {
    unsafe {
        let app_id: Vec<u16> = "Dardcor.Nexora.Browser\0".encode_utf16().collect();
        let _ = SetCurrentProcessExplicitAppUserModelID(app_id.as_ptr());
    }
}

fn spawn_new_window(incognito: bool) {
    if let Ok(exe) = std::env::current_exe() {
        let mut cmd = std::process::Command::new(exe);
        if incognito {
            cmd.arg("--incognito");
        }
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            cmd.creation_flags(0x08000000);
        }
        let _ = cmd.spawn();
    }
}

#[cfg(target_os = "windows")]
fn show_native_more_menu(hwnd: isize, scale_factor: f64) -> Option<u32> {
    const MF_STRING: u32 = 0x00000000;
    const MF_SEPARATOR: u32 = 0x00000800;
    const TPM_RIGHTALIGN: u32 = 0x0008;
    const TPM_TOPALIGN: u32 = 0x0000;
    const TPM_RETURNCMD: u32 = 0x0100;
    const TPM_NONOTIFY: u32 = 0x0080;

    fn to_wide(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(std::iter::once(0)).collect()
    }

    unsafe {
        let hmenu = CreatePopupMenu();
        if hmenu == 0 {
            return None;
        }

        let add_item = |id: usize, text: &str| {
            let wide = to_wide(text);
            AppendMenuW(hmenu, MF_STRING, id, wide.as_ptr());
        };
        let add_separator = || {
            AppendMenuW(hmenu, MF_SEPARATOR, 0, std::ptr::null());
        };

        add_item(1, "New Tab\tCtrl+T");
        add_item(2, "New Window\tCtrl+N");
        add_item(12, "New Incognito Window\tCtrl+Shift+N");
        add_separator();
        add_item(3, "History\tCtrl+H");
        add_item(4, "Bookmarks\tCtrl+Shift+O");
        add_item(5, "Downloads\tCtrl+J");
        add_separator();
        add_item(6, "Zoom In (+10%)\tCtrl++");
        add_item(7, "Zoom Out (-10%)\tCtrl+-");
        add_item(8, "Reset Zoom (100%)\tCtrl+0");
        add_separator();
        add_item(13, "Find in Page...\tCtrl+F");
        add_item(14, "Toggle Reader Mode\tAlt+R");
        add_item(15, "Print...\tCtrl+P");
        add_item(16, "Fullscreen\tF11");
        add_separator();
        add_item(9, "Settings");
        add_item(10, "About Nexora");
        add_separator();
        add_item(11, "Exit\tAlt+F4");

        let mut rect = [0i32; 4];
        GetWindowRect(hwnd, rect.as_mut_ptr());

        let x = rect[2] - (10.0 * scale_factor) as i32;
        let y = rect[1] + (TOOLBAR_LOGICAL_HEIGHT * scale_factor) as i32;

        SetForegroundWindow(hwnd);

        let cmd = TrackPopupMenu(
            hmenu,
            TPM_RIGHTALIGN | TPM_TOPALIGN | TPM_RETURNCMD | TPM_NONOTIFY,
            x,
            y,
            0,
            hwnd,
            std::ptr::null(),
        );

        DestroyMenu(hmenu);

        if cmd > 0 {
            Some(cmd as u32)
        } else {
            None
        }
    }
}

#[cfg(target_os = "windows")]
fn show_native_tab_context_menu(hwnd: isize, screen_x: i32, screen_y: i32) -> Option<u32> {
    const MF_STRING: u32 = 0x00000000;
    const MF_SEPARATOR: u32 = 0x00000800;
    const TPM_LEFTALIGN: u32 = 0x0000;
    const TPM_TOPALIGN: u32 = 0x0000;
    const TPM_RETURNCMD: u32 = 0x0100;
    const TPM_NONOTIFY: u32 = 0x0080;

    fn to_wide(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(std::iter::once(0)).collect()
    }

    unsafe {
        let hmenu = CreatePopupMenu();
        if hmenu == 0 {
            return None;
        }

        let add_item = |id: usize, text: &str| {
            let wide = to_wide(text);
            AppendMenuW(hmenu, MF_STRING, id, wide.as_ptr());
        };
        let add_separator = || {
            AppendMenuW(hmenu, MF_SEPARATOR, 0, std::ptr::null());
        };

        add_item(1, "New Tab to the Right\tCtrl+T");
        add_item(2, "Duplicate Tab");
        add_item(3, "Reload Tab\tCtrl+R");
        add_item(4, "Pin Tab");
        add_separator();
        add_item(5, "Close Tab\tCtrl+W");
        add_item(6, "Close Other Tabs");
        add_item(7, "Close Tabs to the Right");

        SetForegroundWindow(hwnd);

        let cmd = TrackPopupMenu(
            hmenu,
            TPM_LEFTALIGN | TPM_TOPALIGN | TPM_RETURNCMD | TPM_NONOTIFY,
            screen_x,
            screen_y,
            0,
            hwnd,
            std::ptr::null(),
        );

        DestroyMenu(hmenu);

        if cmd > 0 {
            Some(cmd as u32)
        } else {
            None
        }
    }
}

use wry::{
    dpi::{PhysicalPosition, PhysicalSize},
    Rect, WebViewBuilder,
};

use nexora::app::NexoraApp;
use nexora::engine::NanoReader;
use nexora::security::AntiFingerprint;
use nexora::ui::NexoraUiAssets;

const TOOLBAR_LOGICAL_HEIGHT: f64 = 74.0;
const DESKTOP_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";

#[derive(Debug, Clone)]
pub enum BrowserAction {
    Navigate(String),
    GoBack,
    GoForward,
    Reload,
    NewTab(String),
    CloseTab(String),
    SwitchTab(String),
    UpdateUrl(String),
    UpdateTitle(String),
    UpdateShield(usize),
    DragWindow,
    MinimizeWindow,
    ToggleMaximizeWindow,
    CloseWindow,
    TabNavigated { tab_id: String, url: String },
    TabLoadFinished { tab_id: String, url: String },
    TabTitleChanged { tab_id: String, title: String },
    ContentIpc { tab_id: String, body: String },
    ToggleBookmark { url: String, title: String },
    OmniboxQuery(String),
    ExpandOverlay(f64),
    CollapseOverlay,
    SetZoom(u32),
    OpenMoreMenu,
    ReopenClosedTab,
    OpenTabContextMenu { tab_id: String, screen_x: i32, screen_y: i32 },
    ToggleReaderMode,
    FindInPage { query: String, forward: bool },
    StopFind,
    PrintPage,
    ToggleFullscreen,
    GetProfiles,
    SwitchProfile(String),
    AddProfile { name: String, email: String, badge: String, color: String },
    CustomizeProfile { name: String, color: String },
    SignOut,
    BookmarksBarToggled(bool),
    SyncTabs(Vec<nexora::vault::OpenTabRecord>),
    SaveTabGroup(nexora::vault::TabGroupRecord),
    DeleteTabGroup(String),
}

fn create_tab_webview(
    window: &tao::window::Window,
    tab_id: &str,
    initial_url: &str,
    bounds: Rect,
    proxy: EventLoopProxy<BrowserAction>,
    security: Arc<nexora::security::NexoraSecurityManager>,
    db: Arc<nexora::vault::NexoraDatabase>,
) -> Result<wry::WebView, Box<dyn std::error::Error>> {
    let stealth_script = AntiFingerprint::get_stealth_injection_script();

    let proxy_nav = proxy.clone();
    let tab_id_nav = tab_id.to_string();
    let security_nav = Arc::clone(&security);

    let proxy_load = proxy.clone();
    let tab_id_load = tab_id.to_string();

    let proxy_title = proxy.clone();
    let tab_id_title = tab_id.to_string();

    let proxy_ipc = proxy.clone();
    let tab_id_ipc = tab_id.to_string();

    let db_start = Arc::clone(&db);
    let db_finish = Arc::clone(&db);

    let mut builder = WebViewBuilder::new()
        .with_bounds(bounds)
        .with_user_agent(DESKTOP_USER_AGENT)
        .with_devtools(true)
        .with_initialization_script(stealth_script)
        .with_download_started_handler(move |url: String, path: &mut std::path::PathBuf| -> bool {
            let default_dir = directories::UserDirs::new()
                .and_then(|u| u.download_dir().map(|p| p.to_path_buf()))
                .unwrap_or_else(std::env::temp_dir);

            let filename = path
                .file_name()
                .map(|f| f.to_string_lossy().to_string())
                .unwrap_or_else(|| {
                    url::Url::parse(&url)
                        .ok()
                        .and_then(|u| {
                            u.path_segments()
                                .and_then(|mut segs| segs.next_back())
                                .map(|s| s.to_string())
                        })
                        .filter(|s| !s.is_empty())
                        .unwrap_or_else(|| "downloaded_file".to_string())
                });

            let target_path = default_dir.join(&filename);
            *path = target_path.clone();

            let id = uuid::Uuid::new_v4().to_string();
            let _ = db_start.record_download(
                &id,
                &filename,
                &url,
                &target_path.to_string_lossy(),
                "downloading",
            );
            true
        })
        .with_download_completed_handler(move |_url: String, path: Option<std::path::PathBuf>, success: bool| {
            if let Some(p) = path {
                let status = if success { "completed" } else { "failed" };
                let size = std::fs::metadata(&p).map(|m| m.len() as i64).unwrap_or(0);
                let _ = db_finish.update_download_status(&p.to_string_lossy(), status, size);
            }
        })
        .with_navigation_handler(move |url: String| -> bool {
            if url.starts_with("nexora://") {
                let _ = proxy_nav.send_event(BrowserAction::Navigate(url));
                return false;
            }

            if url.starts_with("data:") {
                return true;
            }

            let (should_allow, cleaned_url) = if security_nav.shield.should_block(&url) {
                let (count, _) = security_nav.shield.get_statistics();
                let _ = proxy_nav.send_event(BrowserAction::UpdateShield(count));
                (false, url)
            } else {
                let cleaned = nexora::security::PrivacyHeaders::strip_tracking_params(&url);
                (true, cleaned)
            };

            let _ = proxy_nav.send_event(BrowserAction::TabNavigated {
                tab_id: tab_id_nav.clone(),
                url: cleaned_url,
            });
            should_allow
        })
        .with_on_page_load_handler(move |event, url: String| {
            if matches!(event, wry::PageLoadEvent::Finished) {
                let _ = proxy_load.send_event(BrowserAction::TabLoadFinished {
                    tab_id: tab_id_load.clone(),
                    url,
                });
            }
        })
        .with_document_title_changed_handler(move |title: String| {
            let _ = proxy_title.send_event(BrowserAction::TabTitleChanged {
                tab_id: tab_id_title.clone(),
                title,
            });
        })
        .with_ipc_handler(move |req: wry::http::Request<String>| {
            let body = req.body().to_string();
            let _ = proxy_ipc.send_event(BrowserAction::ContentIpc {
                tab_id: tab_id_ipc.clone(),
                body,
            });
        });

    if initial_url.starts_with("nexora://") || initial_url.is_empty() {
        match initial_url {
            "nexora://history" => {
                builder = builder.with_html(NexoraUiAssets::get_history_html());
            }
            "nexora://bookmarks" => {
                builder = builder.with_html(NexoraUiAssets::get_bookmarks_html());
            }
            "nexora://settings" => {
                builder = builder.with_html(NexoraUiAssets::get_settings_html());
            }
            "nexora://downloads" => {
                builder = builder.with_html(NexoraUiAssets::get_downloads_html());
            }
            _ => {
                builder = builder.with_html(NexoraUiAssets::get_standalone_home_html());
            }
        }
    } else {
        builder = builder.with_url(initial_url);
    }

    let webview = builder.build_as_child(window)?;
    Ok(webview)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    {
        enable_windows_dark_mode();
        set_app_user_model_id();
    }

    let is_incognito = std::env::args().any(|a| a == "--incognito");
    let app = Arc::new(NexoraApp::new_with_mode(is_incognito)?);

    let event_loop = EventLoopBuilder::<BrowserAction>::with_user_event().build();
    let proxy: EventLoopProxy<BrowserAction> = event_loop.create_proxy();

    let icon_bytes = include_bytes!("../assets/icon-32.rgba");
    let window_icon = tao::window::Icon::from_rgba(icon_bytes.to_vec(), 32, 32).ok();

    let win_title = if is_incognito {
        "Nexora (Incognito)"
    } else {
        "Nexora"
    };

    #[cfg(target_os = "windows")]
    let builder = WindowBuilder::new()
        .with_title(win_title)
        .with_window_icon(window_icon)
        .with_decorations(false)
        .with_undecorated_shadow(true)
        .with_resizable(true)
        .with_inner_size(tao::dpi::LogicalSize::new(1280.0, 800.0))
        .with_min_inner_size(tao::dpi::LogicalSize::new(600.0, 400.0));

    #[cfg(not(target_os = "windows"))]
    let builder = WindowBuilder::new()
        .with_title(win_title)
        .with_window_icon(window_icon)
        .with_decorations(false)
        .with_resizable(true)
        .with_inner_size(tao::dpi::LogicalSize::new(1280.0, 800.0))
        .with_min_inner_size(tao::dpi::LogicalSize::new(600.0, 400.0));

    let window = builder.build(&event_loop)?;

    let scale_factor = window.scale_factor();
    let initial_physical = window.inner_size();
    let initial_width = initial_physical.width;
    let initial_height = initial_physical.height;

    let show_bm_bar_setting = app.db.get_setting("show_bookmarks_bar", "false") == "true";
    let initial_logical_h = if show_bm_bar_setting { TOOLBAR_LOGICAL_HEIGHT + 28.0 } else { TOOLBAR_LOGICAL_HEIGHT };
    let initial_toolbar_h = (initial_logical_h * scale_factor) as u32;
    let initial_content_h = initial_height.saturating_sub(initial_toolbar_h);

    let mut content_bounds = Rect {
        position: PhysicalPosition::new(0, initial_toolbar_h as i32).into(),
        size: PhysicalSize::new(initial_width, initial_content_h).into(),
    };

    let proxy_toolbar = proxy.clone();
    let toolbar_html = NexoraUiAssets::get_inlined_toolbar_html();

    let toolbar_webview = WebViewBuilder::new()
        .with_transparent(true)
        .with_bounds(Rect {
            position: PhysicalPosition::new(0, 0).into(),
            size: PhysicalSize::new(initial_width, initial_toolbar_h).into(),
        })
        .with_html(toolbar_html)
        .with_ipc_handler(move |req: wry::http::Request<String>| {
            if let Ok(v) = serde_json::from_str::<Value>(req.body()) {
                if let Some(msg_type) = v.get("type").and_then(|t| t.as_str()) {
                    match msg_type {
                        "navigate" => {
                            if let Some(url) = v.get("url").and_then(|u| u.as_str()) {
                                let _ = proxy_toolbar.send_event(BrowserAction::Navigate(url.to_string()));
                            }
                        }
                        "go_back" => {
                            let _ = proxy_toolbar.send_event(BrowserAction::GoBack);
                        }
                        "go_forward" => {
                            let _ = proxy_toolbar.send_event(BrowserAction::GoForward);
                        }
                        "reload" => {
                            let _ = proxy_toolbar.send_event(BrowserAction::Reload);
                        }
                        "new_tab" => {
                            let id = v.get("id").and_then(|i| i.as_str()).unwrap_or("tab-1");
                            let _ = proxy_toolbar.send_event(BrowserAction::NewTab(id.to_string()));
                        }
                        "new_window" => {
                            spawn_new_window(false);
                        }
                        "new_incognito" => {
                            spawn_new_window(true);
                        }
                        "close_tab" => {
                            if let Some(id) = v.get("id").and_then(|i| i.as_str()) {
                                let _ = proxy_toolbar.send_event(BrowserAction::CloseTab(id.to_string()));
                            }
                        }
                        "switch_tab" => {
                            if let Some(id) = v.get("id").and_then(|i| i.as_str()) {
                                let _ = proxy_toolbar.send_event(BrowserAction::SwitchTab(id.to_string()));
                            }
                        }
                        "bookmarks_bar_toggled" => {
                            let visible = v.get("visible").and_then(|b| b.as_bool()).unwrap_or(true);
                            let _ = proxy_toolbar.send_event(BrowserAction::BookmarksBarToggled(visible));
                        }
                        "drag_window" => {
                            let _ = proxy_toolbar.send_event(BrowserAction::DragWindow);
                        }
                        "minimize_window" => {
                            let _ = proxy_toolbar.send_event(BrowserAction::MinimizeWindow);
                        }
                        "toggle_maximize_window" => {
                            let _ = proxy_toolbar.send_event(BrowserAction::ToggleMaximizeWindow);
                        }
                        "close_window" => {
                            let _ = proxy_toolbar.send_event(BrowserAction::CloseWindow);
                        }
                        "toggle_bookmark" => {
                            let url = v.get("url").and_then(|u| u.as_str()).unwrap_or("").to_string();
                            let title = v.get("title").and_then(|t| t.as_str()).unwrap_or(&url).to_string();
                            let _ = proxy_toolbar.send_event(BrowserAction::ToggleBookmark { url, title });
                        }
                        "omnibox_query" => {
                            if let Some(q) = v.get("query").and_then(|q| q.as_str()) {
                                let _ = proxy_toolbar.send_event(BrowserAction::OmniboxQuery(q.to_string()));
                            }
                        }
                        "expand_overlay" => {
                            let h = v.get("height").and_then(|h| h.as_f64()).unwrap_or(360.0);
                            let _ = proxy_toolbar.send_event(BrowserAction::ExpandOverlay(h));
                        }
                        "collapse_overlay" => {
                            let _ = proxy_toolbar.send_event(BrowserAction::CollapseOverlay);
                        }
                        "set_zoom" => {
                            let level = v.get("level").and_then(|l| l.as_u64()).unwrap_or(100) as u32;
                            let _ = proxy_toolbar.send_event(BrowserAction::SetZoom(level));
                        }
                        "open_more_menu" => {
                            let _ = proxy_toolbar.send_event(BrowserAction::OpenMoreMenu);
                        }
                        "reopen_closed_tab" => {
                            let _ = proxy_toolbar.send_event(BrowserAction::ReopenClosedTab);
                        }
                        "tab_context_menu" => {
                            let tab_id = v.get("tab_id").and_then(|i| i.as_str()).unwrap_or("").to_string();
                            let sx = v.get("screen_x").and_then(|x| x.as_i64()).unwrap_or(0) as i32;
                            let sy = v.get("screen_y").and_then(|y| y.as_i64()).unwrap_or(0) as i32;
                            let _ = proxy_toolbar.send_event(BrowserAction::OpenTabContextMenu {
                                tab_id,
                                screen_x: sx,
                                screen_y: sy,
                            });
                        }
                        "toggle_reader" => {
                            let _ = proxy_toolbar.send_event(BrowserAction::ToggleReaderMode);
                        }
                        "find_in_page" => {
                            let q = v.get("query").and_then(|s| s.as_str()).unwrap_or("").to_string();
                            let forward = v.get("forward").and_then(|b| b.as_bool()).unwrap_or(true);
                            let _ = proxy_toolbar.send_event(BrowserAction::FindInPage { query: q, forward });
                        }
                        "stop_find" => {
                            let _ = proxy_toolbar.send_event(BrowserAction::StopFind);
                        }
                        "get_profiles" => {
                            let _ = proxy_toolbar.send_event(BrowserAction::GetProfiles);
                        }
                        "switch_profile" => {
                            if let Some(id) = v.get("id").and_then(|i| i.as_str()) {
                                let _ = proxy_toolbar.send_event(BrowserAction::SwitchProfile(id.to_string()));
                            }
                        }
                        "add_profile" => {
                            let name = v.get("name").and_then(|n| n.as_str()).unwrap_or("New Profile").to_string();
                            let email = v.get("email").and_then(|e| e.as_str()).unwrap_or("").to_string();
                            let badge = v.get("badge").and_then(|b| b.as_str()).unwrap_or("").to_string();
                            let color = v.get("color").and_then(|c| c.as_str()).unwrap_or("#4285f4").to_string();
                            let _ = proxy_toolbar.send_event(BrowserAction::AddProfile { name, email, badge, color });
                        }
                        "customize_profile" => {
                            if let Some(name) = v.get("name").and_then(|n| n.as_str()) {
                                let color = v.get("color").and_then(|c| c.as_str()).unwrap_or("").to_string();
                                let _ = proxy_toolbar.send_event(BrowserAction::CustomizeProfile { name: name.to_string(), color });
                            }
                        }
                        "sign_out" => {
                            let _ = proxy_toolbar.send_event(BrowserAction::SignOut);
                        }
                        "sync_tabs" => {
                            if let Some(tabs_val) = v.get("tabs") {
                                if let Ok(tabs) = serde_json::from_value::<Vec<nexora::vault::OpenTabRecord>>(tabs_val.clone()) {
                                    let _ = proxy_toolbar.send_event(BrowserAction::SyncTabs(tabs));
                                }
                            }
                        }
                        "save_tab_group" => {
                            if let Some(group_val) = v.get("group") {
                                if let Ok(group) = serde_json::from_value::<nexora::vault::TabGroupRecord>(group_val.clone()) {
                                    let _ = proxy_toolbar.send_event(BrowserAction::SaveTabGroup(group));
                                }
                            }
                        }
                        "delete_tab_group" => {
                            if let Some(group_id) = v.get("group_id").and_then(|g| g.as_str()) {
                                let _ = proxy_toolbar.send_event(BrowserAction::DeleteTabGroup(group_id.to_string()));
                            }
                        }
                        _ => {}
                    }
                }
            }
        })
        .build_as_child(&window)?;

    #[cfg(target_os = "windows")]
    let toolbar_hwnd = unsafe { GetWindow(window.hwnd() as isize, GW_CHILD) };

    let bring_toolbar_to_top = move || {
        #[cfg(target_os = "windows")]
        if toolbar_hwnd != 0 {
            unsafe {
                SetWindowPos(
                    toolbar_hwnd,
                    HWND_TOP,
                    0,
                    0,
                    0,
                    0,
                    SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
                );
                BringWindowToTop(toolbar_hwnd);
            }
        }
    };

    if is_incognito {
        let _ = toolbar_webview.evaluate_script("if (window.__nexora_set_incognito) window.__nexora_set_incognito(true);");
    }

    let current_engine = app.db.get_setting("search_engine", "google");
    let init_settings_script = format!(
        "if (window.__nexora_update_settings) window.__nexora_update_settings({{ search_engine: {:?} }});",
        current_engine
    );
    let _ = toolbar_webview.evaluate_script(&init_settings_script);

    if let Ok(profiles) = app.db.get_profiles() {
        if let Ok(json_str) = serde_json::to_string(&profiles) {
            let script = format!("if (window.__nexora_update_profiles) window.__nexora_update_profiles({});", json_str);
            let _ = toolbar_webview.evaluate_script(&script);
        }
    }

    if let Ok(bookmarks) = app.db.get_bookmarks_bar() {
        if let Ok(json) = serde_json::to_string(&bookmarks) {
            let script = format!("if (window.__nexora_render_bookmarks_bar) window.__nexora_render_bookmarks_bar({});", json);
            let _ = toolbar_webview.evaluate_script(&script);
        }
    }

    let bm_toggle_script = format!("if (window.__nexora_toggle_bookmarks_bar_visibility) window.__nexora_toggle_bookmarks_bar_visibility({});", show_bm_bar_setting);
    let _ = toolbar_webview.evaluate_script(&bm_toggle_script);

    let active_prof_id = app.db.get_active_profile().ok().flatten().map(|p| p.id).unwrap_or_else(|| "prof-default".to_string());
    let saved_groups = if !is_incognito { app.db.get_tab_groups().unwrap_or_default() } else { Vec::new() };
    let saved_tabs = if !is_incognito { app.db.get_open_tabs(Some(&active_prof_id)).unwrap_or_default() } else { Vec::new() };

    if !saved_groups.is_empty() {
        if let Ok(json_str) = serde_json::to_string(&saved_groups) {
            let script = format!("if (window.__nexora_restore_tab_groups) window.__nexora_restore_tab_groups({});", json_str);
            let _ = toolbar_webview.evaluate_script(&script);
        }
    }

    let mut tab_webviews: HashMap<String, wry::WebView> = HashMap::new();
    let mut tab_urls: HashMap<String, String> = HashMap::new();
    let mut active_tab_id: String = "tab-1".to_string();
    let mut closed_tabs: Vec<(String, String)> = Vec::new();
    let mut is_bookmarks_bar_visible = show_bm_bar_setting;

    if !saved_tabs.is_empty() {
        let mut active_found = false;
        for (idx, tab) in saved_tabs.iter().enumerate() {
            let is_active = tab.is_active || (idx == 0 && !active_found);
            if is_active {
                active_tab_id = tab.id.clone();
                active_found = true;
            }
            if let Ok(wv) = create_tab_webview(
                &window,
                &tab.id,
                &tab.url,
                content_bounds,
                proxy.clone(),
                Arc::clone(&app.security),
                Arc::clone(&app.db),
            ) {
                let _ = wv.set_visible(is_active);
                tab_urls.insert(tab.id.clone(), tab.url.clone());
                tab_webviews.insert(tab.id.clone(), wv);
                let _ = app.create_tab(tab.url.clone(), tab.title.clone(), "default".into());
            }
        }
        if let Ok(json_str) = serde_json::to_string(&saved_tabs) {
            let script = format!("if (window.__nexora_restore_tabs) window.__nexora_restore_tabs({});", json_str);
            let _ = toolbar_webview.evaluate_script(&script);
        }
    } else {
        let initial_wv = create_tab_webview(
            &window,
            &active_tab_id,
            "nexora://home",
            content_bounds,
            proxy.clone(),
            Arc::clone(&app.security),
            Arc::clone(&app.db),
        )?;
        let _ = initial_wv.set_visible(true);
        tab_urls.insert(active_tab_id.clone(), "nexora://home".to_string());
        tab_webviews.insert(active_tab_id.clone(), initial_wv);
        let _ = app.create_tab("nexora://home".into(), "Nexora Home".into(), "default".into());
    }
    bring_toolbar_to_top();

    let _ = toolbar_webview.set_bounds(Rect {
        position: PhysicalPosition::new(0, 0).into(),
        size: PhysicalSize::new(initial_width, initial_toolbar_h).into(),
    });

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::UserEvent(action) => {
                match action {
                    BrowserAction::Navigate(url) => {
                        tab_urls.insert(active_tab_id.clone(), url.clone());

                        if let Some(active_wv) = tab_webviews.get(&active_tab_id) {
                            if url.starts_with("nexora://") || url.is_empty() {
                                match url.as_str() {
                                    "nexora://history" => {
                                        let _ = active_wv.load_html(NexoraUiAssets::get_history_html());
                                    }
                                    "nexora://bookmarks" => {
                                        let _ = active_wv.load_html(NexoraUiAssets::get_bookmarks_html());
                                    }
                                    "nexora://settings" => {
                                        let _ = active_wv.load_html(NexoraUiAssets::get_settings_html());
                                    }
                                    "nexora://downloads" => {
                                        let _ = active_wv.load_html(NexoraUiAssets::get_downloads_html());
                                    }
                                    _ => {
                                        let _ = active_wv.load_html(NexoraUiAssets::get_standalone_home_html());
                                    }
                                }
                                let update_script = format!("if (window.__nexora_update_url) window.__nexora_update_url({:?});", url);
                                let _ = toolbar_webview.evaluate_script(&update_script);
                            } else if let Some(cleaned) = app.security.inspect_and_filter_url(&url) {
                                let update_script = format!("if (window.__nexora_update_url) window.__nexora_update_url({:?});", cleaned);
                                let _ = toolbar_webview.evaluate_script(&update_script);
                                let _ = toolbar_webview.evaluate_script("if (window.__nexora_page_loading) window.__nexora_page_loading(true);");
                                let _ = active_wv.load_url(&cleaned);
                                let _ = app.db.record_visit(&cleaned, &url, "default");
                            }
                        }
                    }
                    BrowserAction::GoBack => {
                        if let Some(active_wv) = tab_webviews.get(&active_tab_id) {
                            let _ = active_wv.evaluate_script("window.history.back()");
                        }
                    }
                    BrowserAction::GoForward => {
                        if let Some(active_wv) = tab_webviews.get(&active_tab_id) {
                            let _ = active_wv.evaluate_script("window.history.forward()");
                        }
                    }
                    BrowserAction::Reload => {
                        if let Some(active_wv) = tab_webviews.get(&active_tab_id) {
                            let _ = active_wv.evaluate_script("window.location.reload()");
                        }
                    }
                    BrowserAction::NewTab(id) => {
                        if let Some(prev_wv) = tab_webviews.get(&active_tab_id) {
                            let _ = prev_wv.set_visible(false);
                        }

                        match create_tab_webview(
                            &window,
                            &id,
                            "nexora://home",
                            content_bounds,
                            proxy.clone(),
                            Arc::clone(&app.security),
                            Arc::clone(&app.db),
                        ) {
                            Ok(new_wv) => {
                                let _ = new_wv.set_visible(true);
                                tab_urls.insert(id.clone(), "nexora://home".to_string());
                                tab_webviews.insert(id.clone(), new_wv);
                                active_tab_id = id.clone();
                                let _ = app.create_tab("nexora://home".into(), "Nexora Home".into(), "default".into());
                                bring_toolbar_to_top();

                                let _ = toolbar_webview.evaluate_script("if (window.__nexora_update_url) window.__nexora_update_url('nexora://home');");
                                let _ = toolbar_webview.evaluate_script("if (window.__nexora_set_bookmarked) window.__nexora_set_bookmarked(false);");
                                let _ = toolbar_webview.evaluate_script("if (window.__nexora_set_reader_mode) window.__nexora_set_reader_mode(false);");
                            }
                            Err(e) => {
                                eprintln!("Failed to create tab webview: {}", e);
                            }
                        }
                    }
                    BrowserAction::SwitchTab(id) => {
                        if id != active_tab_id {
                            if let Some(prev_wv) = tab_webviews.get(&active_tab_id) {
                                let _ = prev_wv.set_visible(false);
                            }

                            if let Some(curr_wv) = tab_webviews.get(&id) {
                                let _ = curr_wv.set_visible(true);
                                let _ = curr_wv.set_bounds(content_bounds);

                                let logical_url = tab_urls.get(&id).cloned().unwrap_or_else(|| {
                                    curr_wv.url().unwrap_or_default()
                                });

                                let is_bm = app.db.is_bookmarked(&logical_url);
                                let bm_script = format!("if (window.__nexora_set_bookmarked) window.__nexora_set_bookmarked({});", is_bm);
                                let _ = toolbar_webview.evaluate_script(&bm_script);
                            }

                            active_tab_id = id.clone();
                            app.switch_tab(&id);
                            bring_toolbar_to_top();

                            if let Some(curr_wv) = tab_webviews.get(&id) {
                                let nav_script = r#"
                                    (function() {
                                        try {
                                            let canBack = window.history.length > 1;
                                            if (window.ipc) {
                                                window.ipc.postMessage(JSON.stringify({
                                                    type: 'nav_state_changed',
                                                    can_go_back: canBack,
                                                    can_go_forward: false
                                                }));
                                            }
                                        } catch(e) {}
                                    })();
                                "#;
                                let _ = curr_wv.evaluate_script(nav_script);
                            }
                        }
                    }
                    BrowserAction::CloseTab(id) => {
                        if let Some(wv) = tab_webviews.get(&id) {
                            let url = tab_urls.get(&id).cloned().unwrap_or_else(|| wv.url().unwrap_or_default());
                            if !url.is_empty() && !url.starts_with("nexora://") {
                                closed_tabs.push((url, "Closed Tab".to_string()));
                            }
                        }
                        tab_webviews.remove(&id);
                        tab_urls.remove(&id);
                        app.close_tab(&id);

                        if active_tab_id == id {
                            if let Some((next_id, next_wv)) = tab_webviews.iter().next() {
                                let _ = next_wv.set_visible(true);
                                let _ = next_wv.set_bounds(content_bounds);
                                active_tab_id = next_id.clone();
                            }
                        }
                    }
                    BrowserAction::ReopenClosedTab => {
                        if let Some((url, title)) = closed_tabs.pop() {
                            let new_id = format!("tab-{}", chrono::Utc::now().timestamp_millis());
                            if let Some(prev_wv) = tab_webviews.get(&active_tab_id) {
                                let _ = prev_wv.set_visible(false);
                            }

                            match create_tab_webview(
                                &window,
                                &new_id,
                                &url,
                                content_bounds,
                                proxy.clone(),
                                Arc::clone(&app.security),
                                Arc::clone(&app.db),
                            ) {
                                Ok(new_wv) => {
                                    let _ = new_wv.set_visible(true);
                                    tab_urls.insert(new_id.clone(), url.clone());
                                    tab_webviews.insert(new_id.clone(), new_wv);
                                    active_tab_id = new_id.clone();
                                    let _ = app.create_tab(url.clone(), title.clone(), "default".into());
                                    bring_toolbar_to_top();

                                    let restore_script = format!(
                                        "if (window.__nexora_restore_tab) window.__nexora_restore_tab({:?}, {:?}, {:?});",
                                        new_id, url, title
                                    );
                                    let _ = toolbar_webview.evaluate_script(&restore_script);
                                }
                                Err(e) => {
                                    eprintln!("Failed to restore tab webview: {}", e);
                                }
                            }
                        }
                    }
                    BrowserAction::TabNavigated { tab_id, url } => {
                        let effective_url = if url.is_empty() || url == "about:blank" || url.starts_with("data:") {
                            tab_urls.get(&tab_id).cloned().unwrap_or_else(|| "nexora://home".to_string())
                        } else {
                            tab_urls.insert(tab_id.clone(), url.clone());
                            url
                        };

                        if tab_id == active_tab_id {
                            let script = format!("if (window.__nexora_update_url) window.__nexora_update_url({:?});", effective_url);
                            let _ = toolbar_webview.evaluate_script(&script);

                            let is_bm = app.db.is_bookmarked(&effective_url);
                            let bm_script = format!("if (window.__nexora_set_bookmarked) window.__nexora_set_bookmarked({});", is_bm);
                            let _ = toolbar_webview.evaluate_script(&bm_script);

                            let _ = toolbar_webview.evaluate_script("if (window.__nexora_page_loading) window.__nexora_page_loading(true);");
                        }
                    }
                    BrowserAction::TabLoadFinished { tab_id, url } => {
                        let effective_url = if url.is_empty() || url == "about:blank" || url.starts_with("data:") {
                            tab_urls.get(&tab_id).cloned().unwrap_or_else(|| "nexora://home".to_string())
                        } else {
                            tab_urls.insert(tab_id.clone(), url.clone());
                            url
                        };

                        if tab_id == active_tab_id {
                            let script = format!("if (window.__nexora_update_url) window.__nexora_update_url({:?});", effective_url);
                            let _ = toolbar_webview.evaluate_script(&script);

                            let is_bm = app.db.is_bookmarked(&effective_url);
                            let bm_script = format!("if (window.__nexora_set_bookmarked) window.__nexora_set_bookmarked({});", is_bm);
                            let _ = toolbar_webview.evaluate_script(&bm_script);

                            let _ = toolbar_webview.evaluate_script("if (window.__nexora_page_loading) window.__nexora_page_loading(false);");
                        }

                        if let Some(wv) = tab_webviews.get(&tab_id) {
                            let nav_script = r#"
                                (function() {
                                    try {
                                        let canBack = window.history.length > 1;
                                        if (window.ipc) {
                                            window.ipc.postMessage(JSON.stringify({
                                                type: 'nav_state_changed',
                                                can_go_back: canBack,
                                                can_go_forward: false
                                            }));
                                        }
                                    } catch(e) {}
                                })();
                            "#;
                            let _ = wv.evaluate_script(nav_script);
                        }

                        if effective_url.starts_with("http://") || effective_url.starts_with("https://") {
                            if let Some(wv) = tab_webviews.get(&tab_id) {
                                let favicon_script = r#"
                                    (function() {
                                        try {
                                            let link = document.querySelector('link[rel~="icon"]') || 
                                                       document.querySelector('link[rel="shortcut icon"]') || 
                                                       document.querySelector('link[rel*="icon"]') || 
                                                       document.querySelector('link[rel="apple-touch-icon"]');
                                            let iconUrl = link && link.href ? link.href : (window.location.origin + '/favicon.ico');
                                            if (window.ipc) {
                                                window.ipc.postMessage(JSON.stringify({
                                                    type: 'page_favicon',
                                                    favicon: iconUrl
                                                }));
                                            }
                                        } catch(e) {}
                                    })();
                                "#;
                                let _ = wv.evaluate_script(favicon_script);

                                if effective_url.contains("google.com") {
                                    let google_account_script = r#"
                                        (function() {
                                            try {
                                                let els = document.querySelectorAll('a[aria-label*="@"], div[aria-label*="@"], a[href*="SignOutOptions"], a[aria-label*="Google Account:"], a[aria-label*="Akun Google:"]');
                                                els.forEach(el => {
                                                    let aria = el.getAttribute('aria-label') || el.innerText || '';
                                                    let match = aria.match(/([a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,})/);
                                                    if (match && window.ipc) {
                                                        let email = match[1];
                                                        let name = aria.replace(/Google Account:|Akun Google:/gi, '').replace(email, '').replace(/[()]/g, '').trim();
                                                        window.ipc.postMessage(JSON.stringify({
                                                            type: 'google_account_detected',
                                                            email: email,
                                                            name: name.length > 0 ? name : email.split('@')[0]
                                                        }));
                                                    }
                                                });
                                            } catch(e) {}
                                        })();
                                    "#;
                                    let _ = wv.evaluate_script(google_account_script);
                                }
                            }
                        }
                    }
                    BrowserAction::TabTitleChanged { tab_id, title } => {
                        let script = format!(
                            "if (window.__nexora_update_tab_title) window.__nexora_update_tab_title({:?}, {:?});",
                            tab_id, title
                        );
                        let _ = toolbar_webview.evaluate_script(&script);

                        if let Some(url) = tab_urls.get(&tab_id) {
                            if !url.is_empty() 
                                && !url.starts_with("nexora://") 
                                && !url.starts_with("data:") 
                                && !url.starts_with("about:") 
                            {
                                let _ = app.db.record_visit(url, &title, "default");
                            }
                        }
                    }
                    BrowserAction::ContentIpc { tab_id, body } => {
                        if let Ok(v) = serde_json::from_str::<Value>(&body) {
                            if let Some(msg_type) = v.get("type").and_then(|t| t.as_str()) {
                                match msg_type {
                                    "google_account_detected" => {
                                        if let Some(email) = v.get("email").and_then(|e| e.as_str()) {
                                            if !email.is_empty() {
                                                let email_clean = email.trim().to_string();
                                                let fallback = email_clean.split('@').next().unwrap_or("User");
                                                let name = v.get("name").and_then(|n| n.as_str()).filter(|s| !s.is_empty()).unwrap_or(fallback);
                                                if let Ok(profiles) = app.db.get_profiles() {
                                                    if let Some(existing) = profiles.iter().find(|p| p.email.eq_ignore_ascii_case(&email_clean)) {
                                                        if existing.name != name {
                                                            let _ = app.db.update_profile(&existing.id, name, &email_clean, "Google", &existing.color);
                                                        }
                                                    } else if let Some(active) = profiles.iter().find(|p| p.is_active) {
                                                        if active.email.is_empty() {
                                                            let _ = app.db.update_profile(&active.id, name, &email_clean, "Google", &active.color);
                                                        } else {
                                                            let colors = ["#4285f4", "#34a853", "#ea4335", "#fbbc05", "#9c27b0", "#009688", "#ff5722"];
                                                            let next_color = colors[profiles.len() % colors.len()];
                                                            if let Ok(new_prof) = app.db.add_profile(name, &email_clean, "Google", next_color) {
                                                                let _ = app.db.switch_profile(&new_prof.id);
                                                            }
                                                        }
                                                    } else {
                                                        let _ = app.db.add_profile(name, &email_clean, "Google", "#4285f4");
                                                    }
                                                    let _ = proxy.send_event(BrowserAction::GetProfiles);
                                                }
                                            }
                                        }
                                    }
                                    "nav_state_changed" => {
                                        let can_back = v.get("can_go_back").and_then(|b| b.as_bool()).unwrap_or(false);
                                        let can_forward = v.get("can_go_forward").and_then(|f| f.as_bool()).unwrap_or(false);
                                        if tab_id == active_tab_id {
                                            let script = format!("if (window.__nexora_update_nav_state) window.__nexora_update_nav_state({}, {});", can_back, can_forward);
                                            let _ = toolbar_webview.evaluate_script(&script);
                                        }
                                    }
                                    "get_history" => {
                                        let query = v.get("query").and_then(|q| q.as_str()).unwrap_or("");
                                        let records = app.db.query_history(query, 100).unwrap_or_default();
                                        if let Ok(json) = serde_json::to_string(&records) {
                                            if let Some(wv) = tab_webviews.get(&tab_id) {
                                                let script = format!("if (window.__nexora_render_history) window.__nexora_render_history({});", json);
                                                let _ = wv.evaluate_script(&script);
                                            }
                                        }
                                    }
                                    "delete_history" => {
                                        if let Some(id) = v.get("id").and_then(|i| i.as_i64()) {
                                            let _ = app.db.delete_history_item(id);
                                        }
                                    }
                                    "clear_history" => {
                                        let _ = app.db.clear_history();
                                    }
                                    "clear_browsing_data" => {
                                        let hours = v.get("hours").and_then(|h| h.as_i64()).unwrap_or(0);
                                        let clear_hist = v.get("clear_history").and_then(|b| b.as_bool()).unwrap_or(true);
                                        let clear_dl = v.get("clear_downloads").and_then(|b| b.as_bool()).unwrap_or(true);
                                        if clear_hist {
                                            if hours <= 0 {
                                                let _ = app.db.clear_history();
                                            } else {
                                                let cutoff = chrono::Utc::now().timestamp() - (hours * 3600);
                                                let _ = app.db.clear_history_range(cutoff);
                                            }
                                        }
                                        if clear_dl {
                                            let _ = app.db.clear_downloads();
                                        }
                                    }
                                    "get_bookmarks" => {
                                        let bookmarks = app.db.get_bookmarks().unwrap_or_default();
                                        if let Ok(json) = serde_json::to_string(&bookmarks) {
                                            if let Some(wv) = tab_webviews.get(&tab_id) {
                                                let script = format!("if (window.__nexora_render_bookmarks) window.__nexora_render_bookmarks({});", json);
                                                let _ = wv.evaluate_script(&script);
                                            }
                                        }
                                        if let Ok(bar_items) = app.db.get_bookmarks_bar() {
                                            if let Ok(json) = serde_json::to_string(&bar_items) {
                                                let tb_script = format!("if (window.__nexora_render_bookmarks_bar) window.__nexora_render_bookmarks_bar({});", json);
                                                let _ = toolbar_webview.evaluate_script(&tb_script);
                                            }
                                        }
                                    }
                                    "delete_bookmark" => {
                                        if let Some(url) = v.get("url").and_then(|u| u.as_str()) {
                                            let _ = app.db.remove_bookmark(url);
                                            if let Ok(bookmarks) = app.db.get_bookmarks_bar() {
                                                if let Ok(json) = serde_json::to_string(&bookmarks) {
                                                    let tb_script = format!("if (window.__nexora_render_bookmarks_bar) window.__nexora_render_bookmarks_bar({});", json);
                                                    let _ = toolbar_webview.evaluate_script(&tb_script);
                                                }
                                            }
                                        }
                                    }
                                    "update_bookmark_details" => {
                                        if let (Some(url), Some(title), Some(folder)) = (
                                            v.get("url").and_then(|u| u.as_str()),
                                            v.get("title").and_then(|t| t.as_str()),
                                            v.get("folder").and_then(|f| f.as_str()),
                                        ) {
                                            let _ = app.db.remove_bookmark(url);
                                            let _ = app.db.add_bookmark(url, title, folder);
                                            if let Ok(bookmarks) = app.db.get_bookmarks_bar() {
                                                if let Ok(json) = serde_json::to_string(&bookmarks) {
                                                    let tb_script = format!("if (window.__nexora_render_bookmarks_bar) window.__nexora_render_bookmarks_bar({});", json);
                                                    let _ = toolbar_webview.evaluate_script(&tb_script);
                                                }
                                            }
                                        }
                                    }
                                    "toggle_tab_mute" => {
                                        if let (Some(id), Some(muted)) = (v.get("id").and_then(|i| i.as_str()), v.get("muted").and_then(|m| m.as_bool())) {
                                            if let Some(wv) = tab_webviews.get(id) {
                                                let mute_script = format!(
                                                    "document.querySelectorAll('audio, video').forEach(el => {{ el.muted = {}; }});",
                                                    muted
                                                );
                                                let _ = wv.evaluate_script(&mute_script);
                                            }
                                        }
                                    }
                                    "get_settings" => {
                                        let mut map = serde_json::Map::new();
                                        map.insert("search_engine".into(), Value::String(app.db.get_setting("search_engine", "google")));
                                        map.insert("shield_enabled".into(), Value::String(app.db.get_setting("shield_enabled", "true")));
                                        map.insert("https_only".into(), Value::String(app.db.get_setting("https_only", "true")));
                                        map.insert("show_bookmarks_bar".into(), Value::String(app.db.get_setting("show_bookmarks_bar", "false")));
                                        let json = serde_json::to_string(&Value::Object(map)).unwrap_or_default();
                                        if let Some(wv) = tab_webviews.get(&tab_id) {
                                            let script = format!("if (window.__nexora_render_settings) window.__nexora_render_settings({});", json);
                                            let _ = wv.evaluate_script(&script);
                                        }
                                    }
                                    "set_setting" => {
                                        if let (Some(k), Some(val)) = (v.get("key").and_then(|k| k.as_str()), v.get("value").and_then(|val| val.as_str())) {
                                            let _ = app.db.set_setting(k, val);
                                            if k == "search_engine" {
                                                let update_script = format!(
                                                    "if (window.__nexora_update_settings) window.__nexora_update_settings({{ search_engine: {:?} }});",
                                                    val
                                                );
                                                let _ = toolbar_webview.evaluate_script(&update_script);
                                            }
                                        }
                                    }
                                    "get_downloads" => {
                                        let downloads = app.db.get_downloads().unwrap_or_default();
                                        if let Ok(json) = serde_json::to_string(&downloads) {
                                            if let Some(wv) = tab_webviews.get(&tab_id) {
                                                let script = format!("if (window.__nexora_render_downloads) window.__nexora_render_downloads({});", json);
                                                let _ = wv.evaluate_script(&script);
                                            }
                                        }
                                    }
                                    "clear_downloads" => {
                                        let _ = app.db.clear_downloads();
                                    }
                                    "open_downloads_folder" => {
                                        let default_dir = directories::UserDirs::new()
                                            .and_then(|u| u.download_dir().map(|p| p.to_path_buf()))
                                            .unwrap_or_else(std::env::temp_dir);
                                        let _ = std::process::Command::new("explorer.exe").arg(default_dir).spawn();
                                    }
                                    "open_file" => {
                                        if let Some(path) = v.get("path").and_then(|p| p.as_str()) {
                                            let _ = std::process::Command::new("explorer.exe").arg(path).spawn();
                                        }
                                    }
                                    "activate_reader_mode" => {
                                        if let (Some(html), Some(source_url)) = (
                                            v.get("html").and_then(|h| h.as_str()),
                                            v.get("url").and_then(|u| u.as_str()),
                                        ) {
                                            let reader_html = NanoReader::format_reader_view(html, source_url);
                                            if let Some(wv) = tab_webviews.get(&tab_id) {
                                                let _ = wv.load_html(&reader_html);
                                                let _ = toolbar_webview.evaluate_script("if (window.__nexora_set_reader_mode) window.__nexora_set_reader_mode(true);");
                                            }
                                        }
                                    }
                                    "navigate" => {
                                        if let Some(url) = v.get("url").and_then(|u| u.as_str()) {
                                            let _ = proxy.send_event(BrowserAction::Navigate(url.to_string()));
                                        }
                                    }
                                    "page_favicon" => {
                                        if let Some(favicon) = v.get("favicon").and_then(|f| f.as_str()) {
                                            let script = format!(
                                                "if (window.__nexora_update_tab_favicon) window.__nexora_update_tab_favicon({:?}, {:?});",
                                                tab_id, favicon
                                            );
                                            let _ = toolbar_webview.evaluate_script(&script);
                                        }
                                    }
                                    "shortcut" => {
                                        if let Some(action) = v.get("action").and_then(|a| a.as_str()) {
                                            match action {
                                                "new_tab" => {
                                                    let _ = toolbar_webview.evaluate_script("if (window.__nexora_create_new_tab) window.__nexora_create_new_tab();");
                                                }
                                                "new_window" => {
                                                    spawn_new_window(false);
                                                }
                                                "new_incognito" => {
                                                    spawn_new_window(true);
                                                }
                                                "close_tab" => {
                                                    let _ = toolbar_webview.evaluate_script("if (window.__nexora_close_active_tab) window.__nexora_close_active_tab();");
                                                }
                                                "reopen_closed_tab" => {
                                                    let _ = proxy.send_event(BrowserAction::ReopenClosedTab);
                                                }
                                                "focus_omnibox" => {
                                                    let _ = toolbar_webview.evaluate_script("if (window.__nexora_focus_omnibox) window.__nexora_focus_omnibox();");
                                                }
                                                "reload" => {
                                                    let _ = proxy.send_event(BrowserAction::Reload);
                                                }
                                                "history" => {
                                                    let _ = proxy.send_event(BrowserAction::Navigate("nexora://history".into()));
                                                }
                                                "downloads" => {
                                                    let _ = proxy.send_event(BrowserAction::Navigate("nexora://downloads".into()));
                                                }
                                                "find_in_page" => {
                                                    let _ = toolbar_webview.evaluate_script("if (window.__nexora_toggle_find_bar) window.__nexora_toggle_find_bar();");
                                                }
                                                "print" => {
                                                    let _ = proxy.send_event(BrowserAction::PrintPage);
                                                }
                                                "fullscreen" => {
                                                    let _ = proxy.send_event(BrowserAction::ToggleFullscreen);
                                                }
                                                "reader_mode" => {
                                                    let _ = proxy.send_event(BrowserAction::ToggleReaderMode);
                                                }
                                                "more_menu" => {
                                                    let _ = proxy.send_event(BrowserAction::OpenMoreMenu);
                                                }
                                                "zoom_in" => {
                                                    let _ = toolbar_webview.evaluate_script("if (window.__nexora_zoom_in) window.__nexora_zoom_in();");
                                                }
                                                "zoom_out" => {
                                                    let _ = toolbar_webview.evaluate_script("if (window.__nexora_zoom_out) window.__nexora_zoom_out();");
                                                }
                                                "zoom_reset" => {
                                                    let _ = toolbar_webview.evaluate_script("if (window.__nexora_zoom_reset) window.__nexora_zoom_reset();");
                                                }
                                                _ => {}
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    BrowserAction::ToggleBookmark { url, title } => {
                        let is_bm = app.db.is_bookmarked(&url);
                        if is_bm {
                            let _ = app.db.remove_bookmark(&url);
                            let _ = toolbar_webview.evaluate_script("if (window.__nexora_set_bookmarked) window.__nexora_set_bookmarked(false);");
                        } else {
                            let _ = app.db.add_bookmark(&url, &title, "Bookmarks Bar");
                            let _ = toolbar_webview.evaluate_script("if (window.__nexora_set_bookmarked) window.__nexora_set_bookmarked(true);");
                        }
                        if let Ok(bookmarks) = app.db.get_bookmarks_bar() {
                            if let Ok(json) = serde_json::to_string(&bookmarks) {
                                let script = format!("if (window.__nexora_render_bookmarks_bar) window.__nexora_render_bookmarks_bar({});", json);
                                let _ = toolbar_webview.evaluate_script(&script);
                            }
                        }
                    }
                    BrowserAction::OmniboxQuery(query) => {
                        let suggestions = app.db.get_omnibox_suggestions(&query).unwrap_or_default();
                        if let Ok(json) = serde_json::to_string(&suggestions) {
                            let script = format!("if (window.__nexora_render_omnibox_suggestions) window.__nexora_render_omnibox_suggestions({});", json);
                            let _ = toolbar_webview.evaluate_script(&script);
                        }
                    }
                    BrowserAction::ExpandOverlay(logical_h) => {
                        let sf = window.scale_factor();
                        let expanded_h = (logical_h * sf) as u32;
                        let inner = window.inner_size();
                        let _ = toolbar_webview.set_bounds(Rect {
                            position: PhysicalPosition::new(0, 0).into(),
                            size: PhysicalSize::new(inner.width, expanded_h).into(),
                        });
                        bring_toolbar_to_top();
                    }
                    BrowserAction::CollapseOverlay => {
                        let sf = window.scale_factor();
                        let logical_h = if is_bookmarks_bar_visible { TOOLBAR_LOGICAL_HEIGHT + 28.0 } else { TOOLBAR_LOGICAL_HEIGHT };
                        let normal_h = (logical_h * sf) as u32;
                        let inner = window.inner_size();
                        let _ = toolbar_webview.set_bounds(Rect {
                            position: PhysicalPosition::new(0, 0).into(),
                            size: PhysicalSize::new(inner.width, normal_h).into(),
                        });
                    }
                    BrowserAction::OpenMoreMenu => {
                        #[cfg(target_os = "windows")]
                        {
                            let hwnd = window.hwnd() as isize;
                            let sf = window.scale_factor();
                            if let Some(cmd) = show_native_more_menu(hwnd, sf) {
                                match cmd {
                                    1 => {
                                        let _ = toolbar_webview.evaluate_script("if (window.__nexora_create_new_tab) window.__nexora_create_new_tab();");
                                    }
                                    2 => {
                                        spawn_new_window(false);
                                    }
                                    12 => {
                                        spawn_new_window(true);
                                    }
                                    3 => {
                                        let _ = proxy.send_event(BrowserAction::Navigate("nexora://history".into()));
                                    }
                                    4 => {
                                        let _ = proxy.send_event(BrowserAction::Navigate("nexora://bookmarks".into()));
                                    }
                                    5 => {
                                        let _ = proxy.send_event(BrowserAction::Navigate("nexora://downloads".into()));
                                    }
                                    6 => {
                                        let _ = toolbar_webview.evaluate_script("if (window.__nexora_zoom_in) window.__nexora_zoom_in();");
                                    }
                                    7 => {
                                        let _ = toolbar_webview.evaluate_script("if (window.__nexora_zoom_out) window.__nexora_zoom_out();");
                                    }
                                    8 => {
                                        let _ = toolbar_webview.evaluate_script("if (window.__nexora_zoom_reset) window.__nexora_zoom_reset();");
                                    }
                                    13 => {
                                        let _ = toolbar_webview.evaluate_script("if (window.__nexora_toggle_find_bar) window.__nexora_toggle_find_bar();");
                                    }
                                    14 => {
                                        let _ = proxy.send_event(BrowserAction::ToggleReaderMode);
                                    }
                                    15 => {
                                        let _ = proxy.send_event(BrowserAction::PrintPage);
                                    }
                                    16 => {
                                        let _ = proxy.send_event(BrowserAction::ToggleFullscreen);
                                    }
                                    9 | 10 => {
                                        let _ = proxy.send_event(BrowserAction::Navigate("nexora://settings".into()));
                                    }
                                    11 => {
                                        let _ = proxy.send_event(BrowserAction::CloseWindow);
                                    }
                                    _ => {}
                                }
                            }
                        }

                        #[cfg(not(target_os = "windows"))]
                        {
                            let _ = toolbar_webview.evaluate_script("if (window.__nexora_toggle_more_menu) window.__nexora_toggle_more_menu();");
                        }
                    }
                    BrowserAction::OpenTabContextMenu { tab_id, screen_x, screen_y } => {
                        #[cfg(target_os = "windows")]
                        {
                            let hwnd = window.hwnd() as isize;
                            if let Some(cmd) = show_native_tab_context_menu(hwnd, screen_x, screen_y) {
                                match cmd {
                                    1 => {
                                        let script = format!("if (window.__nexora_tab_context_action) window.__nexora_tab_context_action('new_tab', {:?});", tab_id);
                                        let _ = toolbar_webview.evaluate_script(&script);
                                    }
                                    2 => {
                                        let script = format!("if (window.__nexora_tab_context_action) window.__nexora_tab_context_action('duplicate', {:?});", tab_id);
                                        let _ = toolbar_webview.evaluate_script(&script);
                                    }
                                    3 => {
                                        let script = format!("if (window.__nexora_tab_context_action) window.__nexora_tab_context_action('reload', {:?});", tab_id);
                                        let _ = toolbar_webview.evaluate_script(&script);
                                    }
                                    4 => {
                                        let script = format!("if (window.__nexora_tab_context_action) window.__nexora_tab_context_action('pin', {:?});", tab_id);
                                        let _ = toolbar_webview.evaluate_script(&script);
                                    }
                                    5 => {
                                        let script = format!("if (window.__nexora_tab_context_action) window.__nexora_tab_context_action('close', {:?});", tab_id);
                                        let _ = toolbar_webview.evaluate_script(&script);
                                    }
                                    6 => {
                                        let script = format!("if (window.__nexora_tab_context_action) window.__nexora_tab_context_action('close_others', {:?});", tab_id);
                                        let _ = toolbar_webview.evaluate_script(&script);
                                    }
                                    7 => {
                                        let script = format!("if (window.__nexora_tab_context_action) window.__nexora_tab_context_action('close_right', {:?});", tab_id);
                                        let _ = toolbar_webview.evaluate_script(&script);
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    BrowserAction::ToggleReaderMode => {
                        if let Some(active_wv) = tab_webviews.get(&active_tab_id) {
                            let script = r#"
                                (function() {
                                    if (window.__is_reader_mode) {
                                        window.location.reload();
                                    } else {
                                        let html = document.documentElement.outerHTML;
                                        if (window.ipc) {
                                            window.ipc.postMessage(JSON.stringify({
                                                type: 'activate_reader_mode',
                                                html: html,
                                                url: window.location.href
                                            }));
                                        }
                                    }
                                })();
                            "#;
                            let _ = active_wv.evaluate_script(script);
                        }
                    }
                    BrowserAction::FindInPage { query, forward } => {
                        if let Some(active_wv) = tab_webviews.get(&active_tab_id) {
                            let script = format!(
                                "window.find({:?}, false, {}, true, false, false, false);",
                                query, !forward
                            );
                            let _ = active_wv.evaluate_script(&script);
                        }
                    }
                    BrowserAction::StopFind => {
                        if let Some(active_wv) = tab_webviews.get(&active_tab_id) {
                            let _ = active_wv.evaluate_script("if (window.getSelection) window.getSelection().removeAllRanges();");
                        }
                    }
                    BrowserAction::PrintPage => {
                        if let Some(active_wv) = tab_webviews.get(&active_tab_id) {
                            let _ = active_wv.evaluate_script("window.print();");
                        }
                    }
                    BrowserAction::ToggleFullscreen => {
                        let is_fs = window.fullscreen().is_some();
                        if is_fs {
                            window.set_fullscreen(None);
                        } else {
                            window.set_fullscreen(Some(tao::window::Fullscreen::Borderless(None)));
                        }
                    }
                    BrowserAction::SetZoom(level) => {
                        if let Some(active_wv) = tab_webviews.get(&active_tab_id) {
                            let factor = (level as f64) / 100.0;
                            let script = format!("document.body.style.zoom = '{}';", factor);
                            let _ = active_wv.evaluate_script(&script);
                        }
                    }
                    BrowserAction::UpdateUrl(url) => {
                        let script = format!("if (window.__nexora_update_url) window.__nexora_update_url({:?});", url);
                        let _ = toolbar_webview.evaluate_script(&script);
                    }
                    BrowserAction::UpdateTitle(title) => {
                        let script = format!("if (window.__nexora_update_title) window.__nexora_update_title({:?});", title);
                        let _ = toolbar_webview.evaluate_script(&script);
                    }
                    BrowserAction::UpdateShield(count) => {
                        let script = format!("if (window.__nexora_update_shield) window.__nexora_update_shield({});", count);
                        let _ = toolbar_webview.evaluate_script(&script);
                    }
                    BrowserAction::DragWindow => {
                        let _ = window.drag_window();
                    }
                    BrowserAction::MinimizeWindow => {
                        window.set_minimized(true);
                    }
                    BrowserAction::ToggleMaximizeWindow => {
                        window.set_maximized(!window.is_maximized());
                    }
                    BrowserAction::CloseWindow => {
                        *control_flow = ControlFlow::Exit;
                    }
                    BrowserAction::GetProfiles => {
                        if let Ok(profiles) = app.db.get_profiles() {
                            if let Ok(json_str) = serde_json::to_string(&profiles) {
                                let script = format!("if (window.__nexora_update_profiles) window.__nexora_update_profiles({});", json_str);
                                let _ = toolbar_webview.evaluate_script(&script);
                            }
                        }
                        if !is_incognito {
                            if let Ok(groups) = app.db.get_tab_groups() {
                                if !groups.is_empty() {
                                    if let Ok(json_str) = serde_json::to_string(&groups) {
                                        let script = format!("if (window.__nexora_restore_tab_groups) window.__nexora_restore_tab_groups({});", json_str);
                                        let _ = toolbar_webview.evaluate_script(&script);
                                    }
                                }
                            }
                            let active_prof_id = app.db.get_active_profile().ok().flatten().map(|p| p.id).unwrap_or_else(|| "prof-default".to_string());
                            if let Ok(tabs) = app.db.get_open_tabs(Some(&active_prof_id)) {
                                if !tabs.is_empty() {
                                    if let Ok(json_str) = serde_json::to_string(&tabs) {
                                        let script = format!("if (window.__nexora_restore_tabs) window.__nexora_restore_tabs({});", json_str);
                                        let _ = toolbar_webview.evaluate_script(&script);
                                    }
                                }
                            }
                        }
                    }
                    BrowserAction::SwitchProfile(id) => {
                        let _ = app.db.switch_profile(&id);
                        if let Ok(profiles) = app.db.get_profiles() {
                            if let Some(target) = profiles.iter().find(|p| p.id == id) {
                                let target_url = if !target.email.is_empty() {
                                    format!("https://accounts.google.com/AccountChooser?Email={}&continue=https://myaccount.google.com", target.email)
                                } else {
                                    "nexora://home".to_string()
                                };
                                let _ = proxy.send_event(BrowserAction::Navigate(target_url));
                            }
                            if let Ok(json_str) = serde_json::to_string(&profiles) {
                                let script = format!("if (window.__nexora_update_profiles) window.__nexora_update_profiles({});", json_str);
                                let _ = toolbar_webview.evaluate_script(&script);
                            }
                        }
                    }
                    BrowserAction::AddProfile { name, email, badge, color } => {
                        if let Ok(new_prof) = app.db.add_profile(&name, &email, &badge, &color) {
                            let _ = app.db.switch_profile(&new_prof.id);
                            let _ = proxy.send_event(BrowserAction::Navigate("nexora://home".into()));
                        }
                        if let Ok(profiles) = app.db.get_profiles() {
                            if let Ok(json_str) = serde_json::to_string(&profiles) {
                                let script = format!("if (window.__nexora_update_profiles) window.__nexora_update_profiles({});", json_str);
                                let _ = toolbar_webview.evaluate_script(&script);
                            }
                        }
                    }
                    BrowserAction::CustomizeProfile { name, color } => {
                        if let Ok(Some(active_prof)) = app.db.get_active_profile() {
                            let target_color = if color.is_empty() { active_prof.color } else { color };
                            let _ = app.db.update_profile(&active_prof.id, &name, &active_prof.email, &active_prof.badge, &target_color);
                            if let Ok(profiles) = app.db.get_profiles() {
                                if let Ok(json_str) = serde_json::to_string(&profiles) {
                                    let script = format!("if (window.__nexora_update_profiles) window.__nexora_update_profiles({});", json_str);
                                    let _ = toolbar_webview.evaluate_script(&script);
                                }
                            }
                        }
                    }
                    BrowserAction::SignOut => {
                        let _ = app.db.sign_out();
                        if let Ok(profiles) = app.db.get_profiles() {
                            if let Ok(json_str) = serde_json::to_string(&profiles) {
                                let script = format!("if (window.__nexora_update_profiles) window.__nexora_update_profiles({});", json_str);
                                let _ = toolbar_webview.evaluate_script(&script);
                            }
                        }
                    }
                    BrowserAction::BookmarksBarToggled(visible) => {
                        is_bookmarks_bar_visible = visible;
                        let logical_h = if visible { TOOLBAR_LOGICAL_HEIGHT + 28.0 } else { TOOLBAR_LOGICAL_HEIGHT };
                        let sf = window.scale_factor();
                        let toolbar_h = (logical_h * sf) as u32;
                        let inner = window.inner_size();
                        let content_h = inner.height.saturating_sub(toolbar_h);

                        content_bounds = Rect {
                            position: PhysicalPosition::new(0, toolbar_h as i32).into(),
                            size: PhysicalSize::new(inner.width, content_h).into(),
                        };

                        let _ = toolbar_webview.set_bounds(Rect {
                            position: PhysicalPosition::new(0, 0).into(),
                            size: PhysicalSize::new(inner.width, toolbar_h).into(),
                        });

                        for (_, wv) in tab_webviews.iter() {
                            let _ = wv.set_bounds(content_bounds);
                        }

                        bring_toolbar_to_top();
                    }
                    BrowserAction::SyncTabs(tabs) => {
                        let active_prof_id = app.db.get_active_profile().ok().flatten().map(|p| p.id).unwrap_or_else(|| "prof-default".to_string());
                        let _ = app.db.save_open_tabs(&tabs, &active_prof_id);
                    }
                    BrowserAction::SaveTabGroup(group) => {
                        let _ = app.db.save_tab_group(&group);
                    }
                    BrowserAction::DeleteTabGroup(group_id) => {
                        let _ = app.db.delete_tab_group(&group_id);
                    }
                }
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(physical_size),
                ..
            } => {
                let sf = window.scale_factor();
                let logical_h = if is_bookmarks_bar_visible { TOOLBAR_LOGICAL_HEIGHT + 28.0 } else { TOOLBAR_LOGICAL_HEIGHT };
                let toolbar_h = (logical_h * sf) as u32;
                let new_width = physical_size.width;
                let new_height = physical_size.height;
                let content_h = new_height.saturating_sub(toolbar_h);

                content_bounds = Rect {
                    position: PhysicalPosition::new(0, toolbar_h as i32).into(),
                    size: PhysicalSize::new(new_width, content_h).into(),
                };

                let _ = toolbar_webview.set_bounds(Rect {
                    position: PhysicalPosition::new(0, 0).into(),
                    size: PhysicalSize::new(new_width, toolbar_h).into(),
                });

                for (_, wv) in tab_webviews.iter() {
                    let _ = wv.set_bounds(content_bounds);
                }

                bring_toolbar_to_top();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::MainEventsCleared => {
                app.freezer.sweep_and_hibernate(&app.tabs, Some(&active_tab_id));
            }
            _ => (),
        }
    });
}
