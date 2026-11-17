pub struct NexoraUiAssets;

impl NexoraUiAssets {
    pub const TOOLBAR_HTML: &'static str = include_str!("assets/toolbar.html");
    pub const TOOLBAR_CSS: &'static str = include_str!("assets/toolbar.css");
    pub const TOOLBAR_JS: &'static str = include_str!("assets/toolbar.js");
    pub const HOME_HTML: &'static str = include_str!("assets/home.html");

    /// Inlines CSS and JS into toolbar HTML for zero-latency local rendering.
    pub fn get_inlined_toolbar_html() -> String {
        let mut html = Self::TOOLBAR_HTML.to_string();
        let css_block = format!("<style>{}</style>", Self::TOOLBAR_CSS);
        let js_block = format!("<script>{}</script>", Self::TOOLBAR_JS);

        html = html.replace(r#"<link rel="stylesheet" href="toolbar.css">"#, &css_block);
        html = html.replace(r#"<script src="toolbar.js"></script>"#, &js_block);
        html
    }

    /// Returns the standalone local Nexora home page.
    pub fn get_standalone_home_html() -> &'static str {
        Self::HOME_HTML
    }
}
