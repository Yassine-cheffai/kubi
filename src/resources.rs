// use futures::{StreamExt, TryStreamExt};
use k8s_openapi::api::core::v1::Pod as KPod;
use kube::{
    api::{Api, ListParams, PostParams, ResourceExt},
    Client,
};
use std::collections::HashMap;

#[tokio::main]
pub async fn get_pods() -> Result<Vec<HashMap<String, String>>, Box<dyn std::error::Error>> {
    // Infer the runtime environment and try to create a Kubernetes Client
    let client = Client::try_default().await?;
    let mut pods_list: Vec<HashMap<String, String>> = vec![];

    // Read pods in the configured namespace into the typed interface from k8s-openapi
    let pods: Api<KPod> = Api::default_namespaced(client);
    for p in pods.list(&ListParams::default()).await? {
        println!("found pod {}", p.name_any());
        let mut pod_data: HashMap<String, String> = HashMap::new();
        pod_data.insert("name".to_string(), p.name_any());
        let pod_status = p.status.unwrap();
        let conditions = pod_status.conditions.unwrap();
        println!("pod conditions ->");
        println!("{:?}", conditions);
        pod_data.insert("ip".to_string(), pod_status.pod_ip.unwrap());
        pod_data.insert("status".to_string(), pod_status.phase.unwrap());
        pod_data.insert(
            "nominated_node".to_string(),
            pod_status
                .nominated_node_name
                .unwrap_or("<none>".to_string()),
        );
        let start_time = pod_status.start_time.unwrap();
        pod_data.insert(
            "start_time".to_string(),
            start_time.0.timestamp().to_string(),
        );

        pods_list.push(pod_data);
    }
    Ok(pods_list)
}
