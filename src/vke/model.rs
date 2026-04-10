use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateClusterRequest {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ClusterType", skip_serializing_if = "Option::is_none")]
    pub cluster_type: Option<String>,
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "VpcId")]
    pub vpc_id: String,
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,
    #[serde(rename = "KubernetesVersion")]
    pub kubernetes_version: String,
    #[serde(rename = "PodCidr")]
    pub pod_cidr: String,
    #[serde(rename = "ServiceCidr")]
    pub service_cidr: String,
    #[serde(rename = "ApiServerPublicAccessEnabled")]
    pub api_server_public_access_enabled: bool,
    #[serde(rename = "ApiServerSubnetIds", skip_serializing_if = "Vec::is_empty")]
    pub api_server_subnet_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateClusterResult {
    #[serde(rename = "ClusterId")]
    pub cluster_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateKubeconfigRequest {
    #[serde(rename = "ClusterId")]
    pub cluster_id: String,
    #[serde(rename = "Type")]
    pub kind: String,
    #[serde(rename = "ValidDuration", skip_serializing_if = "Option::is_none")]
    pub valid_duration: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateKubeconfigResult {
    #[serde(rename = "KubeconfigId")]
    pub kubeconfig_id: String,
    #[serde(rename = "KubeConfig", default)]
    pub kubeconfig: String,
    #[serde(rename = "Kubeconfig", default)]
    pub kubeconfig_compat: String,
}

impl CreateKubeconfigResult {
    pub fn content(&self) -> &str {
        if self.kubeconfig.is_empty() {
            &self.kubeconfig_compat
        } else {
            &self.kubeconfig
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GrantPermissionRequest {
    #[serde(rename = "ClusterId")]
    pub cluster_id: String,
    #[serde(rename = "GranteeId")]
    pub grantee_id: String,
    #[serde(rename = "GranteeType")]
    pub grantee_type: String,
    #[serde(rename = "RoleName")]
    pub role_name: String,
    #[serde(rename = "Namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GrantPermissionResult {
    #[serde(rename = "PermissionId")]
    pub permission_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OpenApiEnvelope<T> {
    #[serde(rename = "Result")]
    pub result: T,
}
