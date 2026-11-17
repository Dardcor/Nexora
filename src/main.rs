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
        add_separator();
        add_item(3, "History\tCtrl+H");
        add_item(4, "Bookmarks\tCtrl+Shift+O");
        add_item(5, "Downloads\tCtrl+J");
        add_separator();
        add_item(6, "Zoom In (+10%)\tCtrl++");
        add_item(7, "Zoom Out (-10%)\tCtrl+-");
        add_item(8, "Reset Zoom (100%)\tCtrl+0");
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
use wry::{
    dpi::{PhysicalPosition, PhysicalSize},
    Rect, WebViewBuilder,
};

use nexora::app::NexoraApp;
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
    // Multi-tab specific events
    TabNavigated { tab_id: String, url: String },
    TabLoadFinished { tab_id: String, url: String },
    TabTitleChanged { tab_id: String, title: String },
    ContentIpc { tab_id: String, body: String },
    // Bookmarks and Omnibox
    ToggleBookmark { url: String, title: String },
    OmniboxQuery(String),
    ExpandOverlay(f64),
    CollapseOverlay,
    SetZoom(u32),
    OpenMoreMenu,
}

/// Helper function to create an isolated, dedicated child WebView for any browser tab.
fn create_tab_webview(
    window: &tao::window::Window,
    tab_id: &str,
    initial_url: &str,
    bounds: Rect,
    proxy: EventLoopProxy<BrowserAction>,
    security: Arc<nexora::security::NexoraSecurityManager>,
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

    let mut builder = WebViewBuilder::new()
        .with_bounds(bounds)
        .with_user_agent(DESKTOP_USER_AGENT)
        .with_initialization_script(stealth_script)
        .with_navigation_handler(move |url: String| -> bool {
            if url.starts_with("nexora://") {
                let _ = proxy_nav.send_event(BrowserAction::TabNavigated {
                    tab_id: tab_id_nav.clone(),
                    url,
                });
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
    enable_windows_dark_mode();

    let app = Arc::new(NexoraApp::new()?);

    let event_loop = EventLoopBuilder::<BrowserAction>::with_user_event().build();
    let proxy: EventLoopProxy<BrowserAction> = event_loop.create_proxy();

    let icon_bytes = include_bytes!("../assets/icon-32.rgba");
    let window_icon = tao::window::Icon::from_rgba(icon_bytes.to_vec(), 32, 32).ok();

    #[cfg(target_os = "windows")]
    let builder = WindowBuilder::new()
        .with_title("Nexora")
        .with_window_icon(window_icon)
        .with_decorations(false)
        .with_undecorated_shadow(true)
        .with_resizable(true)
        .with_inner_size(tao::dpi::LogicalSize::new(1280.0, 800.0))
        .with_min_inner_size(tao::dpi::LogicalSize::new(600.0, 400.0));

    #[cfg(not(target_os = "windows"))]
    let builder = WindowBuilder::new()
        .with_title("Nexora")
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

    let initial_toolbar_h = (TOOLBAR_LOGICAL_HEIGHT * scale_factor) as u32;
    let initial_content_h = initial_height.saturating_sub(initial_toolbar_h);

    let mut content_bounds = Rect {
        position: PhysicalPosition::new(0, initial_toolbar_h as i32).into(),
        size: PhysicalSize::new(initial_width, initial_content_h).into(),
    };

    // 1. Toolbar WebView (Tabs, Omnibox, Actions, Dropdowns)
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

    // 2. Multi-Tab Dedicated WebViews Map
    let mut tab_webviews: HashMap<String, wry::WebView> = HashMap::new();
    let mut active_tab_id: String = "tab-1".to_string();

    let initial_wv = create_tab_webview(
        &window,
        &active_tab_id,
        "nexora://home",
        content_bounds,
        proxy.clone(),
        Arc::clone(&app.security),
    )?;
    let _ = initial_wv.set_visible(true);
    tab_webviews.insert(active_tab_id.clone(), initial_wv);
    bring_toolbar_to_top();

    // Initial explicit bounds setup
    let _ = toolbar_webview.set_bounds(Rect {
        position: PhysicalPosition::new(0, 0).into(),
        size: PhysicalSize::new(initial_width, initial_toolbar_h).into(),
    });

    // Main Event Loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::UserEvent(action) => {
                match action {
                    BrowserAction::Navigate(url) => {
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
                            } else {
                                if let Some(cleaned) = app.security.inspect_and_filter_url(&url) {
                                    let _ = active_wv.load_url(&cleaned);
                                    app.vault.record_visit(cleaned.clone(), url.clone(), "default".to_string());
                                    let _ = app.db.record_visit(&cleaned, &url, "default");
                                }
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
                        // Hide current active tab webview
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
                        ) {
                            Ok(new_wv) => {
                                let _ = new_wv.set_visible(true);
                                tab_webviews.insert(id.clone(), new_wv);
                                active_tab_id = id.clone();
                                let _ = app.create_tab("nexora://home".into(), "Nexora Home".into(), "default".into());
                                bring_toolbar_to_top();

                                // Reset omnibox and bookmark star
                                let _ = toolbar_webview.evaluate_script("if (window.__nexora_update_url) window.__nexora_update_url('');");
                                let _ = toolbar_webview.evaluate_script("if (window.__nexora_set_bookmarked) window.__nexora_set_bookmarked(false);");
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

                                // Check bookmark status of current tab
                                if let Ok(url) = curr_wv.url() {
                                    let is_bm = app.db.is_bookmarked(&url);
                                    let bm_script = format!("if (window.__nexora_set_bookmarked) window.__nexora_set_bookmarked({});", is_bm);
                                    let _ = toolbar_webview.evaluate_script(&bm_script);
                                }
                            }

                            active_tab_id = id.clone();
                            app.switch_tab(&id);
                            bring_toolbar_to_top();
                        }
                    }
                    BrowserAction::CloseTab(id) => {
                        tab_webviews.remove(&id);
                        app.close_tab(&id);

                        if active_tab_id == id {
                            if let Some((next_id, next_wv)) = tab_webviews.iter().next() {
                                let _ = next_wv.set_visible(true);
                                let _ = next_wv.set_bounds(content_bounds);
                                active_tab_id = next_id.clone();
                            }
                        }
                    }
                    BrowserAction::TabNavigated { tab_id, url } | BrowserAction::TabLoadFinished { tab_id, url } => {
                        if tab_id == active_tab_id {
                            let script = format!("if (window.__nexora_update_url) window.__nexora_update_url({:?});", url);
                            let _ = toolbar_webview.evaluate_script(&script);

                            let is_bm = app.db.is_bookmarked(&url);
                            let bm_script = format!("if (window.__nexora_set_bookmarked) window.__nexora_set_bookmarked({});", is_bm);
                            let _ = toolbar_webview.evaluate_script(&bm_script);
                        }
                    }
                    BrowserAction::TabTitleChanged { tab_id, title } => {
                        let script = format!(
                            "if (window.__nexora_update_tab_title) window.__nexora_update_tab_title({:?}, {:?});",
                            tab_id, title
                        );
                        let _ = toolbar_webview.evaluate_script(&script);

                        if let Some(wv) = tab_webviews.get(&tab_id) {
                            if let Ok(url) = wv.url() {
                                if !url.is_empty() && !url.starts_with("nexora://") {
                                    let _ = app.db.record_visit(&url, &title, "default");
                                }
                            }
                        }
                    }
                    BrowserAction::ContentIpc { tab_id, body } => {
                        if let Ok(v) = serde_json::from_str::<Value>(&body) {
                            if let Some(msg_type) = v.get("type").and_then(|t| t.as_str()) {
                                match msg_type {
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
                                    "get_bookmarks" => {
                                        let bookmarks = app.db.get_bookmarks().unwrap_or_default();
                                        if let Ok(json) = serde_json::to_string(&bookmarks) {
                                            if let Some(wv) = tab_webviews.get(&tab_id) {
                                                let script = format!("if (window.__nexora_render_bookmarks) window.__nexora_render_bookmarks({});", json);
                                                let _ = wv.evaluate_script(&script);
                                            }
                                            let tb_script = format!("if (window.__nexora_render_bookmarks_bar) window.__nexora_render_bookmarks_bar({});", json);
                                            let _ = toolbar_webview.evaluate_script(&tb_script);
                                        }
                                    }
                                    "delete_bookmark" => {
                                        if let Some(url) = v.get("url").and_then(|u| u.as_str()) {
                                            let _ = app.db.remove_bookmark(url);
                                            if let Ok(bookmarks) = app.db.get_bookmarks() {
                                                if let Ok(json) = serde_json::to_string(&bookmarks) {
                                                    let tb_script = format!("if (window.__nexora_render_bookmarks_bar) window.__nexora_render_bookmarks_bar({});", json);
                                                    let _ = toolbar_webview.evaluate_script(&tb_script);
                                                }
                                            }
                                        }
                                    }
                                    "get_settings" => {
                                        let mut map = serde_json::Map::new();
                                        map.insert("search_engine".into(), Value::String(app.db.get_setting("search_engine", "google")));
                                        map.insert("shield_enabled".into(), Value::String(app.db.get_setting("shield_enabled", "true")));
                                        map.insert("https_only".into(), Value::String(app.db.get_setting("https_only", "true")));
                                        map.insert("show_bookmarks_bar".into(), Value::String(app.db.get_setting("show_bookmarks_bar", "true")));
                                        let json = serde_json::to_string(&Value::Object(map)).unwrap_or_default();
                                        if let Some(wv) = tab_webviews.get(&tab_id) {
                                            let script = format!("if (window.__nexora_render_settings) window.__nexora_render_settings({});", json);
                                            let _ = wv.evaluate_script(&script);
                                        }
                                    }
                                    "set_setting" => {
                                        if let (Some(k), Some(val)) = (v.get("key").and_then(|k| k.as_str()), v.get("value").and_then(|val| val.as_str())) {
                                            let _ = app.db.set_setting(k, val);
                                        }
                                    }
                                    "navigate" => {
                                        if let Some(url) = v.get("url").and_then(|u| u.as_str()) {
                                            let _ = proxy.send_event(BrowserAction::Navigate(url.to_string()));
                                            let script = format!("if (window.__nexora_update_url) window.__nexora_update_url({:?});", url);
                                            let _ = toolbar_webview.evaluate_script(&script);
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
                        if let Ok(bookmarks) = app.db.get_bookmarks() {
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
                        let normal_h = (TOOLBAR_LOGICAL_HEIGHT * sf) as u32;
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
                                    1 | 2 => {
                                        let _ = toolbar_webview.evaluate_script("if (window.__nexora_create_new_tab) window.__nexora_create_new_tab();");
                                    }
                                    3 => {
                                        let _ = proxy.send_event(BrowserAction::Navigate("nexora://history".into()));
                                        let _ = toolbar_webview.evaluate_script("if (window.__nexora_update_url) window.__nexora_update_url('nexora://history');");
                                    }
                                    4 => {
                                        let _ = proxy.send_event(BrowserAction::Navigate("nexora://bookmarks".into()));
                                        let _ = toolbar_webview.evaluate_script("if (window.__nexora_update_url) window.__nexora_update_url('nexora://bookmarks');");
                                    }
                                    5 => {
                                        let _ = proxy.send_event(BrowserAction::Navigate("nexora://downloads".into()));
                                        let _ = toolbar_webview.evaluate_script("if (window.__nexora_update_url) window.__nexora_update_url('nexora://downloads');");
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
                                    9 | 10 => {
                                        let _ = proxy.send_event(BrowserAction::Navigate("nexora://settings".into()));
                                        let _ = toolbar_webview.evaluate_script("if (window.__nexora_update_url) window.__nexora_update_url('nexora://settings');");
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
                        let _ = app.vault.save_history();
                        *control_flow = ControlFlow::Exit;
                    }
                }
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(physical_size),
                ..
            } => {
                let sf = window.scale_factor();
                let toolbar_h = (TOOLBAR_LOGICAL_HEIGHT * sf) as u32;
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
                let _ = app.vault.save_history();
                *control_flow = ControlFlow::Exit;
            }
            _ => (),
        }
    });
}
