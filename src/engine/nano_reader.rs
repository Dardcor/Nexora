use regex::Regex;

/// Nexora Nano: Pure Rust Distraction-Free Reader Engine.
/// Parses raw HTML, eliminates scripts, ads, and layout clutter,
/// and produces a lightweight, privacy-hardened reader document.
pub struct NanoReader;

impl NanoReader {
    /// Extracts readable title and content from raw HTML without executing any JavaScript.
    pub fn format_reader_view(raw_html: &str, source_url: &str) -> String {
        let title = Self::extract_title(raw_html);
        let cleaned_body = Self::clean_body(raw_html);

        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title} - Nexora Reader</title>
    <style>
        :root {{
            --bg-color: #0f172a;
            --text-color: #e2e8f0;
            --accent-color: #38bdf8;
            --meta-color: #94a3b8;
            --border-color: #334155;
        }}
        @media (prefers-color-scheme: light) {{
            :root {{
                --bg-color: #ffffff;
                --text-color: #1e293b;
                --accent-color: #0284c7;
                --meta-color: #64748b;
                --border-color: #e2e8f0;
            }}
        }}
        body {{
            background-color: var(--bg-color);
            color: var(--text-color);
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
            line-height: 1.8;
            font-size: 1.15rem;
            max-width: 760px;
            margin: 0 auto;
            padding: 2.5rem 1.5rem;
            text-rendering: optimizeLegibility;
        }}
        h1 {{
            font-size: 2.2rem;
            line-height: 1.3;
            margin-bottom: 0.5rem;
            letter-spacing: -0.02em;
        }}
        .meta {{
            color: var(--meta-color);
            font-size: 0.95rem;
            border-bottom: 1px solid var(--border-color);
            padding-bottom: 1rem;
            margin-bottom: 2rem;
        }}
        .meta a {{
            color: var(--accent-color);
            text-decoration: none;
        }}
        p {{
            margin: 1.25rem 0;
        }}
        img {{
            max-width: 100%;
            height: auto;
            border-radius: 8px;
            margin: 1.5rem 0;
        }}
        blockquote {{
            border-left: 4px solid var(--accent-color);
            margin: 1.5rem 0;
            padding-left: 1rem;
            color: var(--meta-color);
            font-style: italic;
        }}
    </style>
</head>
<body>
    <article>
        <h1>{title}</h1>
        <div class="meta">
            <span>Nexora Nano Reader Mode &bull; Source: <a href="{source_url}">{source_url}</a></span>
        </div>
        <div class="content">
            {cleaned_body}
        </div>
    </article>
</body>
</html>"#,
            title = title,
            source_url = source_url,
            cleaned_body = cleaned_body
        )
    }

    fn extract_title(html: &str) -> String {
        let title_re = Regex::new(r"(?i)<title[^>]*>(.*?)</title>").unwrap();
        if let Some(caps) = title_re.captures(html) {
            caps.get(1).map(|m| m.as_str().trim()).unwrap_or("Untitled").to_string()
        } else {
            "Untitled Document".to_string()
        }
    }

    fn clean_body(html: &str) -> String {
        // Remove script, style, nav, header, footer tags and their contents
        let removals = [
            r"(?is)<script[^>]*>.*?</script>",
            r"(?is)<style[^>]*>.*?</style>",
            r"(?is)<nav[^>]*>.*?</nav>",
            r"(?is)<header[^>]*>.*?</header>",
            r"(?is)<footer[^>]*>.*?</footer>",
            r"(?is)<aside[^>]*>.*?</aside>",
            r"(?is)<iframe[^>]*>.*?</iframe>",
        ];

        let mut cleaned = html.to_string();
        for pattern in removals {
            let re = Regex::new(pattern).unwrap();
            cleaned = re.replace_all(&cleaned, "").to_string();
        }

        // Try extracting <article> tag content first if present
        let article_re = Regex::new(r"(?is)<article[^>]*>(.*?)</article>").unwrap();
        if let Some(caps) = article_re.captures(&cleaned) {
            return caps.get(1).unwrap().as_str().trim().to_string();
        }

        // Otherwise extract all <p> paragraphs
        let p_re = Regex::new(r"(?is)<p[^>]*>(.*?)</p>").unwrap();
        let mut paragraphs = Vec::new();
        for caps in p_re.captures_iter(&cleaned) {
            let p_content = caps.get(1).unwrap().as_str().trim();
            if !p_content.is_empty() {
                paragraphs.push(format!("<p>{}</p>", p_content));
            }
        }

        if !paragraphs.is_empty() {
            paragraphs.join("\n")
        } else {
            "<p>No readable content extracted.</p>".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nano_reader_cleans_scripts_and_extracts_article() {
        let sample_html = r#"
            <html>
                <head><title>Rust Performance Guide</title><script>alert('track');</script></head>
                <body>
                    <header><nav>Home | About</nav></header>
                    <article>
                        <p>Rust provides zero-cost abstractions and memory safety without a garbage collector.</p>
                        <p>Nexora leverages Rust for maximum speed and minimal memory footprint.</p>
                    </article>
                    <footer>Copyright 2026</footer>
                </body>
            </html>
        "#;

        let reader_html = NanoReader::format_reader_view(sample_html, "https://example.com/rust");
        assert!(reader_html.contains("Rust Performance Guide"));
        assert!(reader_html.contains("zero-cost abstractions"));
        assert!(!reader_html.contains("<script>"));
        assert!(!reader_html.contains("alert('track')"));
        assert!(!reader_html.contains("Home | About"));
    }
}
