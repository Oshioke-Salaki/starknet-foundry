use crate::helpers::constants::{SEED, URL};
use crate::helpers::fixtures::{
    deploy_cairo_0_account, deploy_keystore_account, from_env, remove_devnet_env,
};
use ctor::{ctor, dtor};
use std::net::TcpStream;
use std::process::{Command, Stdio};
use std::string::ToString;
use std::time::{Duration, Instant};
use tokio::runtime::Runtime;
use url::Url;

#[cfg(test)]
#[ctor]
fn start_devnet() {
    fn verify_devnet_availability(address: &str) -> bool {
        TcpStream::connect(address).is_ok()
    }

    let port = Url::parse(URL).unwrap().port().unwrap_or(80).to_string();
    let host = Url::parse(URL)
        .unwrap()
        .host()
        .expect("Can't parse devnet URL!")
        .to_string();

    loop {
        if verify_devnet_availability(&format!("{host}:{port}")) {
            stop_devnet();
        } else {
            break;
        }
    }

    let fork_network_url =
        from_env("FORK_NETWORK_URL").expect("Failed to get FORK_NETWORK_URL environment variable");

    Command::new("tests/utils/devnet/bin/starknet-devnet")
        .args([
            "--port",
            &port,
            "--seed",
            &SEED.to_string(),
            "--state-archive-capacity",
            "full",
            "--fork-network",
            &fork_network_url,
        ])
        .stdout(Stdio::null())
        .spawn()
        .expect("Failed to start devnet!");

    let now = Instant::now();
    let timeout = Duration::from_secs(30);

    loop {
        if verify_devnet_availability(&format!("{host}:{port}")) {
            break;
        } else if now.elapsed() >= timeout {
            eprintln!("Timed out while waiting for devnet!");
            std::process::exit(1);
        }
    }

    let rt = Runtime::new().expect("Could not instantiate Runtime");

    rt.block_on(deploy_keystore_account());
    rt.block_on(deploy_cairo_0_account());
}

#[cfg(test)]
#[dtor]
fn stop_devnet() {
    let port = Url::parse(URL).unwrap().port().unwrap_or(80).to_string();
    Command::new("pkill")
        .args([
            "-f",
            &format!("starknet-devnet.*{}.*{}", &port, &SEED.to_string())[..],
        ])
        .spawn()
        .expect("Failed to kill devnet processes");
    remove_devnet_env();
}
