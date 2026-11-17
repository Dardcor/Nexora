pub mod nano_reader;
pub mod tab;
pub mod tab_freezer;

pub use nano_reader::NanoReader;
pub use tab::BrowserTab;
pub use tab_freezer::TabFreezer;

/// Abstraction for the Web Engine Backend.
/// Enables future transition to pure-Rust engines (e.g., Servo/Ladybird) without changing UI or Security layers.
pub trait WebEngineBackend: Send + Sync {
    fn navigate(&mut self, url: &str) -> Result<(), Box<dyn std::error::Error>>;
    fn go_back(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn go_forward(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn reload(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn execute_script(&mut self, script: &str) -> Result<(), Box<dyn std::error::Error>>;
}
