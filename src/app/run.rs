use anyhow::Result;

use crate::{
    cli::Cli,
    config::{Settings, load_settings},
    http::VolcApiClient,
    vke::{VkeApi, VkeWorkflow},
};

pub async fn run(cli: Cli) -> Result<()> {
    let mut settings = load_settings(&cli.config)?;
    apply_cli_overrides(&mut settings, &cli);

    let kubeconfig_type = cli
        .kubeconfig_type
        .clone()
        .unwrap_or_else(|| String::from("Public"));

    if cli.dry_run {
        println!("{:#?}", settings);
        return Ok(());
    }

    let client = VolcApiClient::new(&settings, "vke", "2022-05-12");
    let api = VkeApi::new(client);
    let workflow = VkeWorkflow::new(api);
    let result = workflow
        .run(&settings, &kubeconfig_type, cli.kubeconfig_valid_duration_seconds)
        .await?;

    println!("集群创建成功: {}", result.cluster_id);
    println!("Kubeconfig ID: {}", result.kubeconfig_id);
    if !result.kubeconfig.is_empty() {
        println!("Kubeconfig 内容:\n{}", result.kubeconfig);
    }
    println!("权限 ID: {}", result.permission_id);
    Ok(())
}

fn apply_cli_overrides(settings: &mut Settings, cli: &Cli) {
    if let Some(region) = &cli.region {
        settings.region = region.clone();
    }
    if let Some(endpoint) = &cli.endpoint {
        settings.endpoint = endpoint.clone();
    }
    if let Some(name) = &cli.name {
        settings.cluster.name = name.clone();
    }
    if let Some(vpc_id) = &cli.vpc_id {
        settings.cluster.vpc_id = vpc_id.clone();
    }
    if !cli.subnet_ids.is_empty() {
        settings.cluster.subnet_ids = cli.subnet_ids.clone();
    }
    if let Some(version) = &cli.kubernetes_version {
        settings.cluster.kubernetes_version = version.clone();
    }
    if let Some(pod_cidr) = &cli.pod_cidr {
        settings.cluster.pod_cidr = pod_cidr.clone();
    }
    if let Some(service_cidr) = &cli.service_cidr {
        settings.cluster.service_cidr = service_cidr.clone();
    }
    if let Some(enabled) = cli.api_server_public_access_enabled {
        settings.cluster.api_server_public_access_enabled = enabled;
    }
    if !cli.api_server_subnet_ids.is_empty() {
        settings.cluster.api_server_subnet_ids = cli.api_server_subnet_ids.clone();
    }
    if let Some(cluster_type) = &cli.cluster_type {
        settings.cluster.cluster_type = cluster_type.clone();
    }
    if let Some(description) = &cli.description {
        settings.cluster.description = description.clone();
    }
    if let Some(grantee_id) = &cli.grantee_id {
        settings.permission.grantee_id = grantee_id.clone();
    }
    if let Some(grantee_type) = &cli.grantee_type {
        settings.permission.grantee_type = grantee_type.clone();
    }
    if let Some(role_name) = &cli.role_name {
        settings.permission.role_name = role_name.clone();
    }
    if let Some(namespace) = &cli.namespace {
        settings.permission.namespace = namespace.clone();
    }
}
