use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Settings {
    #[serde(alias = "AK")]
    pub access_key: String,
    #[serde(alias = "SK")]
    pub secret_key: String,
    #[serde(default = "default_region", alias = "Region")]
    pub region: String,
    #[serde(default = "default_endpoint", alias = "Endpoint")]
    pub endpoint: String,
    #[serde(default)]
    pub cluster: ClusterCreationConfig,
    #[serde(default)]
    pub permission: PermissionConfig,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct ClusterCreationConfig {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub vpc_id: String,
    #[serde(default)]
    pub subnet_ids: Vec<String>,
    #[serde(default)]
    pub kubernetes_version: String,
    #[serde(default)]
    pub pod_cidr: String,
    #[serde(default)]
    pub service_cidr: String,
    #[serde(default)]
    pub api_server_public_access_enabled: bool,
    #[serde(default)]
    pub api_server_subnet_ids: Vec<String>,
    #[serde(default)]
    pub cluster_type: String,
    #[serde(default)]
    pub description: String,
}

impl Default for ClusterCreationConfig {
    fn default() -> Self {
        Self {
            name: String::new(),
            vpc_id: String::new(),
            subnet_ids: Vec::new(),
            kubernetes_version: String::new(),
            pod_cidr: String::new(),
            service_cidr: String::new(),
            api_server_public_access_enabled: false,
            api_server_subnet_ids: Vec::new(),
            cluster_type: String::from("ManagedKubernetes"),
            description: String::new(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Default)]
pub struct PermissionConfig {
    #[serde(default)]
    pub grantee_id: String,
    #[serde(default = "default_grantee_type")]
    pub grantee_type: String,
    #[serde(default = "default_role_name")]
    pub role_name: String,
    #[serde(default)]
    pub namespace: String,
}

fn default_region() -> String {
    String::from("cn-beijing")
}

fn default_endpoint() -> String {
    String::from("https://open.volcengineapi.com")
}

fn default_grantee_type() -> String {
    String::from("User")
}

fn default_role_name() -> String {
    String::from("cluster-admin")
}
