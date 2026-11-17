use std::sync::Arc;
use tao::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::WebViewBuilder;

use nexora::app::NexoraApp;
use nexora::security::AntiFingerprint;
use nexora::ui::NexoraUiAssets;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("==================================================");
    println!("  Nexora Browser - Ultra-Lightweight Rust Core   ");
    println!("  Repository: https://github.com/Dardcor/Nexora  ");
    println!("  Security: 6-Layer Multi-Defense & Local Vault   ");
    println!("==================================================");

    let app = Arc::new(NexoraApp::new()?);

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Nexora")
        .with_inner_size(tao::dpi::LogicalSize::new(1280.0, 800.0))
        .with_min_inner_size(tao::dpi::LogicalSize::new(480.0, 360.0))
        .build(&event_loop)?;

    let app_ipc = Arc::clone(&app);
    let stealth_script = AntiFingerprint::get_stealth_injection_script();
    let shell_html = NexoraUiAssets::get_inlined_shell_html();

    let _webview = WebViewBuilder::new()
        .with_initialization_script(stealth_script)
        .with_ipc_handler(move |req: wry::http::Request<String>| {
            let msg = req.body();
            if let Some(resp) = app_ipc.handle_ipc(msg) {
                println!("[Nexora IPC]: Handled: {}", resp);
            }
        })
        .with_html(shell_html)
        .build(&window)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                println!("[Nexora]: Browser window initialized successfully.");
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("[Nexora]: Saving local encrypted vault session...");
                let _ = app.vault.save_history();
                *control_flow = ControlFlow::Exit;
            }
            _ => (),
        }
    });
}
