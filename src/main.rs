use reqwest::header;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

fn get_client() -> reqwest::Client {
    let mut headers = header::HeaderMap::new();

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap();

    client
}

#[tokio::main]
async fn main() {
    let txids_url = "https://mempool.space/testnet/api/mempool/txids";

    let client = get_client();
    let txids_response = client.get(txids_url).send().await.unwrap();

    if txids_response.status().is_success() {
        let txids = txids_response.json::<Vec<String>>().await.unwrap();

        for i in 0..5 {
            let txid = &txids[i];
            let tx_url = format!("https://mempool.space/testnet/api/tx/{}", txid);
            
            let tx_response = client.get(&tx_url).send().await.unwrap();
            if tx_response.status().is_success() {
                let tx_json = tx_response.json::<serde_json::Value>().await.unwrap();
                
                let vin = tx_json["vin"].as_array().unwrap();
                for input in vin {
                    // Access input information as needed
                    println!("{:?}", input);
                }
            } else {
                println!("Request failed with status code: {}", tx_response.status());
            }
        }
    } else {
        println!("Request failed with status code: {}", txids_response.status());
    }
}