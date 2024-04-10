use crate::consts::RPC_URL_VERSION;
use anyhow::Context;
use std::env;
use url::Url;

pub fn node_url() -> anyhow::Result<Url> {
    let node_url = env::var("NODE_URL")
        .context("The required environmental variable `NODE_URL` is not set. Please set it manually or in .cargo/config.toml file"
    )?;

    Url::parse(&node_url).with_context(|| {
        format!("Failed to parse the URL from the `NODE_URL` environmental variable: {node_url}")
    })
}

pub fn node_url_with_version() -> anyhow::Result<Url> {
    let mut node_url = node_url()?;
    node_url.set_path(format!("rpc/{RPC_URL_VERSION}").as_str());
    node_url.set_query(None);

    Ok(node_url)
}
