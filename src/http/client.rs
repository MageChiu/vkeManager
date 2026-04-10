use chrono::Utc;
use reqwest::Url;
use serde::{Serialize, de::DeserializeOwned};

use crate::{config::model::Settings, http::{error::ApiError, signer::VolcSigner}};

#[derive(Debug, Clone)]
pub struct VolcApiClient {
    endpoint: String,
    version: String,
    http_client: reqwest::Client,
    signer: VolcSigner,
}

impl VolcApiClient {
    pub fn new(settings: &Settings, service: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            endpoint: settings.endpoint.clone(),
            version: version.into(),
            http_client: reqwest::Client::new(),
            signer: VolcSigner::new(settings, service),
        }
    }

    pub async fn post_action<TReq, TResp>(&self, action: &str, payload: &TReq) -> Result<TResp, ApiError>
    where
        TReq: Serialize + ?Sized,
        TResp: DeserializeOwned,
    {
        let body = serde_json::to_string(payload)?;
        let mut url = Url::parse(&self.endpoint)?;
        url.query_pairs_mut()
            .append_pair("Action", action)
            .append_pair("Version", &self.version);

        let headers = self.signer.sign_json_request("POST", &url, &body, Utc::now());
        let response = self
            .http_client
            .post(url)
            .headers(headers)
            .body(body)
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;
        if !status.is_success() {
            return Err(ApiError::UnexpectedStatus { status, body });
        }

        Ok(serde_json::from_str(&body)?)
    }
}

#[cfg(test)]
mod tests {
    use httpmock::prelude::*;
    use serde_json::json;

    use super::*;
    use crate::config::model::Settings;

    #[derive(Debug, serde::Deserialize)]
    struct FakeResponse {
        result: String,
    }

    fn sample_settings(endpoint: String) -> Settings {
        Settings {
            access_key: String::from("ak-test"),
            secret_key: String::from("sk-test"),
            region: String::from("cn-beijing"),
            endpoint,
            cluster: Default::default(),
            permission: Default::default(),
        }
    }

    #[tokio::test]
    async fn post_action_sends_signed_request() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(POST)
                .path("/")
                .query_param("Action", "CreateCluster")
                .query_param("Version", "2022-05-12")
                .header_exists("authorization")
                .header_exists("x-date")
                .header_exists("x-content-sha256")
                .json_body(json!({"hello": "world"}));
            then.status(200)
                .header("content-type", "application/json")
                .body(r#"{"result":"ok"}"#);
        });

        let client = VolcApiClient::new(&sample_settings(server.base_url()), "vke", "2022-05-12");
        let response: FakeResponse = client
            .post_action("CreateCluster", &json!({"hello": "world"}))
            .await
            .expect("请求应成功");

        mock.assert();
        assert_eq!(response.result, "ok");
    }
}
