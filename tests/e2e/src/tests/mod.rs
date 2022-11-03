use std::{
    thread,
    time::{Duration, Instant},
};

use avalanche_network_runner_sdk::{Client, GlobalConfig, StartRequest};
use avalanche_types::client::info as avalanche_sdk_info;

#[tokio::test]
async fn e2e() {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .is_test(true)
        .try_init();

    let (ep, is_set) = crate::get_network_runner_grpc_endpoint();
    assert!(is_set);

    let cli = Client::new(&ep).await;

    log::info!("ping...");
    let resp = cli.ping().await.expect("failed ping");
    log::info!("network-runner is running (ping response {:?})", resp);

    let (exec_path, is_set) = crate::get_network_runner_avalanchego_path();
    assert!(is_set);

    log::info!("starting...");
    let resp = cli
        .start(StartRequest {
            exec_path,
            global_node_config: Some(
                serde_json::to_string(&GlobalConfig {
                    log_level: String::from("info"),
                })
                .unwrap(),
            ),
            // TODO: set "blockchain_specs"
            ..Default::default()
        })
        .await
        .expect("failed start");
    log::info!(
        "started avalanchego cluster with network-runner: {:?}",
        resp
    );

    // enough time for network-runner to get ready
    thread::sleep(Duration::from_secs(20));

    log::info!("checking cluster healthiness...");
    let mut ready = false;
    let timeout = Duration::from_secs(300);
    let interval = Duration::from_secs(15);
    let start = Instant::now();
    let mut cnt: u128 = 0;
    loop {
        let elapsed = start.elapsed();
        if elapsed.gt(&timeout) {
            break;
        }

        let itv = {
            if cnt == 0 {
                // first poll with no wait
                Duration::from_secs(1)
            } else {
                interval
            }
        };
        thread::sleep(itv);

        ready = {
            match cli.health().await {
                Ok(_) => {
                    log::info!("healthy now!");
                    true
                }
                Err(e) => {
                    log::warn!("not healthy yet {}", e);
                    false
                }
            }
        };
        if ready {
            break;
        }

        cnt += 1;
    }
    assert!(ready);

    log::info!("checking status...");
    let status = cli.status().await.expect("failed status");
    assert!(status.cluster_info.is_some());
    let cluster_info = status.cluster_info.unwrap();
    let mut rpc_eps: Vec<String> = Vec::new();
    for (node_name, iv) in cluster_info.node_infos.into_iter() {
        log::info!("{}: {}", node_name, iv.uri);
        rpc_eps.push(iv.uri.clone());
    }
    log::info!("avalanchego RPC endpoints: {:?}", rpc_eps);

    let resp = avalanche_sdk_info::get_network_id(&rpc_eps[0])
        .await
        .unwrap();
    let network_id = resp.result.unwrap().network_id;
    log::info!("network Id: {}", network_id);

    // enough time for txs processing
    thread::sleep(Duration::from_secs(15));

    // TODO: run tests

    log::info!("stopping...");
    let _resp = cli.stop().await.expect("failed stop");
    log::info!("successfully stopped network");
}
