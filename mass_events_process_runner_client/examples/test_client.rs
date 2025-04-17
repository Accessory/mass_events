use mass_events_process_runner_client::ProcessRunnerClient;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ProcessRunnerClient::new("http://127.0.0.1:8456");

    client
        .post_add_process(
            "test",
            json!({
                "name": "John Doe",
                "age": 43,
                "phones": [
                    "+44 1234567",
                    "+44 2345678"
                ]
            }),
        )
        .await?;
    println!("{:#?}", ());

    let resp2 = client.post_get_process("test").await?;
    println!("{:#?}", resp2);

    let resp3 = client.get_list_queues().await?;
    println!("{:#?}", resp3);
    Ok(())
}
