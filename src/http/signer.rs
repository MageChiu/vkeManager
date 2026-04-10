use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use sha2::{Digest, Sha256};
use url::Url;

use crate::config::model::Settings;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone)]
pub struct VolcSigner {
    access_key: String,
    secret_key: String,
    region: String,
    service: String,
}

impl VolcSigner {
    pub fn new(settings: &Settings, service: impl Into<String>) -> Self {
        Self {
            access_key: settings.access_key.clone(),
            secret_key: settings.secret_key.clone(),
            region: settings.region.clone(),
            service: service.into(),
        }
    }

    pub fn sign_json_request(
        &self,
        method: &str,
        url: &Url,
        body: &str,
        now: DateTime<Utc>,
    ) -> HeaderMap {
        let content_sha256 = sha256_hex(body.as_bytes());
        let x_date = now.format("%Y%m%dT%H%M%SZ").to_string();
        let short_date = now.format("%Y%m%d").to_string();
        let canonical_query = canonical_query_string(url);
        let host = url.host_str().unwrap_or_default();
        let content_type = "application/json";
        let signed_headers = "content-type;host;x-content-sha256;x-date";

        let canonical_headers = format!(
            "content-type:{content_type}\nhost:{host}\nx-content-sha256:{content_sha256}\nx-date:{x_date}\n"
        );
        let canonical_request = format!(
            "{method}\n/\n{canonical_query}\n{canonical_headers}\n{signed_headers}\n{content_sha256}"
        );

        let credential_scope = format!("{short_date}/{}/{}/request", self.region, self.service);
        let string_to_sign = format!(
            "HMAC-SHA256\n{x_date}\n{credential_scope}\n{}",
            sha256_hex(canonical_request.as_bytes())
        );
        let signing_key = self.derive_signing_key(&short_date);
        let signature = hex::encode(hmac_sha256(&signing_key, &string_to_sign));
        let authorization = format!(
            "HMAC-SHA256 Credential={}/{}, SignedHeaders={}, Signature={}",
            self.access_key, credential_scope, signed_headers, signature
        );

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            HeaderName::from_static("x-content-sha256"),
            HeaderValue::from_str(&content_sha256).expect("x-content-sha256 必须是合法头部"),
        );
        headers.insert(
            HeaderName::from_static("x-date"),
            HeaderValue::from_str(&x_date).expect("x-date 必须是合法头部"),
        );
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&authorization).expect("authorization 必须是合法头部"),
        );
        headers
    }

    fn derive_signing_key(&self, short_date: &str) -> Vec<u8> {
        let date_key = hmac_sha256(self.secret_key.as_bytes(), short_date);
        let region_key = hmac_sha256(&date_key, &self.region);
        let service_key = hmac_sha256(&region_key, &self.service);
        hmac_sha256(&service_key, "request")
    }
}

fn hmac_sha256(key: &[u8], value: &str) -> Vec<u8> {
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC key 初始化失败");
    mac.update(value.as_bytes());
    mac.finalize().into_bytes().to_vec()
}

fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

fn canonical_query_string(url: &Url) -> String {
    let mut items = url
        .query_pairs()
        .map(|(key, value)| (key.into_owned(), value.into_owned()))
        .collect::<Vec<_>>();
    items.sort_by(|left, right| left.0.cmp(&right.0).then(left.1.cmp(&right.1)));
    items
        .into_iter()
        .map(|(key, value)| format!("{key}={value}"))
        .collect::<Vec<_>>()
        .join("&")
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;
    use crate::config::model::Settings;

    fn sample_settings() -> Settings {
        Settings {
            access_key: String::from("ak-test"),
            secret_key: String::from("sk-test"),
            region: String::from("cn-beijing"),
            endpoint: String::from("https://open.volcengineapi.com"),
            cluster: Default::default(),
            permission: Default::default(),
        }
    }

    #[test]
    fn sign_json_request_builds_required_headers() {
        let signer = VolcSigner::new(&sample_settings(), "vke");
        let url =
            Url::parse("https://open.volcengineapi.com/?Action=CreateCluster&Version=2022-05-12")
                .expect("测试 URL 应可解析");
        let now = Utc
            .with_ymd_and_hms(2026, 3, 20, 8, 0, 0)
            .single()
            .expect("测试时间应有效");

        let headers = signer.sign_json_request("POST", &url, "{}", now);

        assert!(headers.contains_key(AUTHORIZATION));
        assert!(headers.contains_key("x-date"));
        assert!(headers.contains_key("x-content-sha256"));
        assert_eq!(
            headers.get(CONTENT_TYPE).expect("必须包含 content-type"),
            "application/json"
        );
    }
}
