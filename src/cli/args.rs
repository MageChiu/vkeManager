use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[command(author, version, about = "创建火山引擎 VKE 集群的命令行工具")]
pub struct Cli {
    #[arg(long, default_value = "configs/contextsearch_devqa.yaml")]
    pub config: PathBuf,
    #[arg(long)]
    pub name: Option<String>,
    #[arg(long)]
    pub vpc_id: Option<String>,
    #[arg(long, value_delimiter = ',')]
    pub subnet_ids: Vec<String>,
    #[arg(long)]
    pub kubernetes_version: Option<String>,
    #[arg(long)]
    pub pod_cidr: Option<String>,
    #[arg(long)]
    pub service_cidr: Option<String>,
    #[arg(long)]
    pub api_server_public_access_enabled: Option<bool>,
    #[arg(long, value_delimiter = ',')]
    pub api_server_subnet_ids: Vec<String>,
    #[arg(long)]
    pub cluster_type: Option<String>,
    #[arg(long)]
    pub description: Option<String>,
    #[arg(long)]
    pub kubeconfig_type: Option<String>,
    #[arg(long)]
    pub kubeconfig_valid_duration_seconds: Option<i32>,
    #[arg(long)]
    pub grantee_id: Option<String>,
    #[arg(long)]
    pub grantee_type: Option<String>,
    #[arg(long)]
    pub role_name: Option<String>,
    #[arg(long)]
    pub namespace: Option<String>,
    #[arg(long)]
    pub region: Option<String>,
    #[arg(long)]
    pub endpoint: Option<String>,
    #[arg(long, default_value_t = false)]
    pub dry_run: bool,
}
