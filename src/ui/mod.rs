pub struct NexoraUiAssets;

impl NexoraUiAssets {
    pub const TOOLBAR_HTML: &'static str = include_str!("assets/toolbar.html");
    pub const TOOLBAR_CSS: &'static str = include_str!("assets/toolbar.css");
    pub const TOOLBAR_JS: &'static str = include_str!("assets/toolbar.js");
    pub const HOME_HTML: &'static str = include_str!("assets/home.html");
    pub const HISTORY_HTML: &'static str = include_str!("assets/history.html");
    pub const BOOKMARKS_HTML: &'static str = include_str!("assets/bookmarks.html");
    pub const SETTINGS_HTML: &'static str = include_str!("assets/settings.html");
    pub const DOWNLOADS_HTML: &'static str = include_str!("assets/downloads.html");

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

    pub fn get_history_html() -> &'static str {
        Self::HISTORY_HTML
    }

    pub fn get_bookmarks_html() -> &'static str {
        Self::BOOKMARKS_HTML
    }

    pub fn get_settings_html() -> &'static str {
        Self::SETTINGS_HTML
    }

    pub fn get_downloads_html() -> &'static str {
        Self::DOWNLOADS_HTML
    }
}
