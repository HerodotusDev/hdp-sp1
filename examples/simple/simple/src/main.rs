use hdp_sdk::DataProcessorClient;

fn main() {
    let client = DataProcessorClient::new();
    client.execute("./program".into()).unwrap();
}
