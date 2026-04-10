use crate::{http::{ApiError, VolcApiClient}, vke::model::{CreateClusterRequest, CreateClusterResult, CreateKubeconfigRequest, CreateKubeconfigResult, GrantPermissionRequest, GrantPermissionResult, OpenApiEnvelope}};

#[derive(Debug, Clone)]
pub struct VkeApi {
    client: VolcApiClient,
}

impl VkeApi {
    pub fn new(client: VolcApiClient) -> Self {
        Self { client }
    }

    pub async fn create_cluster(&self, request: &CreateClusterRequest) -> Result<CreateClusterResult, ApiError> {
        let response: OpenApiEnvelope<CreateClusterResult> = self.client.post_action("CreateCluster", request).await?;
        Ok(response.result)
    }

    pub async fn create_kubeconfig(
        &self,
        request: &CreateKubeconfigRequest,
    ) -> Result<CreateKubeconfigResult, ApiError> {
        let response: OpenApiEnvelope<CreateKubeconfigResult> =
            self.client.post_action("CreateKubeconfig", request).await?;
        Ok(response.result)
    }

    pub async fn grant_permission(
        &self,
        request: &GrantPermissionRequest,
    ) -> Result<GrantPermissionResult, ApiError> {
        let response: OpenApiEnvelope<GrantPermissionResult> =
            self.client.post_action("GrantPermission", request).await?;
        Ok(response.result)
    }
}
