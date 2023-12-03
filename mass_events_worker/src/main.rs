mod configuration;

use std::time::Duration;

use clap::Parser;
use configuration::Configuration;
use mass_events_process_runner_client::{ProcessRunnerClient, ProcessRunnerClientError};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), ProcessRunnerClientError> {
    dotenvy::dotenv().ok();
    let configuration = Configuration::parse();
    let client = ProcessRunnerClient::new(&configuration.mass_events_process_runner_base_url);

    loop {
        let value = client.post_get_process(&"test").await?;
        if let Some(object) = value.as_object() {
            if object.contains_key("command"){
                println!("Recived command at: {}", chrono::offset::Local::now());
                println!("{:#?}", &value);
            } else {
                sleep(Duration::from_secs(7)).await;
            }
        }
        
    }
}
