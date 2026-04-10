use std::fs;

use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn cli_dry_run_uses_config_and_prints_settings() {
    let temp_dir = std::env::temp_dir().join("vke_manager_cli_tests");
    fs::create_dir_all(&temp_dir).expect("应能创建临时目录");
    let config_path = temp_dir.join("cli-config.yaml");
    fs::write(
        &config_path,
        r#"AK: test-ak
SK: test-sk
cluster:
  name: cli-cluster
  vpc_id: vpc-1
  subnet_ids: [subnet-1]
  kubernetes_version: v1.28
  pod_cidr: 172.16.0.0/16
  service_cidr: 192.168.0.0/16
permission:
  grantee_id: user-1
"#,
    )
    .expect("应能写入测试配置");

    let mut command = Command::cargo_bin("vke-manager").expect("二进制应可构建");
    command
        .arg("--config")
        .arg(&config_path)
        .arg("--dry-run")
        .assert()
        .success()
        .stdout(contains("cli-cluster"))
        .stdout(contains("vpc-1"));
}
