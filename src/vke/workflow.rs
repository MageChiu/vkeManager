use anyhow::{Result, bail};

use crate::{
    config::model::Settings,
    vke::{
        api::VkeApi,
        model::{CreateClusterRequest, CreateKubeconfigRequest, GrantPermissionRequest},
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClusterWorkflowOutput {
    pub cluster_id: String,
    pub kubeconfig_id: String,
    pub kubeconfig: String,
    pub permission_id: String,
}

#[derive(Debug, Clone)]
pub struct VkeWorkflow {
    api: VkeApi,
}

impl VkeWorkflow {
    pub fn new(api: VkeApi) -> Self {
        Self { api }
    }

    pub async fn run(
        &self,
        settings: &Settings,
        kubeconfig_type: &str,
        kubeconfig_valid_duration: Option<i32>,
    ) -> Result<ClusterWorkflowOutput> {
        validate_cluster_settings(settings)?;
        validate_permission_settings(settings)?;

        let cluster = self
            .api
            .create_cluster(&CreateClusterRequest {
                name: settings.cluster.name.clone(),
                cluster_type: Some(settings.cluster.cluster_type.clone()),
                description: optional_string(&settings.cluster.description),
                vpc_id: settings.cluster.vpc_id.clone(),
                subnet_ids: settings.cluster.subnet_ids.clone(),
                kubernetes_version: settings.cluster.kubernetes_version.clone(),
                pod_cidr: settings.cluster.pod_cidr.clone(),
                service_cidr: settings.cluster.service_cidr.clone(),
                api_server_public_access_enabled: settings.cluster.api_server_public_access_enabled,
                api_server_subnet_ids: settings.cluster.api_server_subnet_ids.clone(),
            })
            .await?;

        let kubeconfig = self
            .api
            .create_kubeconfig(&CreateKubeconfigRequest {
                cluster_id: cluster.cluster_id.clone(),
                kind: kubeconfig_type.to_owned(),
                valid_duration: kubeconfig_valid_duration,
            })
            .await?;

        let permission = self
            .api
            .grant_permission(&GrantPermissionRequest {
                cluster_id: cluster.cluster_id.clone(),
                grantee_id: settings.permission.grantee_id.clone(),
                grantee_type: settings.permission.grantee_type.clone(),
                role_name: settings.permission.role_name.clone(),
                namespace: optional_string(&settings.permission.namespace),
            })
            .await?;

        let kubeconfig_content = kubeconfig.content().to_owned();

        Ok(ClusterWorkflowOutput {
            cluster_id: cluster.cluster_id,
            kubeconfig_id: kubeconfig.kubeconfig_id,
            kubeconfig: kubeconfig_content,
            permission_id: permission.permission_id,
        })
    }
}

fn optional_string(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_owned())
    }
}

fn validate_cluster_settings(settings: &Settings) -> Result<()> {
    if settings.cluster.name.trim().is_empty() {
        bail!("必须提供集群名称");
    }
    if settings.cluster.vpc_id.trim().is_empty() {
        bail!("必须提供 vpc_id");
    }
    if settings.cluster.subnet_ids.is_empty() {
        bail!("必须至少提供一个 subnet_id");
    }
    if settings.cluster.kubernetes_version.trim().is_empty() {
        bail!("必须提供 kubernetes_version");
    }
    if settings.cluster.pod_cidr.trim().is_empty() {
        bail!("必须提供 pod_cidr");
    }
    if settings.cluster.service_cidr.trim().is_empty() {
        bail!("必须提供 service_cidr");
    }
    Ok(())
}

fn validate_permission_settings(settings: &Settings) -> Result<()> {
    if settings.permission.grantee_id.trim().is_empty() {
        bail!("必须提供 grantee_id");
    }
    Ok(())
}
