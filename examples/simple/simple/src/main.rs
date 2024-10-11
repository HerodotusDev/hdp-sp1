use hdp_sdk::DataProcessorClient;

fn main() {
    let client = DataProcessorClient::new();
    let (proof, vk) = client.prove("./program".into()).unwrap();
    client.verify(&proof, &vk).expect("failed to verify proof");
}
