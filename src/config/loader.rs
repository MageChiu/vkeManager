use std::{env, fs, path::Path};

use anyhow::{bail, Context, Result};

use crate::config::model::Settings;

pub fn load_settings(path: impl AsRef<Path>) -> Result<Settings> {
    let path = path.as_ref();
    let content = fs::read_to_string(path)
        .with_context(|| format!("读取配置文件失败: {}", path.display()))?;

    let mut settings: Settings = serde_yaml::from_str(&content)
        .with_context(|| format!("解析 YAML 失败: {}", path.display()))?;

    if let Ok(access_key) = env::var("VOLCENGINE_ACCESS_KEY") {
        settings.access_key = access_key;
    }
    if let Ok(secret_key) = env::var("VOLCENGINE_SECRET_KEY") {
        settings.secret_key = secret_key;
    }
    if let Ok(region) = env::var("VOLCENGINE_REGION") {
        settings.region = region;
    }
    if let Ok(endpoint) = env::var("VOLCENGINE_ENDPOINT") {
        settings.endpoint = endpoint;
    }

    validate_settings(&settings)?;
    Ok(settings)
}

fn validate_settings(settings: &Settings) -> Result<()> {
    if settings.access_key.trim().is_empty() {
        bail!("配置缺少 access key");
    }
    if settings.secret_key.trim().is_empty() {
        bail!("配置缺少 secret key");
    }
    if settings.region.trim().is_empty() {
        bail!("配置缺少 region");
    }
    if settings.endpoint.trim().is_empty() {
        bail!("配置缺少 endpoint");
    }
    Ok(())
}
