use std::env;
use tikv_client::RawClient;

#[tokio::main]
async fn main() {
    let client = RawClient::new(vec!["127.0.0.1:2379"]).await.unwrap();
    // let client = RawClient::new(vec!["127.0.0.1:2379", "127.0.0.1:2382", "127.0.0.1:2384"])
    //     .await
    //     .unwrap();
    let args: Vec<String> = env::args().collect();
    let key = args[1].clone();
    let value = client.get(key).await.unwrap();
    println!("{:?}", value.map(|v| String::from_utf8(v).unwrap()));
}
