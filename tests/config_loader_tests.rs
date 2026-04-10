use std::fs;

use vke_manager::config::load_settings;

#[test]
fn load_settings_supports_existing_ak_sk_format() {
    let temp_dir = std::env::temp_dir().join("vke_manager_tests");
    fs::create_dir_all(&temp_dir).expect("应能创建临时目录");
    let file_path = temp_dir.join("config.yaml");
    fs::write(&file_path, "AK: demo-ak\nSK: demo-sk\n").expect("应能写入测试配置");

    let settings = load_settings(&file_path).expect("配置应能加载成功");

    assert_eq!(settings.access_key, "demo-ak");
    assert_eq!(settings.secret_key, "demo-sk");
    assert_eq!(settings.region, "cn-beijing");
    assert_eq!(settings.endpoint, "https://open.volcengineapi.com");
}
