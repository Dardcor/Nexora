use serde_json::Value;
use std::sync::Arc;
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder, EventLoopProxy},
    window::WindowBuilder,
};
use wry::{
    dpi::{LogicalPosition, LogicalSize},
    Rect, WebViewBuilder,
};

use nexora::app::NexoraApp;
use nexora::security::AntiFingerprint;
use nexora::ui::NexoraUiAssets;

const TOOLBAR_HEIGHT: u32 = 76;

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
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Arc::new(NexoraApp::new()?);

    let event_loop = EventLoopBuilder::<BrowserAction>::with_user_event().build();
    let proxy: EventLoopProxy<BrowserAction> = event_loop.create_proxy();

    let window = WindowBuilder::new()
        .with_title("Nexora")
        .with_inner_size(tao::dpi::LogicalSize::new(1280.0, 800.0))
        .with_min_inner_size(tao::dpi::LogicalSize::new(600.0, 400.0))
        .build(&event_loop)?;

    let initial_size = window.inner_size().to_logical::<u32>(window.scale_factor());
    let width = initial_size.width;
    let height = initial_size.height;

    // 1. Toolbar WebView (Tabs, Omnibox, Shield Indicator)
    let proxy_toolbar = proxy.clone();
    let toolbar_html = NexoraUiAssets::get_inlined_toolbar_html();

    let toolbar_webview = WebViewBuilder::new()
        .with_bounds(Rect {
            position: LogicalPosition::new(0, 0).into(),
            size: LogicalSize::new(width, TOOLBAR_HEIGHT).into(),
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
                            let id = v.get("id").and_then(|i| i.as_str()).unwrap_or("default");
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
                        _ => {}
                    }
                }
            }
        })
        .build(&window)?;

    // 2. Content WebView (Full Native Web Rendering - No iframe restrictions)
    let proxy_load = proxy.clone();
    let proxy_title = proxy.clone();
    let proxy_nav = proxy.clone();
    let security = Arc::clone(&app.security);
    let stealth_script = AntiFingerprint::get_stealth_injection_script();
    let home_html = NexoraUiAssets::get_standalone_home_html();

    let content_height = height.saturating_sub(TOOLBAR_HEIGHT);

    let content_webview = WebViewBuilder::new()
        .with_bounds(Rect {
            position: LogicalPosition::new(0, TOOLBAR_HEIGHT).into(),
            size: LogicalSize::new(width, content_height).into(),
        })
        .with_initialization_script(stealth_script)
        .with_html(home_html)
        .with_navigation_handler(move |url: String| -> bool {
            let (should_allow, cleaned_url) = if security.shield.should_block(&url) {
                let (count, _) = security.shield.get_statistics();
                let _ = proxy_nav.send_event(BrowserAction::UpdateShield(count));
                (false, url)
            } else {
                let cleaned = nexora::security::PrivacyHeaders::strip_tracking_params(&url);
                (true, cleaned)
            };

            let _ = proxy_nav.send_event(BrowserAction::UpdateUrl(cleaned_url));
            should_allow
        })
        .with_on_page_load_handler(move |event, url: String| {
            if matches!(event, wry::PageLoadEvent::Finished) {
                let _ = proxy_load.send_event(BrowserAction::UpdateUrl(url));
            }
        })
        .with_document_title_changed_handler(move |title: String| {
            let _ = proxy_title.send_event(BrowserAction::UpdateTitle(title));
        })
        .build(&window)?;

    // Main Event Loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::UserEvent(action) => {
                match action {
                    BrowserAction::Navigate(url) => {
                        if url == "nexora://home" || url.is_empty() {
                            let _ = content_webview.load_html(NexoraUiAssets::get_standalone_home_html());
                        } else {
                            // Strip tracking params and inspect with security shield
                            if let Some(cleaned) = app.security.inspect_and_filter_url(&url) {
                                let _ = content_webview.load_url(&cleaned);
                                app.vault.record_visit(cleaned, url.clone(), "default".to_string());
                            }
                        }
                    }
                    BrowserAction::GoBack => {
                        let _ = content_webview.evaluate_script("window.history.back()");
                    }
                    BrowserAction::GoForward => {
                        let _ = content_webview.evaluate_script("window.history.forward()");
                    }
                    BrowserAction::Reload => {
                        let _ = content_webview.evaluate_script("window.location.reload()");
                    }
                    BrowserAction::NewTab(_) => {
                        let _ = content_webview.load_html(NexoraUiAssets::get_standalone_home_html());
                    }
                    BrowserAction::SwitchTab(_) => {
                        // In future multi-tab: switch webview buffer
                    }
                    BrowserAction::CloseTab(_) => {
                        // Handled in toolbar state
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
                }
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(physical_size),
                ..
            } => {
                let size = physical_size.to_logical::<u32>(window.scale_factor());
                let new_width = size.width;
                let new_height = size.height;
                let content_h = new_height.saturating_sub(TOOLBAR_HEIGHT);

                let _ = toolbar_webview.set_bounds(Rect {
                    position: LogicalPosition::new(0, 0).into(),
                    size: LogicalSize::new(new_width, TOOLBAR_HEIGHT).into(),
                });
                let _ = content_webview.set_bounds(Rect {
                    position: LogicalPosition::new(0, TOOLBAR_HEIGHT).into(),
                    size: LogicalSize::new(new_width, content_h).into(),
                });
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
