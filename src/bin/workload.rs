use tikv_client::RawClient;

#[tokio::main]
async fn main() {
    let client = RawClient::new(vec!["192.168.122.250:2379"]).await.unwrap();
    let mut tasks = vec![];
    for _ in 0..8 {
        let client = client.clone();
        tasks.push(tokio::spawn(async move {
            for _ in 0..10_000 {
                let k: [u8; 2] = rand::random();
                let v: [u8; 32] = rand::random();
                client.put(k.to_vec(), v).await.unwrap();
                let k: [u8; 2] = rand::random();
                client.get(k.to_vec()).await.unwrap();
            }
        }))
    }
    for t in tasks {
        t.await.unwrap();
    }
}
