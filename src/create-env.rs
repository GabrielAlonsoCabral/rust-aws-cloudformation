use std::fs::File;
use std::io::prelude::*;

use aws_config::{meta::region::RegionProviderChain, SdkConfig};
use aws_sdk_cloudformation::{
    client::fluent_builders::ListExports, model::Export, output::ListExportsOutput, Client,
};

#[tokio::main]
async fn main() {
    let region_provider: RegionProviderChain =
        RegionProviderChain::default_provider().or_else("sa-east-1");
    let config: SdkConfig = aws_config::from_env().region(region_provider).load().await;
    let client: Client = Client::new(&config);

    let list_exports: ListExports = client.list_exports();
    let send: ListExportsOutput = list_exports.send().await.unwrap();
    let exports: &[Export] = send.exports().unwrap();

    let mut file: File = File::create(".env").unwrap();

    for export in exports {
        let text: String = format!("{}={}\n", export.name().unwrap(), export.value().unwrap());

        file.write_all(text.as_bytes()).unwrap();
    }
}
