pub struct NexoraUiAssets;

impl NexoraUiAssets {
    pub const SHELL_HTML: &'static str = include_str!("assets/shell.html");
    pub const SHELL_CSS: &'static str = include_str!("assets/shell.css");
    pub const SHELL_JS: &'static str = include_str!("assets/shell.js");
    pub const NEWTAB_HTML: &'static str = include_str!("assets/newtab.html");

    /// Bundles the shell HTML with inlined CSS and JS for self-contained, zero-latency local rendering.
    pub fn get_inlined_shell_html() -> String {
        let mut html = Self::SHELL_HTML.to_string();
        let css_block = format!("<style>{}</style>", Self::SHELL_CSS);
        let js_block = format!("<script>{}</script>", Self::SHELL_JS);

        html = html.replace(r#"<link rel="stylesheet" href="shell.css">"#, &css_block);
        html = html.replace(r#"<script src="shell.js"></script>"#, &js_block);
        html
    }
}
