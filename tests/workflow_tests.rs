use httpmock::prelude::*;
use serde_json::json;
use vke_manager::{
    config::Settings,
    http::VolcApiClient,
    vke::{VkeApi, VkeWorkflow},
};

fn sample_settings(endpoint: String) -> Settings {
    Settings {
        access_key: String::from("ak-test"),
        secret_key: String::from("sk-test"),
        region: String::from("cn-beijing"),
        endpoint,
        cluster: vke_manager::config::ClusterCreationConfig {
            name: String::from("demo-cluster"),
            vpc_id: String::from("vpc-123"),
            subnet_ids: vec![String::from("subnet-1")],
            kubernetes_version: String::from("v1.28"),
            pod_cidr: String::from("172.16.0.0/16"),
            service_cidr: String::from("192.168.0.0/16"),
            api_server_public_access_enabled: true,
            api_server_subnet_ids: vec![String::from("subnet-1")],
            cluster_type: String::from("ManagedKubernetes"),
            description: String::from("test cluster"),
        },
        permission: vke_manager::config::PermissionConfig {
            grantee_id: String::from("user-1"),
            grantee_type: String::from("User"),
            role_name: String::from("cluster-admin"),
            namespace: String::new(),
        },
    }
}

#[tokio::test]
async fn workflow_runs_three_steps_in_order() {
    let server = MockServer::start();
    let create_cluster = server.mock(|when, then| {
        when.method(POST)
            .path("/")
            .query_param("Action", "CreateCluster");
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({"Result": {"ClusterId": "cc-123"}}));
    });
    let create_kubeconfig = server.mock(|when, then| {
        when.method(POST)
            .path("/")
            .query_param("Action", "CreateKubeconfig");
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({"Result": {"KubeconfigId": "kc-123", "KubeConfig": "apiVersion: v1"}}));
    });
    let grant_permission = server.mock(|when, then| {
        when.method(POST)
            .path("/")
            .query_param("Action", "GrantPermission");
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({"Result": {"PermissionId": "perm-123"}}));
    });

    let settings = sample_settings(server.base_url());
    let client = VolcApiClient::new(&settings, "vke", "2022-05-12");
    let workflow = VkeWorkflow::new(VkeApi::new(client));

    let result = workflow
        .run(&settings, "Public", Some(3600))
        .await
        .expect("工作流应执行成功");

    create_cluster.assert();
    create_kubeconfig.assert();
    grant_permission.assert();
    assert_eq!(result.cluster_id, "cc-123");
    assert_eq!(result.kubeconfig_id, "kc-123");
    assert_eq!(result.permission_id, "perm-123");
    assert_eq!(result.kubeconfig, "apiVersion: v1");
}
