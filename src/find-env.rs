use aws_config::{meta::region::RegionProviderChain, SdkConfig};
use aws_sdk_cloudformation::{
    client::fluent_builders::ListExports, model::Export, output::ListExportsOutput, Client,
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "stage", default_value = "dev")]
    stage: String,

    #[structopt(long = "name")]
    name: String,
}

#[tokio::main]
async fn main() {
    let args: Cli = Cli::from_args();

    let region_provider: RegionProviderChain =
        RegionProviderChain::default_provider().or_else("sa-east-1");
    let config: SdkConfig = aws_config::from_env().region(region_provider).load().await;
    let client: Client = Client::new(&config);

    let list_exports: ListExports = client.list_exports();
    let send: ListExportsOutput = list_exports.send().await.unwrap();
    let exports: &[Export] = send.exports().unwrap();

    let arg_to_find: String = format!("{}-{}", args.stage, args.name);

    let export: &Export = exports
        .into_iter()
        .find(|&x| x.name().unwrap() == arg_to_find)
        .unwrap();

    println!("{:?}", export.value().unwrap());
}
