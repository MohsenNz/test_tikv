use tikv_client::{IntoOwnedRange, RawClient};

#[tokio::main]
async fn main() {
    let client = RawClient::new(vec!["192.168.122.250:2379"]).await.unwrap();
    let key = "ae".to_string();
    let value = "11111".to_string();
    client.put(key, value.clone()).await.unwrap();
    let key = "he".to_string();
    client.put(key, value.clone()).await.unwrap();
    let key = "ha".to_string();
    client.put(key, value.clone()).await.unwrap();
    let key = "haa".to_string();
    client.put(key, value.clone()).await.unwrap();

    let x = client.scan_keys(("he"..).into_owned(), 100).await.unwrap();
    println!("from: he -- {:?}", x);
}
