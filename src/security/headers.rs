use url::Url;

pub struct PrivacyHeaders;

impl PrivacyHeaders {
    pub fn strip_tracking_params(raw_url: &str) -> String {
        let Ok(mut parsed) = Url::parse(raw_url) else {
            return raw_url.to_string();
        };

        let tracking_params = [
            "utm_source",
            "utm_medium",
            "utm_campaign",
            "utm_term",
            "utm_content",
            "fbclid",
            "gclid",
            "dclid",
            "msclkid",
            "mc_eid",
            "zanpid",
            "igshid",
            "si",
            "_hsenc",
            "_hsmi",
        ];

        let filtered_pairs: Vec<(String, String)> = parsed
            .query_pairs()
            .filter(|(key, _)| !tracking_params.contains(&key.as_ref()))
            .map(|(k, v)| (k.into_owned(), v.into_owned()))
            .collect();

        if filtered_pairs.is_empty() {
            parsed.set_query(None);
        } else {
            parsed.query_pairs_mut().clear().extend_pairs(filtered_pairs);
        }

        parsed.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_tracking_parameters() {
        let dirty_url = "https://example.com/product?id=42&utm_source=newsletter&utm_medium=email&fbclid=123456789";
        let cleaned = PrivacyHeaders::strip_tracking_params(dirty_url);
        assert_eq!(cleaned, "https://example.com/product?id=42");

        let clean_url = "https://github.com/Dardcor/Nexora";
        assert_eq!(PrivacyHeaders::strip_tracking_params(clean_url), clean_url);
    }
}
